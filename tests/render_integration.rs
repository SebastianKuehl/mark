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
        let html = mark::render::render_markdown(&markdown, title);

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
