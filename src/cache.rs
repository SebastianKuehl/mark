//! Render cache — remembers the last rendered HTML output for each source file.
//!
//! The cache is stored at `~/.mark/render-cache.toml` in TOML format,
//! keyed by canonical source path string:
//!
//! ```toml
//! ["/abs/path/to/overview.md"]
//! rendered_html = "/abs/path/to/rendered/overview-ts-hash.html"
//! source_mtime_secs = 1711648523
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A single cache entry for one source file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CacheEntry {
    /// Absolute path to the rendered HTML file.
    pub rendered_html: PathBuf,
    /// Unix timestamp (seconds) of the source file's mtime at render time.
    pub source_mtime_secs: u64,
}

/// In-memory render cache backed by a TOML file on disk.
pub struct RenderCache {
    /// Map from canonical source path (as string) → cache entry.
    entries: HashMap<String, CacheEntry>,
    /// Path to the TOML file on disk.
    cache_path: PathBuf,
}

impl RenderCache {
    /// Load the cache from `cache_path`.
    ///
    /// Returns an empty cache if the file does not exist or cannot be parsed —
    /// never panics.
    pub fn load(cache_path: PathBuf) -> Self {
        let entries = if cache_path.exists() {
            match std::fs::read_to_string(&cache_path) {
                Ok(text) => {
                    toml::from_str::<HashMap<String, CacheEntry>>(&text).unwrap_or_default()
                }
                Err(_) => HashMap::new(),
            }
        } else {
            HashMap::new()
        };

        RenderCache {
            entries,
            cache_path,
        }
    }

    /// Persist the cache to disk.
    ///
    /// Best-effort: prints a warning on failure but does not propagate the error.
    pub fn save(&self) {
        match toml::to_string_pretty(&self.entries) {
            Ok(text) => {
                if let Some(parent) = self.cache_path.parent() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        eprintln!(
                            "Warning: could not create cache directory '{}': {e}",
                            parent.display()
                        );
                        return;
                    }
                }
                if let Err(e) = std::fs::write(&self.cache_path, text.as_bytes()) {
                    eprintln!(
                        "Warning: could not write render cache '{}': {e}",
                        self.cache_path.display()
                    );
                }
            }
            Err(e) => {
                eprintln!("Warning: could not serialize render cache: {e}");
            }
        }
    }

    /// Look up the cache entry for `source`.
    pub fn get(&self, source: &Path) -> Option<&CacheEntry> {
        self.entries.get(source.to_string_lossy().as_ref())
    }

    /// Insert or update the cache entry for `source`.
    pub fn set(&mut self, source: &Path, entry: CacheEntry) {
        self.entries
            .insert(source.to_string_lossy().into_owned(), entry);
    }

    /// Remove entries whose `rendered_html` file no longer exists on disk.
    ///
    /// Called by `--cleanup` to prune stale cache state.
    pub fn remove_missing_entries(&mut self) {
        self.entries.retain(|_, entry| entry.rendered_html.exists());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn make_cache(dir: &Path) -> RenderCache {
        RenderCache::load(dir.join("render-cache.toml"))
    }

    // ── load/save round-trip ──────────────────────────────────────────────────

    #[test]
    fn load_returns_empty_cache_for_missing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cache = make_cache(dir.path());
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn load_returns_empty_cache_on_parse_error() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("render-cache.toml");
        std::fs::write(&path, b"not valid toml [[[").expect("write");
        let cache = RenderCache::load(path);
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn save_and_reload_preserves_entries() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("render-cache.toml");
        let mut cache = RenderCache::load(path.clone());

        let source = Path::new("/home/user/notes.md");
        let entry = CacheEntry {
            rendered_html: PathBuf::from("/home/user/.mark/rendered/notes-123-abc.html"),
            source_mtime_secs: 1_700_000_000,
        };
        cache.set(source, entry.clone());
        cache.save();

        let reloaded = RenderCache::load(path);
        assert_eq!(reloaded.get(source), Some(&entry));
    }

    // ── get / set ─────────────────────────────────────────────────────────────

    #[test]
    fn get_returns_none_for_unknown_source() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cache = make_cache(dir.path());
        assert!(cache.get(Path::new("/no/such/file.md")).is_none());
    }

    #[test]
    fn set_then_get_returns_entry() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut cache = make_cache(dir.path());
        let source = Path::new("/project/doc.md");
        let entry = CacheEntry {
            rendered_html: PathBuf::from("/tmp/doc-1-00000001.html"),
            source_mtime_secs: 42,
        };
        cache.set(source, entry.clone());
        assert_eq!(cache.get(source), Some(&entry));
    }

    #[test]
    fn set_overwrites_existing_entry() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut cache = make_cache(dir.path());
        let source = Path::new("/project/doc.md");

        cache.set(
            source,
            CacheEntry {
                rendered_html: PathBuf::from("/old.html"),
                source_mtime_secs: 1,
            },
        );
        let newer = CacheEntry {
            rendered_html: PathBuf::from("/new.html"),
            source_mtime_secs: 2,
        };
        cache.set(source, newer.clone());
        assert_eq!(cache.get(source), Some(&newer));
    }

    // ── mtime-based cache hit / miss ──────────────────────────────────────────

    #[test]
    fn mtime_hit_when_secs_match() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut cache = make_cache(dir.path());
        let source = Path::new("/project/doc.md");
        let mtime: u64 = 1_711_648_523;
        cache.set(
            source,
            CacheEntry {
                rendered_html: PathBuf::from("/out.html"),
                source_mtime_secs: mtime,
            },
        );
        let entry = cache.get(source).expect("entry");
        assert_eq!(entry.source_mtime_secs, mtime);
    }

    #[test]
    fn mtime_miss_when_secs_differ() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut cache = make_cache(dir.path());
        let source = Path::new("/project/doc.md");
        cache.set(
            source,
            CacheEntry {
                rendered_html: PathBuf::from("/out.html"),
                source_mtime_secs: 100,
            },
        );
        let entry = cache.get(source).expect("entry");
        // Simulate: current mtime (200) != cached (100) → stale
        assert_ne!(entry.source_mtime_secs, 200u64);
    }

    // ── remove_missing_entries ────────────────────────────────────────────────

    #[test]
    fn remove_missing_entries_keeps_existing_html() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html = dir.path().join("out.html");
        std::fs::write(&html, b"<html/>").expect("write");

        let mut cache = make_cache(dir.path());
        let source = Path::new("/doc.md");
        cache.set(
            source,
            CacheEntry {
                rendered_html: html.clone(),
                source_mtime_secs: 1,
            },
        );
        cache.remove_missing_entries();
        assert!(
            cache.get(source).is_some(),
            "entry for existing html must survive"
        );
    }

    #[test]
    fn remove_missing_entries_drops_stale_entry() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut cache = make_cache(dir.path());
        let source = Path::new("/doc.md");
        cache.set(
            source,
            CacheEntry {
                // This HTML file does not exist.
                rendered_html: dir.path().join("ghost.html"),
                source_mtime_secs: 1,
            },
        );
        assert!(
            cache.get(source).is_some(),
            "entry should exist before pruning"
        );
        cache.remove_missing_entries();
        assert!(cache.get(source).is_none(), "stale entry must be pruned");
    }

    #[test]
    fn remove_missing_entries_is_selective() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html_good = dir.path().join("good.html");
        std::fs::write(&html_good, b"<html/>").expect("write");

        let mut cache = make_cache(dir.path());
        let good = Path::new("/good.md");
        let stale = Path::new("/stale.md");

        cache.set(
            good,
            CacheEntry {
                rendered_html: html_good.clone(),
                source_mtime_secs: 1,
            },
        );
        cache.set(
            stale,
            CacheEntry {
                rendered_html: dir.path().join("gone.html"),
                source_mtime_secs: 2,
            },
        );

        cache.remove_missing_entries();
        assert!(cache.get(good).is_some(), "good entry must survive");
        assert!(cache.get(stale).is_none(), "stale entry must be pruned");
    }

    // ── cleanup pruning integration ───────────────────────────────────────────

    #[test]
    fn save_creates_parent_directory_if_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let deep = dir.path().join("deep").join("path").join("cache.toml");
        let mut cache = RenderCache::load(deep.clone());
        cache.set(
            Path::new("/x.md"),
            CacheEntry {
                rendered_html: PathBuf::from("/x.html"),
                source_mtime_secs: 7,
            },
        );
        cache.save();
        assert!(deep.exists(), "cache file must be created with parent dirs");
    }
}
