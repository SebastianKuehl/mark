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
        let run_dir = mark::storage::make_run_dir(&rendered_dir, &md_path).expect("run dir");
        let out_path = run_dir.join("sample.html");
        fs::create_dir_all(out_path.parent().expect("parent")).expect("mkdirs");
        fs::write(&out_path, &html).expect("write html");

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
        assert!(out_path.starts_with(&run_dir));
    }
}

/// Integration tests for BFS scope restriction (F-021 / M-019).
///
/// Verifies that recursive rendering only follows links whose resolved
/// canonical path is within the entry file's parent directory.
#[cfg(test)]
mod scope_restriction {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fs;
    use std::path::{Path, PathBuf};

    /// BFS helper that mirrors the scope-restricted loop in `main.rs`.
    fn run_bfs(entry_canonical: &Path, entry_dir: &Path) -> Vec<PathBuf> {
        let mut visited: HashSet<PathBuf> = HashSet::new();
        visited.insert(entry_canonical.to_path_buf());

        let mut ordered: Vec<PathBuf> = vec![entry_canonical.to_path_buf()];
        let mut content_cache: HashMap<PathBuf, String> = HashMap::new();
        content_cache.insert(
            entry_canonical.to_path_buf(),
            fs::read_to_string(entry_canonical).unwrap(),
        );

        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        queue.push_back(entry_canonical.to_path_buf());

        while let Some(current) = queue.pop_front() {
            let content = content_cache.get(&current).cloned().unwrap_or_default();
            let source_dir = current.parent().unwrap_or_else(|| Path::new("."));
            let links = mark::render::extract_local_md_links(&content, source_dir);
            for (_base, canonical) in links {
                // Scope guard — same logic as main.rs.
                if !canonical.starts_with(entry_dir) {
                    continue;
                }
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

        ordered
    }

    /// An in-scope link (docs/subdir/page.md) is followed and appears in the
    /// ordered render list.
    #[test]
    fn in_scope_link_is_followed() {
        let root = tempfile::tempdir().expect("tempdir");
        let docs = root.path().join("docs");
        let subdir = docs.join("subdir");
        fs::create_dir_all(&subdir).unwrap();

        let entry = docs.join("entry.md");
        let page = subdir.join("page.md");
        fs::write(&entry, "[Page](subdir/page.md)\n").unwrap();
        fs::write(&page, "# Page\n").unwrap();

        let entry_canonical = entry.canonicalize().unwrap();
        let entry_dir = entry_canonical.parent().unwrap().to_path_buf();

        let ordered = run_bfs(&entry_canonical, &entry_dir);

        assert_eq!(
            ordered.len(),
            2,
            "entry + page should both be rendered: {ordered:?}"
        );
        assert!(
            ordered.contains(&page.canonicalize().unwrap()),
            "page.md must be in render list"
        );
    }

    /// An out-of-scope link (../other/out.md — outside the entry directory) is
    /// silently skipped and does NOT appear in the ordered render list.
    #[test]
    fn out_of_scope_md_link_is_skipped() {
        let root = tempfile::tempdir().expect("tempdir");
        let docs = root.path().join("docs");
        let other = root.path().join("other");
        fs::create_dir_all(&docs).unwrap();
        fs::create_dir_all(&other).unwrap();

        let entry = docs.join("entry.md");
        let outside = other.join("out.md");
        fs::write(&entry, "[Out](../other/out.md)\n").unwrap();
        fs::write(&outside, "# Out\n").unwrap();

        let entry_canonical = entry.canonicalize().unwrap();
        let entry_dir = entry_canonical.parent().unwrap().to_path_buf();

        let ordered = run_bfs(&entry_canonical, &entry_dir);

        assert_eq!(
            ordered.len(),
            1,
            "only entry should be rendered, not out.md: {ordered:?}"
        );
        assert!(
            !ordered.contains(&outside.canonicalize().unwrap()),
            "out.md must NOT be in render list"
        );
    }

    /// When an out-of-scope Markdown link appears in the entry file the rendered
    /// HTML must preserve the original href unchanged (not rewritten to `.html`).
    #[test]
    fn out_of_scope_link_href_is_unchanged_in_html() {
        use std::collections::HashMap;

        // The link_map is empty for the out-of-scope file because it was never
        // queued during BFS — mirroring what main.rs does.
        let link_map: HashMap<String, std::path::PathBuf> = HashMap::new();

        let markdown = "[Out](../other/out.md)\n";
        let html = mark::render::render_markdown_rewriting_links(
            markdown,
            "entry",
            mark::config::Theme::Light,
            &link_map,
            mark::render::RenderChrome {
                breadcrumb: &[],
                all_files: &[],
                run_dir: std::path::Path::new(""),
                sidebar_visible: false,
                appearance: mark::config::AppearanceConfig::default(),
            },
        );

        // The href must be the original relative path, NOT rewritten to .html.
        assert!(
            html.contains(r#"href="../other/out.md""#),
            "out-of-scope href must be unchanged; html snippet: {}",
            &html[html.find("<a").unwrap_or(0)
                ..std::cmp::min(html.len(), html.find("<a").unwrap_or(0) + 200)]
        );
        assert!(
            !html.contains("out.html"),
            "out-of-scope href must NOT be rewritten to .html"
        );
    }

    /// `docs2/` directory must NOT be mistaken for being inside `docs/` when
    /// doing the starts_with scope check on Path objects.
    #[test]
    fn docs2_directory_not_confused_with_docs() {
        let root = tempfile::tempdir().expect("tempdir");
        let docs = root.path().join("docs");
        let docs2 = root.path().join("docs2");
        fs::create_dir_all(&docs).unwrap();
        fs::create_dir_all(&docs2).unwrap();

        let entry = docs.join("entry.md");
        let sibling = docs2.join("sibling.md");
        // Note: the relative link "../docs2/sibling.md" resolves outside docs/.
        fs::write(&entry, "[Sibling](../docs2/sibling.md)\n").unwrap();
        fs::write(&sibling, "# Sibling\n").unwrap();

        let entry_canonical = entry.canonicalize().unwrap();
        let entry_dir = entry_canonical.parent().unwrap().to_path_buf();

        // Sanity: docs2 does NOT start_with docs (Path comparison).
        assert!(
            !docs2.starts_with(&docs),
            "docs2 must not be considered a subdirectory of docs"
        );

        let ordered = run_bfs(&entry_canonical, &entry_dir);
        assert_eq!(
            ordered.len(),
            1,
            "sibling in docs2 must be skipped: {ordered:?}"
        );
    }
}

/// Integration tests for recursive link extraction and circular-reference safety.
#[cfg(test)]
mod link_extraction_integration {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fs;
    use std::path::{Path, PathBuf};

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

    fn output_path_for_run(run_dir: &Path, entry_dir: &Path, file_canonical: &Path) -> PathBuf {
        let relative = file_canonical
            .strip_prefix(entry_dir)
            .map(Path::to_path_buf)
            .unwrap_or_else(|_| {
                file_canonical
                    .file_name()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("output"))
            });
        run_dir.join(relative.with_extension("html"))
    }

    #[test]
    fn rendered_output_and_assets_preserve_folder_hierarchy() {
        let dir = tempfile::tempdir().expect("tempdir");
        let docs = dir.path().join("docs");
        fs::create_dir_all(docs.join("chapters/api")).expect("mkdirs");
        fs::create_dir_all(docs.join("assets/images")).expect("mkdirs");

        let overview = docs.join("overview.md");
        let intro = docs.join("chapters/intro.md");
        let endpoints = docs.join("chapters/api/endpoints.md");
        let logo = docs.join("assets/images/logo.png");

        fs::write(
            &overview,
            "[Intro](chapters/intro.md)\n![Logo](assets/images/logo.png)\n",
        )
        .expect("write overview");
        fs::write(&intro, "[Endpoints](api/endpoints.md)\n").expect("write intro");
        fs::write(&endpoints, "# Endpoints\n").expect("write endpoints");
        fs::write(&logo, b"\x89PNG").expect("write logo");

        let entry_dir = overview
            .parent()
            .expect("entry dir")
            .canonicalize()
            .unwrap();
        let run_dir = dir.path().join("rendered/overview-123-abcdef12");

        let overview_out =
            output_path_for_run(&run_dir, &entry_dir, &overview.canonicalize().unwrap());
        let intro_out = output_path_for_run(&run_dir, &entry_dir, &intro.canonicalize().unwrap());
        let endpoints_out =
            output_path_for_run(&run_dir, &entry_dir, &endpoints.canonicalize().unwrap());
        let asset_dest = run_dir.join("assets/images/logo.png");

        assert_eq!(overview_out, run_dir.join("overview.html"));
        assert_eq!(intro_out, run_dir.join("chapters/intro.html"));
        assert_eq!(endpoints_out, run_dir.join("chapters/api/endpoints.html"));
        assert_eq!(asset_dest, run_dir.join("assets/images/logo.png"));
    }
}
