#[cfg(test)]
mod view_controls {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output};

    fn run_mark(home: &Path, cwd: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mark"))
            .args(args)
            .current_dir(cwd)
            .env("HOME", home)
            .output()
            .expect("run mark")
    }

    fn rendered_path(stdout: &str) -> PathBuf {
        let rendered_line = stdout
            .lines()
            .find(|line| line.starts_with("Rendered: "))
            .expect("Rendered line");
        PathBuf::from(rendered_line.trim_start_matches("Rendered: ").trim())
    }

    #[test]
    fn config_commands_persist_render_mode_sidebar_and_system_theme() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        fs::create_dir_all(&home).expect("home");

        let output = run_mark(
            &home,
            sandbox.path(),
            &["config", "set-render-mode", "single"],
        );
        assert!(output.status.success(), "{output:?}");

        let output = run_mark(&home, sandbox.path(), &["config", "set-sidebar", "visible"]);
        assert!(output.status.success(), "{output:?}");

        let output = run_mark(&home, sandbox.path(), &["config", "set-theme", "system"]);
        assert!(output.status.success(), "{output:?}");

        let config = fs::read_to_string(home.join(".mark/config.toml")).expect("config");
        assert!(config.contains("render_mode = \"single\""), "{config}");
        assert!(config.contains("sidebar = \"visible\""), "{config}");
        assert!(config.contains("theme = \"system\""), "{config}");
    }

    #[test]
    fn single_mode_renders_only_entry_file_and_notes_skipped_links() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        let docs = sandbox.path().join("docs");
        fs::create_dir_all(home.join(".mark")).expect("home");
        fs::create_dir_all(docs.join("chapters")).expect("chapters");

        let overview = docs.join("overview.md");
        let intro = docs.join("chapters/intro.md");
        fs::write(&overview, "[Intro](chapters/intro.md)\n# Overview\n").expect("overview");
        fs::write(&intro, "# Intro\n").expect("intro");

        let output = run_mark(
            &home,
            &docs,
            &["--single", "--no-open", overview.to_str().expect("path")],
        );
        assert!(
            output.status.success(),
            "stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Note: single mode skipped local Markdown links: chapters/intro.md"),
            "{stdout}"
        );

        let rendered = rendered_path(&stdout);
        let html = fs::read_to_string(&rendered).expect("rendered html");
        assert!(!html.contains("id=\"mark-sidebar\""), "{html}");
        assert!(!html.contains("&lt;put rendered html here&gt;"), "{html}");
        assert!(html.contains("href=\"chapters/intro.md\""), "{html}");
        assert!(!rendered
            .parent()
            .expect("run dir")
            .join("chapters/intro.html")
            .exists());
    }

    #[test]
    fn recursive_mode_preserves_sidebar_and_rewrites_links() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        let docs = sandbox.path().join("docs");
        fs::create_dir_all(home.join(".mark")).expect("home");
        fs::create_dir_all(docs.join("chapters")).expect("chapters");

        let overview = docs.join("overview.md");
        let intro = docs.join("chapters/intro.md");
        fs::write(&overview, "[Intro](chapters/intro.md)\n# Overview\n").expect("overview");
        fs::write(&intro, "# Intro\n").expect("intro");

        let output = run_mark(
            &home,
            &docs,
            &["--recursive", "--no-open", overview.to_str().expect("path")],
        );
        assert!(
            output.status.success(),
            "stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("→ rendered:"), "{stdout}");

        let rendered = rendered_path(&stdout);
        let html = fs::read_to_string(&rendered).expect("rendered html");
        assert!(html.contains("id=\"mark-sidebar\""), "{html}");
        assert!(html.contains("aria-label=\"Rendered file tree\""), "{html}");
        assert!(html.contains("data-theme-option=\"system\""), "{html}");
        assert!(html.contains("mark-theme-option-icon"), "{html}");
        assert!(html.contains("lucide lucide-monitor h-3.5 w-3.5"), "{html}");
        assert!(html.contains(">System</span>"), "{html}");
        assert!(!html.contains("#/overview.html"), "{html}");
        assert!(!html.contains("&lt;put rendered html here&gt;"), "{html}");
        assert!(!html.contains("href=\"chapters/intro.md\""), "{html}");
        assert!(html.contains("intro.html"), "{html}");
        assert!(rendered
            .parent()
            .expect("run dir")
            .join("chapters/intro.html")
            .exists());
    }
}
