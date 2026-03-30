#[cfg(test)]
mod wipe_command {
    use std::fs;
    use std::path::Path;
    use std::process::{Command, Output};

    fn run_mark(home: &Path, cwd: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mark"))
            .args(args)
            .current_dir(cwd)
            .env("HOME", home)
            .output()
            .expect("run mark")
    }

    fn touch_old(path: &Path) {
        let status = Command::new("touch")
            .arg("-t")
            .arg("202001010101")
            .arg(path)
            .status()
            .expect("touch");
        assert!(
            status.success(),
            "touch should succeed for {}",
            path.display()
        );
    }

    #[test]
    fn root_cleanup_flag_is_rejected() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        fs::create_dir_all(&home).expect("home");

        let output = run_mark(&home, sandbox.path(), &["--cleanup"]);

        assert!(!output.status.success(), "{output:?}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("--cleanup"), "{stderr}");
    }

    #[test]
    fn wipe_config_deletes_only_config_file() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        let mark_dir = home.join(".mark");
        fs::create_dir_all(mark_dir.join("rendered/run-1")).expect("rendered");
        fs::write(mark_dir.join("config.toml"), "theme = \"dark\"\n").expect("config");
        fs::write(mark_dir.join("render-cache.toml"), "").expect("cache");
        fs::write(mark_dir.join("rendered/run-1/out.html"), "<p>ok</p>").expect("html");

        let output = run_mark(&home, sandbox.path(), &["wipe", "--config"]);

        assert!(output.status.success(), "{output:?}");
        assert!(!mark_dir.join("config.toml").exists());
        assert!(mark_dir.join("render-cache.toml").exists());
        assert!(mark_dir.join("rendered/run-1/out.html").exists());
    }

    #[test]
    fn wipe_renders_deletes_only_rendered_output() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        let mark_dir = home.join(".mark");
        fs::create_dir_all(mark_dir.join("rendered/run-1")).expect("rendered");
        fs::write(mark_dir.join("config.toml"), "theme = \"dark\"\n").expect("config");
        fs::write(mark_dir.join("render-cache.toml"), "").expect("cache");
        fs::write(mark_dir.join("rendered/run-1/out.html"), "<p>ok</p>").expect("html");

        let output = run_mark(&home, sandbox.path(), &["wipe", "--renders"]);

        assert!(output.status.success(), "{output:?}");
        assert!(mark_dir.join("config.toml").exists());
        assert!(mark_dir.join("render-cache.toml").exists());
        assert!(!mark_dir.join("rendered").exists());
    }

    #[test]
    fn wipe_old_renders_removes_only_stale_runs() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        let mark_dir = home.join(".mark");
        let stale_run = mark_dir.join("rendered/stale-run");
        let fresh_run = mark_dir.join("rendered/fresh-run");
        fs::create_dir_all(&stale_run).expect("stale");
        fs::create_dir_all(&fresh_run).expect("fresh");
        let stale_file = stale_run.join("out.html");
        let fresh_file = fresh_run.join("out.html");
        fs::write(&stale_file, "<p>stale</p>").expect("stale html");
        fs::write(&fresh_file, "<p>fresh</p>").expect("fresh html");
        touch_old(&stale_run);
        touch_old(&stale_file);

        let output = run_mark(&home, sandbox.path(), &["wipe", "--old-renders"]);

        assert!(output.status.success(), "{output:?}");
        assert!(!stale_run.exists(), "stale run should be removed");
        assert!(fresh_run.exists(), "fresh run should remain");
    }

    #[test]
    fn wipe_all_deletes_mark_directory_with_yes() {
        let sandbox = tempfile::tempdir().expect("tempdir");
        let home = sandbox.path().join("home");
        let mark_dir = home.join(".mark");
        fs::create_dir_all(mark_dir.join("rendered")).expect("rendered");
        fs::write(mark_dir.join("config.toml"), "theme = \"light\"\n").expect("config");

        let output = run_mark(&home, sandbox.path(), &["wipe", "--all", "--yes"]);

        assert!(output.status.success(), "{output:?}");
        assert!(!mark_dir.exists(), ".mark should be gone");
    }
}
