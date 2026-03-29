use std::collections::HashMap;
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Event, Options, Parser, Tag};

use crate::config::Theme;
use crate::copy_clean::{is_supported_language, strip_full_line_comments};

/// Render Markdown source to a complete standalone HTML5 document.
///
/// The output is self-contained: no external stylesheets or scripts are
/// referenced. `title` is used as the `<title>` and shown as an `<h1>` in the
/// page header. `theme` controls the colour scheme of the output.
///
/// # Security note
///
/// Raw HTML embedded in Markdown is passed through unchanged (standard
/// pulldown-cmark behaviour). This is intentional: `mark` is designed to
/// render **local files owned by the user**. Do not use it to render
/// untrusted Markdown — the result could contain executable JavaScript. See
/// the README for details.
pub fn render_markdown(markdown: &str, title: &str, theme: Theme) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, opts);
    let mut body = String::new();
    html::push_html(&mut body, parser);
    let body = post_process_code_blocks(&body);

    build_html_document(title, &body, theme)
}

/// Wrap a rendered HTML body in a complete HTML5 document with embedded CSS and JS.
fn build_html_document(title: &str, body: &str, theme: Theme) -> String {
    let css = include_str!("style.css");

    let copy_css = if theme == Theme::Dark {
        r#".mark-code-block {
  position: relative;
  margin: 1em 0;
}
.mark-code-toolbar {
  display: flex;
  justify-content: flex-end;
  gap: 0.4em;
  padding: 0.25em 0.5em;
  background: #2a2a2a;
  border: 1px solid #444;
  border-bottom: none;
  border-radius: 4px 4px 0 0;
}
.mark-code-block pre {
  margin-top: 0;
  border-radius: 0 0 4px 4px;
}
.mark-btn {
  font-size: 0.78em;
  padding: 0.2em 0.6em;
  border: 1px solid #555;
  border-radius: 3px;
  background: #333;
  color: #ccc;
  cursor: pointer;
  transition: background 0.15s;
}
.mark-btn:hover {
  background: #444;
}
.mark-btn.mark-copied {
  color: #6dbf6d;
  border-color: #6dbf6d;
}
.mark-btn.mark-failed {
  color: #f66;
  border-color: #f66;
}"#
    } else {
        r#".mark-code-block {
  position: relative;
  margin: 1em 0;
}
.mark-code-toolbar {
  display: flex;
  justify-content: flex-end;
  gap: 0.4em;
  padding: 0.25em 0.5em;
  background: #f0f0f0;
  border: 1px solid #ddd;
  border-bottom: none;
  border-radius: 4px 4px 0 0;
}
.mark-code-block pre {
  margin-top: 0;
  border-radius: 0 0 4px 4px;
}
.mark-btn {
  font-size: 0.78em;
  padding: 0.2em 0.6em;
  border: 1px solid #bbb;
  border-radius: 3px;
  background: #fff;
  cursor: pointer;
  transition: background 0.15s;
}
.mark-btn:hover {
  background: #e8e8e8;
}
.mark-btn.mark-copied {
  color: #2a7a2a;
  border-color: #2a7a2a;
}
.mark-btn.mark-failed {
  color: #b00;
  border-color: #b00;
}"#
    };

    let copy_js = r#"(function() {
  function flash(btn, msg, cls) {
    var original = btn.innerHTML;
    btn.textContent = msg;
    btn.classList.add(cls);
    setTimeout(function() {
      btn.innerHTML = original;
      btn.classList.remove(cls);
    }, 1800);
  }

  function copyText(text, btn, successMsg) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(function() {
        flash(btn, successMsg, 'mark-copied');
      }).catch(function() {
        flash(btn, '\u2717 Failed', 'mark-failed');
      });
    } else {
      try {
        var ta = document.createElement('textarea');
        ta.value = text;
        ta.style.position = 'fixed';
        ta.style.opacity = '0';
        document.body.appendChild(ta);
        ta.focus();
        ta.select();
        var ok = false;
        try {
          ok = document.execCommand('copy');
        } finally {
          document.body.removeChild(ta);
        }
        if (ok) {
          flash(btn, successMsg, 'mark-copied');
        } else {
          flash(btn, '\u2717 Failed', 'mark-failed');
        }
      } catch(e) {
        flash(btn, '\u2717 Failed', 'mark-failed');
      }
    }
  }

  document.addEventListener('DOMContentLoaded', function() {
    document.querySelectorAll('.mark-code-block').forEach(function(block) {
      var pre = block.querySelector('pre');
      var code = pre ? pre.querySelector('code') : null;

      var copyBtn = block.querySelector('.mark-copy-btn');
      if (copyBtn && code) {
        copyBtn.addEventListener('click', function() {
          copyText(code.textContent, copyBtn, '\u2713 Copied');
        });
      }

      var cleanBtn = block.querySelector('.mark-copy-clean-btn');
      if (cleanBtn) {
        var cleanCode = block.dataset.cleanCode;
        if (cleanCode !== undefined) {
          cleanBtn.addEventListener('click', function() {
            copyText(cleanCode, cleanBtn, '\u2713 Copied clean');
          });
        }
      }
    });
  });
})();"#;

    let theme_attr = match theme {
        Theme::Dark => "dark",
        Theme::Light => "light",
    };

    // Dark theme overrides for base styles.
    let theme_css = if theme == Theme::Dark {
        r#"
/* dark theme */
body { color: #e0e0e0; background: #1a1a1a; }
h1, h2 { border-bottom-color: #444; }
a { color: #7ab4f5; }
code { background: #2a2a2a; border-color: #444; color: #e0e0e0; }
pre { background: #2a2a2a; border-color: #444; }
blockquote { border-left-color: #555; color: #aaa; background: #222; }
th { background: #2a2a2a; }
tr:nth-child(even) { background: #222; }
th, td { border-color: #444; }
hr { border-top-color: #444; }"#
    } else {
        ""
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en" data-theme="{theme_attr}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
{css}
{theme_css}
</style>
<style>
{copy_css}
</style>
</head>
<body>
<article>
{body}
</article>
<script>
{copy_js}
</script>
</body>
</html>
"#,
        theme_attr = theme_attr,
        title = escape_html(title),
        css = css,
        theme_css = theme_css,
        copy_css = copy_css,
        body = body,
        copy_js = copy_js,
    )
}

/// Escape special HTML characters in a plain-text value.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Decode a small set of HTML entities using a single-pass scan so that
/// sequences like `&amp;lt;` are decoded to `&lt;` (not to `<`).
fn html_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut remaining = s;
    while let Some(amp_pos) = remaining.find('&') {
        result.push_str(&remaining[..amp_pos]);
        remaining = &remaining[amp_pos..];
        if remaining.starts_with("&amp;") {
            result.push('&');
            remaining = &remaining["&amp;".len()..];
        } else if remaining.starts_with("&lt;") {
            result.push('<');
            remaining = &remaining["&lt;".len()..];
        } else if remaining.starts_with("&gt;") {
            result.push('>');
            remaining = &remaining["&gt;".len()..];
        } else if remaining.starts_with("&quot;") {
            result.push('"');
            remaining = &remaining["&quot;".len()..];
        } else if remaining.starts_with("&#39;") {
            result.push('\'');
            remaining = &remaining["&#39;".len()..];
        } else if remaining.starts_with("&#x27;") {
            result.push('\'');
            remaining = &remaining["&#x27;".len()..];
        } else {
            // Unknown entity — pass through the `&` literally.
            result.push('&');
            remaining = &remaining[1..];
        }
    }
    result.push_str(remaining);
    result
}

/// Encode a string for safe use as an HTML attribute value.
fn html_encode_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Wrap every `<pre><code…>…</code></pre>` block in a toolbar div with copy
/// buttons.  For supported languages a `data-clean-code` attribute is added
/// and a second "Copy clean" button is included.
pub(crate) fn post_process_code_blocks(html: &str) -> String {
    let mut output = String::with_capacity(html.len() + 512);
    let mut rest = html;

    while let Some(pre_start) = rest.find("<pre>") {
        // Emit everything before this `<pre>`.
        output.push_str(&rest[..pre_start]);
        rest = &rest[pre_start..];

        // Find the matching `</pre>`.
        let Some(pre_end) = rest.find("</pre>") else {
            break;
        };
        let pre_end = pre_end + "</pre>".len();
        let pre_block = &rest[..pre_end]; // the full `<pre>…</pre>` slice
        rest = &rest[pre_end..];

        // Extract the language from `<code class="language-LANG">` if present.
        let lang: Option<&str> = pre_block.find(r#"class="language-"#).and_then(|pos| {
            let after = &pre_block[pos + r#"class="language-"#.len()..];
            let end = after.find('"')?;
            Some(&after[..end])
        });

        // Extract the raw (HTML-encoded) code text between `<code…>` and `</code>`.
        let raw_code: &str = pre_block
            .find('>')
            .and_then(|open_gt| {
                let after_open_tag = &pre_block[open_gt + 1..];
                // Skip into the `<code…>` tag.
                after_open_tag.find('>').and_then(|code_gt| {
                    let after_code_tag = &after_open_tag[code_gt + 1..];
                    let end = after_code_tag.find("</code>")?;
                    Some(&after_code_tag[..end])
                })
            })
            .unwrap_or("");

        let toolbar_and_wrapper = if let Some(lang_str) = lang {
            if is_supported_language(lang_str) {
                let decoded = html_decode(raw_code);
                let cleaned = strip_full_line_comments(lang_str, &decoded);
                let encoded_clean = html_encode_attr(&cleaned);
                format!(
                    r#"<div class="mark-code-block" data-clean-code="{encoded_clean}">
<div class="mark-code-toolbar">
<button class="mark-btn mark-copy-btn" type="button">&#x1F4CB; Copy</button>
<button class="mark-btn mark-copy-clean-btn" type="button">&#x1F9F9; Copy clean</button>
</div>
{pre_block}
</div>"#
                )
            } else {
                // Known language, but not one we support for clean-copy.
                format!(
                    r#"<div class="mark-code-block">
<div class="mark-code-toolbar">
<button class="mark-btn mark-copy-btn" type="button">&#x1F4CB; Copy</button>
</div>
{pre_block}
</div>"#
                )
            }
        } else {
            // No language annotation.
            format!(
                r#"<div class="mark-code-block">
<div class="mark-code-toolbar">
<button class="mark-btn mark-copy-btn" type="button">&#x1F4CB; Copy</button>
</div>
{pre_block}
</div>"#
            )
        };

        output.push_str(&toolbar_and_wrapper);
    }

    // Append anything remaining after the last `</pre>`.
    output.push_str(rest);
    output
}

// ── Link extraction ──────────────────────────────────────────────────────────

/// Returns `true` if the URL looks like an external or non-file reference that
/// should never be rewritten.
fn is_external_url(url: &str) -> bool {
    url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("mailto:")
        || url.starts_with("//")
        || url.starts_with('#')
}

/// Split `url` into `(base, fragment)` at the first `#`.
///
/// If there is no `#`, `fragment` is an empty string.
fn split_fragment(url: &str) -> (&str, &str) {
    match url.find('#') {
        Some(pos) => (&url[..pos], &url[pos..]),
        None => (url, ""),
    }
}

/// Returns `true` if `path` ends with `.md` or `.markdown` (case-insensitive).
fn is_md_extension(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".md") || lower.ends_with(".markdown")
}

/// Extract all local Markdown links from `markdown` that resolve to existing
/// files relative to `source_dir`.
///
/// A link is included when:
/// - it is not an external URL (`http://`, `https://`, `mailto:`, `//`, `#`),
/// - its base path (without any `#fragment`) ends with `.md` or `.markdown`
///   (case-insensitive), and
/// - the resolved path exists on disk (confirmed via
///   [`std::fs::canonicalize`]).
///
/// Image links are **not** included; only `[text](target)` / `[text][ref]`
/// style links are considered.
///
/// # Returns
/// A `Vec` of `(target_base, canonical_path)` tuples in document order,
/// deduplicated by `target_base`.  `target_base` is the link destination with
/// the fragment stripped so it can be used as the key in a rewrite map.
pub fn extract_local_md_links(markdown: &str, source_dir: &Path) -> Vec<(String, PathBuf)> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, opts);
    let mut links: Vec<(String, PathBuf)> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for event in parser {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            let url = dest_url.as_ref();
            if is_external_url(url) {
                continue;
            }
            let (base, _fragment) = split_fragment(url);
            if !is_md_extension(base) {
                continue;
            }
            if seen.contains(base) {
                continue;
            }
            // Resolve and canonicalize.
            let resolved = source_dir.join(base);
            if let Ok(canonical) = std::fs::canonicalize(&resolved) {
                if canonical.is_file() {
                    seen.insert(base.to_string());
                    links.push((base.to_string(), canonical));
                }
            }
        }
    }
    links
}

// ── Rendering with link rewriting ────────────────────────────────────────────

/// Render `markdown` to a complete HTML5 document, rewriting every local
/// `.md` link destination to the corresponding rendered `.html` path.
///
/// `link_map` maps a link's **base target** (the part before any `#`) to the
/// absolute [`PathBuf`] of its rendered HTML file.  Anchor fragments in the
/// original link are preserved on the rewritten `href`.
///
/// External URLs and links whose base is not present in `link_map` are left
/// unchanged.  When `link_map` is empty the output is identical to
/// [`render_markdown`].
///
/// The rewriting is performed by transforming pulldown-cmark link events
/// before passing them to the HTML serialiser, so it operates on the parsed
/// AST rather than the raw HTML string.
pub fn render_markdown_rewriting_links(
    markdown: &str,
    title: &str,
    theme: Theme,
    link_map: &HashMap<String, PathBuf>,
) -> String {
    if link_map.is_empty() {
        return render_markdown(markdown, title, theme);
    }

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    // Collect into a Vec so we own all event data before calling push_html.
    let events: Vec<Event<'_>> = Parser::new_ext(markdown, opts)
        .map(|event| match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title: link_title,
                id,
            }) => {
                let url = dest_url.as_ref();
                if is_external_url(url) {
                    return Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title: link_title,
                        id,
                    });
                }
                let (base, fragment) = split_fragment(url);
                if let Some(html_path) = link_map.get(base) {
                    let new_url: String = if fragment.is_empty() {
                        html_path.to_string_lossy().into_owned()
                    } else {
                        format!("{}{}", html_path.to_string_lossy(), fragment)
                    };
                    Event::Start(Tag::Link {
                        link_type,
                        dest_url: new_url.into(),
                        title: link_title,
                        id,
                    })
                } else {
                    Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title: link_title,
                        id,
                    })
                }
            }
            other => other,
        })
        .collect();

    let mut body = String::new();
    html::push_html(&mut body, events.into_iter());
    let body = post_process_code_blocks(&body);
    build_html_document(title, &body, theme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Theme;

    #[test]
    fn render_produces_nonempty_html() {
        let html = render_markdown("# Hello\n\nWorld.", "Hello", Theme::Light);
        assert!(!html.is_empty());
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn render_contains_heading() {
        let html = render_markdown("# My Title\n", "My Title", Theme::Light);
        assert!(html.contains("<h1>My Title</h1>"));
    }

    #[test]
    fn render_contains_paragraph() {
        let html = render_markdown("Hello world\n", "test", Theme::Light);
        assert!(html.contains("<p>Hello world</p>"));
    }

    #[test]
    fn render_title_in_head() {
        let html = render_markdown("text", "My Doc", Theme::Light);
        assert!(html.contains("<title>My Doc</title>"));
    }

    #[test]
    fn render_escapes_title_html() {
        let html = render_markdown("x", "<script>alert(1)</script>", Theme::Light);
        assert!(html.contains("&lt;script&gt;"));
        // The raw injected script should not appear as an executable tag in the title.
        assert!(!html.contains("<title><script>"));
        assert!(!html.contains("<script>alert(1)"));
    }

    #[test]
    fn render_includes_embedded_css() {
        let html = render_markdown("x", "t", Theme::Light);
        assert!(html.contains("<style>"));
    }

    #[test]
    fn render_light_theme_has_data_attribute() {
        let html = render_markdown("x", "t", Theme::Light);
        assert!(html.contains(r#"data-theme="light""#));
    }

    #[test]
    fn render_dark_theme_has_data_attribute() {
        let html = render_markdown("x", "t", Theme::Dark);
        assert!(html.contains(r#"data-theme="dark""#));
    }

    #[test]
    fn render_dark_theme_contains_dark_css() {
        let html = render_markdown("x", "t", Theme::Dark);
        // Dark theme CSS includes a dark background colour.
        assert!(html.contains("background: #1a1a1a"));
    }

    #[test]
    fn render_light_theme_no_dark_css() {
        let html = render_markdown("x", "t", Theme::Light);
        assert!(!html.contains("background: #1a1a1a"));
    }

    #[test]
    fn code_block_has_copy_button() {
        let md = "```\nsome code\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        assert!(
            html.contains("mark-copy-btn"),
            "copy button should be present"
        );
    }

    #[test]
    fn code_block_supported_lang_has_clean_button() {
        let md = "```rust\nfn main() {}\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        assert!(
            html.contains(r#"class="mark-btn mark-copy-clean-btn""#),
            "clean button element should be present for rust"
        );
    }

    #[test]
    fn code_block_unsupported_lang_no_clean_button() {
        let md = "```html\n<p>hello</p>\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        // Check the button element itself is absent (not just the class string,
        // which also appears inside the embedded JS).
        assert!(
            !html.contains(r#"class="mark-btn mark-copy-clean-btn""#),
            "clean button element should NOT be present for html"
        );
    }

    #[test]
    fn code_block_no_lang_no_clean_button() {
        let md = "```\nsome code\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        assert!(
            !html.contains(r#"class="mark-btn mark-copy-clean-btn""#),
            "clean button element should NOT be present when no language specified"
        );
    }

    // ── extract_local_md_links tests ─────────────────────────────────────────

    #[test]
    fn extract_links_finds_local_md() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Create a target file so canonicalize succeeds.
        let target = dir.path().join("chapter.md");
        std::fs::write(&target, "# Chapter").expect("write");

        let md = "[Chapter](chapter.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0, "chapter.md");
        assert_eq!(links[0].1, target.canonicalize().unwrap());
    }

    #[test]
    fn extract_links_skips_external_urls() {
        let dir = tempfile::tempdir().expect("tempdir");
        let md = "[Google](https://google.com)\n[Local](http://localhost/foo.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "external links must not be extracted");
    }

    #[test]
    fn extract_links_skips_pure_anchors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let md = "[Section](#heading)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "pure anchor links must not be extracted");
    }

    #[test]
    fn extract_links_skips_non_md_extensions() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Create a real file so the only filter is the extension.
        let pdf = dir.path().join("report.pdf");
        std::fs::write(&pdf, "data").expect("write");
        let md = "[Report](report.pdf)\n[Image](photo.png)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "non-.md links must not be extracted");
    }

    #[test]
    fn extract_links_deduplicates_same_target() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("api.md");
        std::fs::write(&target, "# API").expect("write");
        let md = "[API](api.md)\n[API again](api.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert_eq!(links.len(), 1, "duplicate links must appear only once");
    }

    #[test]
    fn extract_links_strips_fragment_for_key() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("api.md");
        std::fs::write(&target, "# API").expect("write");
        let md = "[Section](api.md#endpoints)\n";
        let links = extract_local_md_links(md, dir.path());
        assert_eq!(links.len(), 1);
        // The key must be the base without the fragment.
        assert_eq!(links[0].0, "api.md");
    }

    #[test]
    fn extract_links_ignores_nonexistent_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        // No actual file created — canonicalize will fail.
        let md = "[Ghost](ghost.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "nonexistent files must not be extracted");
    }

    // ── render_markdown_rewriting_links tests ─────────────────────────────────

    #[test]
    fn rewrite_links_replaces_md_href() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("api.md");
        std::fs::write(&target, "# API").expect("write");

        let html_path = dir.path().join("api-123-abcd1234.html");
        let mut link_map = HashMap::new();
        link_map.insert("api.md".to_string(), html_path.clone());

        let md = "[API](api.md)\n";
        let html = render_markdown_rewriting_links(md, "t", Theme::Light, &link_map);
        let expected = format!("href=\"{}\"", html_path.display());
        assert!(
            html.contains(&expected),
            "expected rewritten href in html:\n{html}"
        );
        assert!(
            !html.contains("href=\"api.md\""),
            "original .md href must not survive"
        );
    }

    #[test]
    fn rewrite_links_preserves_fragment() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html_path = dir.path().join("api-123-abcd1234.html");
        let mut link_map = HashMap::new();
        link_map.insert("api.md".to_string(), html_path.clone());

        let md = "[Section](api.md#endpoints)\n";
        let html = render_markdown_rewriting_links(md, "t", Theme::Light, &link_map);
        let expected = format!("href=\"{}#endpoints\"", html_path.display());
        assert!(
            html.contains(&expected),
            "fragment must be preserved:\n{html}"
        );
    }

    #[test]
    fn rewrite_links_leaves_external_urls_unchanged() {
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let md = "[Google](https://google.com)\n";
        let html = render_markdown_rewriting_links(md, "t", Theme::Light, &link_map);
        assert!(
            html.contains("href=\"https://google.com\""),
            "external URL must not be modified:\n{html}"
        );
    }

    #[test]
    fn rewrite_links_leaves_non_md_local_links_unchanged() {
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let md = "[Image](photo.png)\n";
        let html = render_markdown_rewriting_links(md, "t", Theme::Light, &link_map);
        assert!(
            html.contains("href=\"photo.png\""),
            "non-md link must not be modified:\n{html}"
        );
    }

    #[test]
    fn rewrite_links_empty_map_is_identity() {
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let md = "[Chapter](chapter.md)\n";
        let html_rewrite = render_markdown_rewriting_links(md, "t", Theme::Light, &link_map);
        let html_plain = render_markdown(md, "t", Theme::Light);
        assert_eq!(
            html_rewrite, html_plain,
            "empty link_map must produce identical output to render_markdown"
        );
    }
}
