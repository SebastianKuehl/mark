use crate::error::MarkError;
use std::path::{Path, PathBuf};

fn output_name_parts(input: &Path) -> (String, u128, u32) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output")
        .to_string();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let path_bytes = input.as_os_str().as_encoded_bytes();
    let hash: u32 = path_bytes
        .iter()
        .fold(0u32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u32));

    (stem, timestamp, hash)
}

/// Paths used by the mark application.
pub struct AppPaths {
    /// `~/.mark`
    pub root: PathBuf,
    /// `~/.mark/rendered`
    pub rendered: PathBuf,
    /// `~/.mark/bin`
    #[allow(dead_code)]
    pub bin: PathBuf,
    /// `~/.mark/config.toml`
    pub config: PathBuf,
    /// `~/.mark/render-cache.toml`
    pub render_cache: PathBuf,
}

impl AppPaths {
    /// Resolve all app paths from the user's home directory.
    pub fn resolve() -> Result<Self, MarkError> {
        let home = dirs::home_dir().ok_or(MarkError::NoHomeDir)?;
        let root = home.join(".mark");
        let rendered = root.join("rendered");
        let bin = root.join("bin");
        let config = root.join("config.toml");
        let render_cache = root.join("render-cache.toml");
        Ok(AppPaths {
            root,
            rendered,
            bin,
            config,
            render_cache,
        })
    }

    /// Ensure `.mark/rendered/` exists, creating it if needed.
    pub fn ensure_rendered_dir(&self) -> Result<(), MarkError> {
        std::fs::create_dir_all(&self.rendered)?;
        Ok(())
    }
}

/// Write `html` into `rendered_dir` using `filename`, returning the full path.
pub fn write_rendered(
    rendered_dir: &Path,
    filename: &str,
    html: &str,
) -> Result<PathBuf, MarkError> {
    std::fs::create_dir_all(rendered_dir)?;
    let out = rendered_dir.join(filename);
    std::fs::write(&out, html.as_bytes())?;
    Ok(out)
}

/// Create and return a unique per-invocation run directory inside `rendered_dir`.
///
/// Format: `<stem>-<timestamp>-<short-hash>/`
pub fn make_run_dir(rendered_dir: &Path, entry_path: &Path) -> Result<PathBuf, MarkError> {
    use std::thread;
    use std::time::Duration;

    std::fs::create_dir_all(rendered_dir)?;

    let mut last_error: Option<std::io::Error> = None;
    for _ in 0..10 {
        let (stem, timestamp, hash) = output_name_parts(entry_path);
        let run_dir = rendered_dir.join(format!("{stem}-{timestamp}-{hash:08x}"));
        match std::fs::create_dir(&run_dir) {
            Ok(()) => return Ok(run_dir),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                last_error = Some(err);
                thread::sleep(Duration::from_millis(1));
            }
            Err(err) => return Err(err.into()),
        }
    }

    Err(last_error
        .unwrap_or_else(|| std::io::Error::other("could not allocate unique run directory"))
        .into())
}

/// Generate a unique output filename for a rendered HTML file.
///
/// Format: `<stem>-<timestamp>-<short-hash>.html`
///
/// The hash is derived from the canonical input path so the same file
/// rendered at the same second still gets a unique, deterministic name.
pub fn output_filename(input: &Path) -> String {
    let (stem, timestamp, hash) = output_name_parts(input);
    format!("{stem}-{timestamp}-{hash:08x}.html")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn app_paths_resolve_succeeds() {
        let paths = AppPaths::resolve().expect("should resolve home dir");
        assert!(paths.root.ends_with(".mark"));
        assert!(paths.rendered.ends_with("rendered"));
        assert!(paths.bin.ends_with("bin"));
        assert!(paths.config.ends_with("config.toml"));
        assert!(paths.render_cache.ends_with("render-cache.toml"));
    }

    #[test]
    fn app_paths_are_under_home() {
        let home = dirs::home_dir().expect("home dir");
        let paths = AppPaths::resolve().expect("resolve");
        assert!(paths.root.starts_with(&home));
        assert!(paths.rendered.starts_with(&home));
        assert!(paths.bin.starts_with(&home));
        assert!(paths.render_cache.starts_with(&home));
    }

    #[test]
    fn output_filename_uses_stem() {
        let name = output_filename(Path::new("notes.md"));
        assert!(name.starts_with("notes-"), "got: {name}");
        assert!(name.ends_with(".html"), "got: {name}");
    }

    #[test]
    fn output_filename_is_unique_for_different_inputs() {
        let a = output_filename(Path::new("a.md"));
        let b = output_filename(Path::new("b.md"));
        assert!(a.starts_with("a-"));
        assert!(b.starts_with("b-"));
    }

    #[test]
    fn output_filename_format_segments() {
        let name = output_filename(Path::new("/tmp/my-doc.md"));
        // Expected: my-doc-<secs>-<8hex>.html
        let parts: Vec<&str> = name.splitn(4, '-').collect();
        assert_eq!(parts.len(), 4, "expected 4 dash-separated segments: {name}");
        assert!(parts[3].ends_with(".html"), "got: {name}");
    }

    #[test]
    fn write_rendered_creates_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html = "<html><body>hello</body></html>";
        let out = write_rendered(dir.path(), "test.html", html).expect("write");
        assert!(out.exists());
        let content = std::fs::read_to_string(&out).expect("read");
        assert_eq!(content, html);
    }

    #[test]
    fn write_rendered_creates_missing_dir() {
        let base = tempfile::tempdir().expect("tempdir");
        let rendered_dir = base.path().join("rendered");
        // Directory does not exist yet.
        let out = write_rendered(&rendered_dir, "out.html", "<p>ok</p>").expect("write");
        assert!(out.exists());
    }

    #[test]
    fn make_run_dir_creates_directory() {
        let base = tempfile::tempdir().expect("tempdir");
        let run_dir = make_run_dir(base.path(), Path::new("notes.md")).expect("run dir");
        assert!(run_dir.exists(), "run dir should exist");
        assert!(run_dir.is_dir(), "run dir should be a directory");
        assert!(run_dir.starts_with(base.path()));
    }

    #[test]
    fn make_run_dir_uses_entry_stem() {
        let base = tempfile::tempdir().expect("tempdir");
        let run_dir = make_run_dir(base.path(), Path::new("docs/overview.md")).expect("run dir");
        let name = run_dir
            .file_name()
            .and_then(|s| s.to_str())
            .expect("dir name");
        assert!(name.starts_with("overview-"), "got: {name}");
        assert!(!name.ends_with(".html"), "got: {name}");
    }

    #[test]
    fn make_run_dir_creates_distinct_dirs_across_calls() {
        let base = tempfile::tempdir().expect("tempdir");
        let first = make_run_dir(base.path(), Path::new("docs/overview.md")).expect("first");
        let second = make_run_dir(base.path(), Path::new("docs/overview.md")).expect("second");
        assert_ne!(first, second, "each invocation should get its own run dir");
    }
}
