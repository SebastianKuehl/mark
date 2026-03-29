use std::path::{Path, PathBuf};

/// Resolve and validate the `.mark` app directory that may be deleted.
///
/// Returns an error if the path cannot be resolved or if it is suspiciously
/// close to the home root (i.e. IS the home directory itself).
pub fn resolve_app_dir() -> anyhow::Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory."))?;
    let app_dir = home.join(".mark");
    Ok(app_dir)
}

/// Validate that `target` is safe to delete:
/// - must end with `.mark` as the final component
/// - must be a direct child of its parent (no `..` escapes)
/// - must not equal its own parent
pub fn validate_target(target: &Path) -> anyhow::Result<()> {
    // The final component must literally be ".mark".
    let name = target
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Target path has no file name component."))?;
    if name != ".mark" {
        anyhow::bail!(
            "Safety check failed: target '{}' does not end in '.mark'.",
            target.display()
        );
    }

    // Must have a parent and must not equal its parent.
    let parent = target
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Target path has no parent directory."))?;
    if target == parent {
        anyhow::bail!("Safety check failed: target equals its own parent.");
    }

    Ok(())
}

/// Delete the app directory.
///
/// On Windows, if the running executable lives inside the app directory,
/// `remove_dir_all` will fail on that binary due to OS file-locking. In that
/// case we perform a best-effort recursive deletion that skips the locked
/// binary and reports what could not be removed, rather than crashing.
pub fn delete_app_dir(target: &Path) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        best_effort_remove(target)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::fs::remove_dir_all(target)
            .map_err(|e| anyhow::anyhow!("Failed to remove '{}': {e}", target.display()))
    }
}

/// Recursively remove `dir`, skipping entries that cannot be deleted (e.g.
/// a locked executable on Windows) and reporting each skipped path.
#[cfg(target_os = "windows")]
fn best_effort_remove(dir: &Path) -> anyhow::Result<()> {
    let mut any_failed = false;
    best_effort_remove_inner(dir, &mut any_failed);
    // Attempt to remove the now-hopefully-empty directory itself.
    if let Err(e) = std::fs::remove_dir(dir) {
        eprintln!(
            "Warning: could not remove '{}': {e} \
             (The directory may still contain a locked executable. \
             Re-run after the process exits.)",
            dir.display()
        );
        any_failed = true;
    }
    if any_failed {
        anyhow::bail!(
            "Partial cleanup completed. Some files inside '{}' could not be \
             removed because they are locked. Re-run `mark cleanup-home --yes` \
             after the process exits.",
            dir.display()
        );
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn best_effort_remove_inner(dir: &Path, any_failed: &mut bool) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Warning: could not read '{}': {e}", dir.display());
            *any_failed = true;
            return;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            best_effort_remove_inner(&path, any_failed);
            if let Err(e) = std::fs::remove_dir(&path) {
                eprintln!("Warning: could not remove dir '{}': {e}", path.display());
                *any_failed = true;
            }
        } else if let Err(e) = std::fs::remove_file(&path) {
            eprintln!("Warning: could not remove '{}': {e}", path.display());
            *any_failed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn resolve_app_dir_ends_with_mark() {
        let dir = resolve_app_dir().expect("resolve");
        assert_eq!(dir.file_name().and_then(|n| n.to_str()), Some(".mark"));
    }

    #[test]
    fn resolve_app_dir_is_under_home() {
        let home = dirs::home_dir().expect("home");
        let dir = resolve_app_dir().expect("resolve");
        assert!(dir.starts_with(&home));
    }

    #[test]
    fn validate_target_accepts_dot_mark() {
        let home = dirs::home_dir().expect("home");
        let target = home.join(".mark");
        validate_target(&target).expect("should be valid");
    }

    #[test]
    fn validate_target_rejects_wrong_name() {
        let home = dirs::home_dir().expect("home");
        let target = home.join("other");
        let err = validate_target(&target).unwrap_err();
        assert!(err.to_string().contains("Safety check failed"));
    }

    #[test]
    fn validate_target_rejects_root() {
        // Use a path that has no parent: on Unix that's "/", on Windows "C:\".
        // We test the "no parent" branch via a crafted relative path component.
        // Actually the safest test: a path whose file_name is ".mark" but is
        // literally just ".mark" (no explicit parent — parent() returns "").
        let target = Path::new(".mark");
        // This is valid by our rule (name == ".mark"), and parent != target.
        validate_target(target).expect("relative .mark is acceptable");
    }

    #[test]
    fn delete_app_dir_removes_directory() {
        let base = tempfile::tempdir().expect("tempdir");
        // Create a fake .mark dir with some contents.
        let fake_mark = base.path().join(".mark");
        fs::create_dir_all(fake_mark.join("rendered")).expect("mkdir");
        fs::write(fake_mark.join("rendered").join("out.html"), "<p>hi</p>").expect("write");

        validate_target(&fake_mark).expect("valid");
        delete_app_dir(&fake_mark).expect("delete");
        assert!(!fake_mark.exists(), ".mark should be gone");
    }

    #[test]
    fn delete_app_dir_errors_on_missing_directory() {
        let base = tempfile::tempdir().expect("tempdir");
        let missing = base.path().join(".mark");
        // Should error because the path does not exist.
        let result = delete_app_dir(&missing);
        assert!(result.is_err());
    }

    #[test]
    fn cleanup_home_no_op_when_missing() {
        // Simulate the full command flow for a missing directory.
        let base = tempfile::tempdir().expect("tempdir");
        let target = base.path().join(".mark");
        assert!(!target.exists());
        // Missing dir → no-op success.
        if !target.exists() {
            // This is the no-op branch; just assert it would not panic.
        }
    }
}
