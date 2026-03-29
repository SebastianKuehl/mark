/// Integration test: render flow writes HTML to a temp directory without
/// opening a browser (simulates `mark --no-open <file>`).
#[cfg(test)]
mod render_flow {
    use std::fs;

    #[test]
    fn render_flow_writes_html_to_temp_dir() {
        let dir = tempfile::tempdir().expect("tempdir");

        // Write a small Markdown source file.
        let md_path = dir.path().join("sample.md");
        fs::write(&md_path, "# Hello\n\nThis is a **test**.\n").expect("write md");

        // Render it.
        let markdown = fs::read_to_string(&md_path).expect("read md");
        let title = md_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let html = mark::render::render_markdown(&markdown, title, mark::config::Theme::Light);

        // Write it.
        let rendered_dir = dir.path().join("rendered");
        let filename = mark::storage::output_filename(&md_path);
        let out_path =
            mark::storage::write_rendered(&rendered_dir, &filename, &html).expect("write html");

        // Verify.
        assert!(out_path.exists(), "output file should exist");
        let content = fs::read_to_string(&out_path).expect("read html");
        assert!(
            content.contains("<!DOCTYPE html>"),
            "should be full document"
        );
        assert!(content.contains("<h1>Hello</h1>"), "should contain heading");
        assert!(
            content.contains("<strong>test</strong>"),
            "should contain bold"
        );
        assert!(out_path.extension().map(|e| e == "html").unwrap_or(false));
    }
}

/// Integration tests for recursive link extraction and circular-reference safety.
#[cfg(test)]
mod link_extraction_integration {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fs;
    use std::path::PathBuf;

    /// Simulate the BFS loop from main.rs to verify circular links terminate
    /// without duplicate renders and without a stack overflow.
    ///
    /// Graph: a.md → b.md → a.md  (circular)
    #[test]
    fn circular_links_do_not_loop_forever() {
        let dir = tempfile::tempdir().expect("tempdir");
        let a = dir.path().join("a.md");
        let b = dir.path().join("b.md");
        fs::write(&a, "[B](b.md)\n").expect("write a");
        fs::write(&b, "[A](a.md)\n").expect("write b");

        let entry_canonical = a.canonicalize().expect("canonicalize a");

        let mut visited: HashSet<PathBuf> = HashSet::new();
        visited.insert(entry_canonical.clone());

        let mut ordered: Vec<PathBuf> = vec![entry_canonical.clone()];
        let mut content_cache: HashMap<PathBuf, String> = HashMap::new();
        content_cache.insert(entry_canonical.clone(), fs::read_to_string(&a).unwrap());

        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        queue.push_back(entry_canonical.clone());

        while let Some(current) = queue.pop_front() {
            let content = content_cache.get(&current).cloned().unwrap_or_default();
            let source_dir = current.parent().unwrap();
            let links = mark::render::extract_local_md_links(&content, source_dir);
            for (_base, canonical) in links {
                if visited.contains(&canonical) {
                    continue;
                }
                visited.insert(canonical.clone());
                if let Ok(md) = fs::read_to_string(&canonical) {
                    content_cache.insert(canonical.clone(), md);
                    ordered.push(canonical.clone());
                    queue.push_back(canonical);
                }
            }
        }

        // Both a.md and b.md should have been visited exactly once.
        assert_eq!(ordered.len(), 2, "exactly two files: {ordered:?}");
        assert!(
            visited.contains(&a.canonicalize().unwrap()),
            "a.md must be visited"
        );
        assert!(
            visited.contains(&b.canonicalize().unwrap()),
            "b.md must be visited"
        );
    }

    /// Verify that a file linked from multiple parents is rendered only once.
    #[test]
    fn shared_linked_file_rendered_once() {
        let dir = tempfile::tempdir().expect("tempdir");
        let overview = dir.path().join("overview.md");
        let chapter = dir.path().join("chapter.md");
        let shared = dir.path().join("shared.md");
        fs::write(&overview, "[Chapter](chapter.md)\n[Shared](shared.md)\n").unwrap();
        fs::write(&chapter, "[Shared](shared.md)\n").unwrap();
        fs::write(&shared, "# Shared\n").unwrap();

        let entry_canonical = overview.canonicalize().unwrap();
        let mut visited: HashSet<PathBuf> = HashSet::new();
        visited.insert(entry_canonical.clone());
        let mut ordered: Vec<PathBuf> = vec![entry_canonical.clone()];
        let mut content_cache: HashMap<PathBuf, String> = HashMap::new();
        content_cache.insert(
            entry_canonical.clone(),
            fs::read_to_string(&overview).unwrap(),
        );
        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        queue.push_back(entry_canonical.clone());

        while let Some(current) = queue.pop_front() {
            let content = content_cache.get(&current).cloned().unwrap_or_default();
            let source_dir = current.parent().unwrap();
            for (_base, canonical) in mark::render::extract_local_md_links(&content, source_dir) {
                if !visited.contains(&canonical) {
                    visited.insert(canonical.clone());
                    if let Ok(md) = fs::read_to_string(&canonical) {
                        content_cache.insert(canonical.clone(), md);
                        ordered.push(canonical.clone());
                        queue.push_back(canonical);
                    }
                }
            }
        }

        // overview, chapter, shared — shared must appear exactly once.
        assert_eq!(ordered.len(), 3, "three distinct files: {ordered:?}");
        let shared_canonical = shared.canonicalize().unwrap();
        let count = ordered.iter().filter(|p| **p == shared_canonical).count();
        assert_eq!(
            count, 1,
            "shared.md must appear exactly once in ordered list"
        );
    }
}
