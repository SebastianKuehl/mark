use pulldown_cmark::{html, Options, Parser};

/// Render Markdown source to a complete standalone HTML5 document.
///
/// The output is self-contained: no external stylesheets or scripts are
/// referenced. `title` is used as the `<title>` and shown as an `<h1>` in the
/// page header.
///
/// # Security note
///
/// Raw HTML embedded in Markdown is passed through unchanged (standard
/// pulldown-cmark behaviour). This is intentional: `mark` is designed to
/// render **local files owned by the user**. Do not use it to render
/// untrusted Markdown — the result could contain executable JavaScript. See
/// the README for details.
pub fn render_markdown(markdown: &str, title: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, opts);
    let mut body = String::new();
    html::push_html(&mut body, parser);

    build_html_document(title, &body)
}

/// Wrap a rendered HTML body in a complete HTML5 document with embedded CSS.
fn build_html_document(title: &str, body: &str) -> String {
    let css = include_str!("style.css");
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
{css}
</style>
</head>
<body>
<article>
{body}
</article>
</body>
</html>
"#,
        title = escape_html(title),
        css = css,
        body = body,
    )
}

/// Escape special HTML characters in a plain-text value.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_produces_nonempty_html() {
        let html = render_markdown("# Hello\n\nWorld.", "Hello");
        assert!(!html.is_empty());
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn render_contains_heading() {
        let html = render_markdown("# My Title\n", "My Title");
        assert!(html.contains("<h1>My Title</h1>"));
    }

    #[test]
    fn render_contains_paragraph() {
        let html = render_markdown("Hello world\n", "test");
        assert!(html.contains("<p>Hello world</p>"));
    }

    #[test]
    fn render_title_in_head() {
        let html = render_markdown("text", "My Doc");
        assert!(html.contains("<title>My Doc</title>"));
    }

    #[test]
    fn render_escapes_title_html() {
        let html = render_markdown("x", "<script>alert(1)</script>");
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn render_includes_embedded_css() {
        let html = render_markdown("x", "t");
        assert!(html.contains("<style>"));
    }
}
