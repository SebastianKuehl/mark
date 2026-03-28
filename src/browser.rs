use std::path::Path;

/// Open `path` in the system default browser.
///
/// Fails with a descriptive error if the OS cannot launch the browser.
pub fn open_browser(path: &Path) -> anyhow::Result<()> {
    open::that(path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to open '{}' in the default browser: {e}",
            path.display()
        )
    })
}
