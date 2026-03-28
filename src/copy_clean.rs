/// Languages supported by `strip_full_line_comments`.
const SUPPORTED_LANGS: &[&str] = &[
    "bash",
    "sh",
    "zsh",
    "fish",
    "powershell",
    "python",
    "rust",
    "javascript",
    "typescript",
];

/// Returns `true` when `lang` is a language for which we know how to strip
/// full-line comments.
pub fn is_supported_language(lang: &str) -> bool {
    SUPPORTED_LANGS.contains(&lang)
}

/// Remove full-line comments from `code` for the given `lang`.
///
/// A "full-line comment" is a line whose trimmed content starts with the
/// language's single-line comment marker.  Inline comments (code followed by
/// a comment on the same line) are **not** removed.  Empty lines and lines
/// for unsupported languages are passed through unchanged.
///
/// Comment markers used:
/// - `#`  — bash, sh, zsh, fish, powershell, python
/// - `//` — rust, javascript, typescript
pub fn strip_full_line_comments(lang: &str, code: &str) -> String {
    let marker: Option<&str> = match lang {
        "bash" | "sh" | "zsh" | "fish" | "powershell" | "python" => Some("#"),
        "rust" | "javascript" | "typescript" => Some("//"),
        _ => None,
    };

    match marker {
        None => code.to_owned(),
        Some(m) => code
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                // Keep empty lines and lines that do NOT start with the marker.
                trimmed.is_empty() || !trimmed.starts_with(m)
            })
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_comments_rust() {
        let code = "// comment\nfn main() {}\n// another\n";
        let result = strip_full_line_comments("rust", code);
        assert!(
            !result.contains("// comment"),
            "full-line comment should be removed"
        );
        assert!(
            !result.contains("// another"),
            "second full-line comment should be removed"
        );
        assert!(
            result.contains("fn main()"),
            "code line should be preserved"
        );
    }

    #[test]
    fn strip_comments_python() {
        let code = "# comment\nx = 1\n# another\n";
        let result = strip_full_line_comments("python", code);
        assert!(
            !result.contains("# comment"),
            "full-line comment should be removed"
        );
        assert!(
            !result.contains("# another"),
            "second full-line comment should be removed"
        );
        assert!(result.contains("x = 1"), "code line should be preserved");
    }

    #[test]
    fn strip_comments_bash() {
        let code = "# comment\necho hello\n# another\n";
        let result = strip_full_line_comments("bash", code);
        assert!(
            !result.contains("# comment"),
            "full-line comment should be removed"
        );
        assert!(
            result.contains("echo hello"),
            "code line should be preserved"
        );
    }

    #[test]
    fn strip_comments_keeps_inline() {
        let code = r#"println!("hello"); // comment"#;
        let result = strip_full_line_comments("rust", code);
        assert!(
            result.contains(r#"println!("hello"); // comment"#),
            "inline comment line should be preserved as-is"
        );
    }

    #[test]
    fn supported_languages() {
        for lang in &[
            "bash",
            "sh",
            "zsh",
            "fish",
            "powershell",
            "python",
            "rust",
            "javascript",
            "typescript",
        ] {
            assert!(is_supported_language(lang), "{lang} should be supported");
        }
    }

    #[test]
    fn unsupported_language() {
        for lang in &["sql", "html", "css", "unknown", ""] {
            assert!(
                !is_supported_language(lang),
                "{lang} should not be supported"
            );
        }
    }
}
