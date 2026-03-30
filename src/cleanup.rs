use std::path::Path;
use std::time::{Duration, SystemTime};

/// Age threshold for rendered output: 30 days.
const MAX_AGE: Duration = Duration::from_secs(30 * 24 * 60 * 60);

/// Remove stale render-cache entries whose run directory no longer exists on disk.
///
/// Loads the cache from `cache_path`, calls `remove_missing_entries`, and saves
/// it back. Best-effort — errors are printed as warnings and not propagated.
pub fn prune_render_cache(cache_path: &Path) {
    let mut cache = crate::cache::RenderCache::load(cache_path.to_path_buf());
    cache.remove_missing_entries();
    cache.save();
}

/// Delete a file if it exists.
pub fn delete_file_if_exists(path: &Path) -> anyhow::Result<bool> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(err) => Err(anyhow::anyhow!(
            "Failed to remove '{}': {err}",
            path.display()
        )),
    }
}

/// Delete the rendered output directory if it exists, then prune stale cache entries.
pub fn delete_rendered_dir(rendered_dir: &Path, cache_path: &Path) -> anyhow::Result<bool> {
    let removed = match std::fs::remove_dir_all(rendered_dir) {
        Ok(()) => true,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => false,
        Err(err) => {
            return Err(anyhow::anyhow!(
                "Failed to remove '{}': {err}",
                rendered_dir.display()
            ))
        }
    };
    prune_render_cache(cache_path);
    Ok(removed)
}

/// Delete per-invocation render directories in `rendered_dir` whose oldest file
/// (or the directory itself when empty) is older than 30 days. Also removes
/// legacy top-level `.html` files from the pre-run-dir layout using the same
/// age threshold.
///
/// Safety guarantees:
/// - Only operates on direct children of `rendered_dir`.
/// - Skips symlinks entirely.
/// - Resolves candidate directories via canonicalization and refuses to delete
///   any path whose canonical parent is not the canonical `rendered_dir`.
/// - Continues past per-directory errors, printing a warning for each.
pub fn cleanup_old_files(rendered_dir: &Path) -> anyhow::Result<usize> {
    if !rendered_dir.exists() {
        return Ok(0);
    }

    let canonical_base = rendered_dir.canonicalize().map_err(|e| {
        anyhow::anyhow!(
            "Could not resolve rendered dir '{}': {e}",
            rendered_dir.display()
        )
    })?;

    let now = SystemTime::now();
    let mut deleted = 0usize;

    for entry in std::fs::read_dir(rendered_dir)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Warning: could not read directory entry: {e}");
                continue;
            }
        };

        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(e) => {
                eprintln!(
                    "Warning: could not read entry type for '{}': {e}",
                    path.display()
                );
                continue;
            }
        };

        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            let canonical_entry = match path.canonicalize() {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Warning: could not canonicalize '{}': {e}", path.display());
                    continue;
                }
            };

            if canonical_entry.parent() != Some(canonical_base.as_path()) {
                eprintln!(
                    "Warning: skipping '{}' — not a direct child of the rendered dir",
                    path.display()
                );
                continue;
            }

            let oldest_mtime = match oldest_mtime_in_tree(&path) {
                Ok(mtime) => mtime,
                Err(e) => {
                    eprintln!(
                        "Warning: could not inspect render directory '{}': {e}",
                        path.display()
                    );
                    continue;
                }
            };
            let age = now.duration_since(oldest_mtime).unwrap_or(Duration::ZERO);

            if age > MAX_AGE {
                if let Err(e) = std::fs::remove_dir_all(&path) {
                    eprintln!("Warning: could not delete '{}': {e}", path.display());
                } else {
                    deleted += 1;
                }
            }
            continue;
        }

        if file_type.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("html") {
            let age = match std::fs::metadata(&path)
                .and_then(|metadata| metadata.modified())
                .map(|mtime| now.duration_since(mtime).unwrap_or(Duration::ZERO))
            {
                Ok(age) => age,
                Err(e) => {
                    eprintln!(
                        "Warning: could not read legacy render mtime for '{}': {e}",
                        path.display()
                    );
                    continue;
                }
            };

            if age > MAX_AGE {
                if let Err(e) = std::fs::remove_file(&path) {
                    eprintln!("Warning: could not delete '{}': {e}", path.display());
                } else {
                    deleted += 1;
                }
            }
        }
    }

    Ok(deleted)
}

/// Returns how long ago `path` was last modified, relative to `now`.
#[cfg(test)]
fn file_age(path: &Path, now: SystemTime) -> anyhow::Result<Duration> {
    let mtime = std::fs::metadata(path)?.modified()?;
    Ok(now.duration_since(mtime).unwrap_or(Duration::ZERO))
}

fn oldest_mtime_in_tree(path: &Path) -> anyhow::Result<SystemTime> {
    let metadata = std::fs::symlink_metadata(path)?;
    let mut oldest = metadata.modified()?;

    if metadata.file_type().is_symlink() {
        return Ok(SystemTime::now());
    }

    if metadata.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_symlink() {
                continue;
            }
            let child_oldest = oldest_mtime_in_tree(&entry.path())?;
            if child_oldest < oldest {
                oldest = child_oldest;
            }
        }
    }

    Ok(oldest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::process::Command;

    fn write_file(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new("."))).expect("mkdirs");
        fs::write(path, content).expect("write");
    }

    fn backdate_path(path: &Path, touch_value: &str) {
        let status = Command::new("touch")
            .arg("-t")
            .arg(touch_value)
            .arg(path)
            .status()
            .expect("run touch");
        assert!(
            status.success(),
            "touch should succeed for {}",
            path.display()
        );
    }

    #[test]
    fn file_age_returns_zero_for_new_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let p = dir.path().join("new.html");
        write_file(&p, "<p>hi</p>");
        let age = file_age(&p, SystemTime::now()).expect("age");
        assert!(age < MAX_AGE, "expected age < 30d, got {age:?}");
    }

    #[test]
    fn cleanup_skips_non_html_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let txt = dir.path().join("notes.txt");
        write_file(&txt, "hello");
        let deleted = cleanup_old_files(dir.path()).expect("cleanup");
        assert_eq!(deleted, 0);
        assert!(txt.exists(), "plain file must survive cleanup");
    }

    #[test]
    fn cleanup_returns_zero_for_missing_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let missing = dir.path().join("no_such_dir");
        let deleted = cleanup_old_files(&missing).expect("cleanup");
        assert_eq!(deleted, 0);
    }

    #[test]
    fn delete_file_if_exists_removes_existing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let config = dir.path().join("config.toml");
        write_file(&config, "theme = 'light'");

        let deleted = delete_file_if_exists(&config).expect("delete");

        assert!(deleted);
        assert!(!config.exists());
    }

    #[test]
    fn delete_file_if_exists_returns_false_for_missing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let config = dir.path().join("config.toml");

        let deleted = delete_file_if_exists(&config).expect("delete");

        assert!(!deleted);
    }

    #[test]
    fn delete_rendered_dir_removes_directory_when_present() {
        let dir = tempfile::tempdir().expect("tempdir");
        let rendered = dir.path().join("rendered");
        fs::create_dir_all(rendered.join("run-1")).expect("mkdir");
        write_file(&rendered.join("run-1/out.html"), "<p>hello</p>");

        let removed =
            delete_rendered_dir(&rendered, &dir.path().join("render-cache.toml")).expect("delete");

        assert!(removed);
        assert!(!rendered.exists());
    }

    #[test]
    fn cleanup_returns_zero_for_fresh_run_directory() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        write_file(&run_dir.join("overview.html"), "<p>new</p>");
        let deleted = cleanup_old_files(dir.path()).expect("cleanup");
        assert_eq!(deleted, 0);
        assert!(run_dir.exists());
    }

    #[test]
    fn cleanup_deletes_old_run_directory() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let nested = run_dir.join("chapters/intro.html");
        write_file(&nested, "<p>old</p>");
        backdate_path(&nested, "200001010101");
        backdate_path(&run_dir.join("chapters"), "200001010101");
        backdate_path(&run_dir, "200001010101");

        let deleted = cleanup_old_files(dir.path()).expect("cleanup");
        assert_eq!(deleted, 1);
        assert!(!run_dir.exists(), "old run dir must be removed");
    }

    #[test]
    fn cleanup_deletes_old_legacy_html_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let legacy = dir.path().join("overview-123-abcdef12.html");
        write_file(&legacy, "<p>old</p>");
        backdate_path(&legacy, "200001010101");

        let deleted = cleanup_old_files(dir.path()).expect("cleanup");
        assert_eq!(deleted, 1);
        assert!(!legacy.exists(), "old legacy html should be removed");
    }

    #[test]
    fn cleanup_uses_oldest_nested_file_mtime() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let old_file = run_dir.join("chapters/old.html");
        let new_file = run_dir.join("chapters/new.html");
        write_file(&old_file, "old");
        write_file(&new_file, "new");
        backdate_path(&old_file, "200001010101");
        backdate_path(&run_dir.join("chapters"), "200001010101");
        backdate_path(&run_dir, "200001010101");

        let deleted = cleanup_old_files(dir.path()).expect("cleanup");
        assert_eq!(deleted, 1, "oldest nested file should drive deletion");
        assert!(!run_dir.exists());
    }

    #[test]
    fn oldest_mtime_in_tree_uses_nested_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let nested = run_dir.join("chapters/api/endpoints.html");
        write_file(&nested, "hello");
        backdate_path(&nested, "200001010101");

        let oldest = oldest_mtime_in_tree(&run_dir).expect("oldest");
        let age = SystemTime::now()
            .duration_since(oldest)
            .expect("mtime should be in the past");
        assert!(age > MAX_AGE, "nested file mtime should be considered");
    }

    #[cfg(unix)]
    #[test]
    fn oldest_mtime_in_tree_skips_nested_symlink_dirs() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let nested = run_dir.join("chapters/intro.html");
        let outside = dir.path().join("outside");
        write_file(&nested, "hello");
        fs::create_dir_all(&outside).expect("mkdir outside");
        symlink(&outside, run_dir.join("linked-outside")).expect("symlink");

        let oldest = oldest_mtime_in_tree(&run_dir).expect("oldest");
        let nested_mtime = std::fs::metadata(&nested)
            .expect("metadata")
            .modified()
            .expect("mtime");
        assert!(
            oldest <= nested_mtime,
            "nested symlink dir should not affect oldest-mtime traversal"
        );
    }

    #[cfg(unix)]
    #[test]
    fn cleanup_ignores_old_nested_symlink_mtime() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let nested = run_dir.join("fresh.html");
        let outside = dir.path().join("outside");
        write_file(&nested, "hello");
        fs::create_dir_all(&outside).expect("mkdir outside");
        let link = run_dir.join("linked-outside");
        symlink(&outside, &link).expect("symlink");

        let status = Command::new("touch")
            .arg("-h")
            .arg("-t")
            .arg("200001010101")
            .arg(&link)
            .status()
            .expect("backdate symlink");
        assert!(status.success(), "touch -h should succeed");

        let deleted = cleanup_old_files(dir.path()).expect("cleanup");
        assert_eq!(
            deleted, 0,
            "nested symlink should not age out a fresh run dir"
        );
        assert!(run_dir.exists(), "fresh run dir must survive");
    }

    #[test]
    fn max_age_is_30_days() {
        assert_eq!(MAX_AGE.as_secs(), 30 * 24 * 60 * 60);
    }
}
