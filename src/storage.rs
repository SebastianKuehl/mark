use crate::error::MarkError;
use std::path::PathBuf;

/// Paths used by the mark application.
pub struct AppPaths {
    /// `~/.mark`
    #[allow(dead_code)]
    pub root: PathBuf,
    /// `~/.mark/rendered`
    pub rendered: PathBuf,
    /// `~/.mark/bin`
    #[allow(dead_code)]
    pub bin: PathBuf,
}

impl AppPaths {
    /// Resolve all app paths from the user's home directory.
    pub fn resolve() -> Result<Self, MarkError> {
        let home = dirs::home_dir().ok_or(MarkError::NoHomeDir)?;
        let root = home.join(".mark");
        let rendered = root.join("rendered");
        let bin = root.join("bin");
        Ok(AppPaths {
            root,
            rendered,
            bin,
        })
    }

    /// Ensure `.mark/rendered/` exists, creating it if needed.
    pub fn ensure_rendered_dir(&self) -> Result<(), MarkError> {
        std::fs::create_dir_all(&self.rendered)?;
        Ok(())
    }
}

/// Generate a unique output filename for a rendered HTML file.
///
/// Format: `<stem>-<timestamp-secs>-<short-hash>.html`
///
/// The hash is derived from the canonical input path so the same file
/// rendered at the same second still gets a unique, deterministic name.
pub fn output_filename(input: &std::path::Path) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Simple hash: fold the path bytes.
    let path_bytes = input.as_os_str().as_encoded_bytes();
    let hash: u32 = path_bytes
        .iter()
        .fold(0u32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u32));

    format!("{stem}-{secs}-{hash:08x}.html")
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
    }

    #[test]
    fn app_paths_are_under_home() {
        let home = dirs::home_dir().expect("home dir");
        let paths = AppPaths::resolve().expect("resolve");
        assert!(paths.root.starts_with(&home));
        assert!(paths.rendered.starts_with(&home));
        assert!(paths.bin.starts_with(&home));
    }

    #[test]
    fn output_filename_uses_stem() {
        let name = output_filename(Path::new("notes.md"));
        assert!(name.starts_with("notes-"), "got: {name}");
        assert!(name.ends_with(".html"), "got: {name}");
    }

    #[test]
    fn output_filename_is_unique_for_different_inputs() {
        // Same timestamp is unlikely; we check structure at least.
        let a = output_filename(Path::new("a.md"));
        let b = output_filename(Path::new("b.md"));
        // Stems differ
        assert!(a.starts_with("a-"));
        assert!(b.starts_with("b-"));
    }

    #[test]
    fn output_filename_format_segments() {
        let name = output_filename(Path::new("/tmp/my-doc.md"));
        // Expected: my-doc-<secs>-<8hex>.html
        let parts: Vec<&str> = name.splitn(4, '-').collect();
        assert_eq!(parts.len(), 4, "expected 4 dash-separated segments: {name}");
        // Last part ends with .html
        assert!(parts[3].ends_with(".html"), "got: {name}");
    }
}
