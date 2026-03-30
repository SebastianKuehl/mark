/// Tests for shell completion generation (Milestone 6).
///
/// Each test verifies that:
/// 1. The completion script for a given shell is non-empty.
/// 2. The output contains the binary name, confirming it is a real script.
#[cfg(test)]
mod completion_generation {
    use clap::CommandFactory;
    use clap_complete::Shell;
    use mark::cli::Cli;
    use mark::completions;
    use std::process::Command;

    fn generate_completions(shell: Shell) -> String {
        completions::render(shell)
    }

    fn run_bash_completion(script: &str, words: &[&str]) -> Vec<String> {
        let output = Command::new("bash")
            .arg("-lc")
            .arg(
                r#"eval "$1"
shift
COMP_WORDS=("$@")
COMP_CWORD=$((${#COMP_WORDS[@]} - 1))
prev=""
if (( COMP_CWORD > 0 )); then
    prev="${COMP_WORDS[COMP_CWORD-1]}"
fi
_mark mark "${COMP_WORDS[COMP_CWORD]}" "${prev}"
printf '%s\n' "${COMPREPLY[@]}""#,
            )
            .arg("bash")
            .arg(script)
            .args(words)
            .output()
            .expect("bash completion probe should run");
        assert!(
            output.status.success(),
            "bash completion probe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("bash completion output should be valid UTF-8")
            .lines()
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn bash_completions_non_empty() {
        let script = generate_completions(Shell::Bash);
        assert!(
            !script.is_empty(),
            "bash completion script must not be empty"
        );
        assert!(
            script.contains("mark"),
            "bash completion script should reference 'mark'"
        );
    }

    #[test]
    fn bash_completion_keeps_root_subcommands_before_file() {
        let script = generate_completions(Shell::Bash);
        let completions = run_bash_completion(&script, &["mark", ""]);
        assert!(
            completions.iter().any(|item| item == "config"),
            "root completions should still include subcommands before FILE"
        );
        assert!(
            completions.iter().any(|item| item == "cleanup-home"),
            "root completions should still include cleanup-home before FILE"
        );
    }

    #[test]
    fn bash_completion_hides_root_subcommands_after_file() {
        let script = generate_completions(Shell::Bash);
        let completions = run_bash_completion(&script, &["mark", "some-file.md", ""]);
        assert!(
            !completions.iter().any(|item| item == "config"),
            "config should not be suggested after FILE is already present"
        );
        assert!(
            !completions.iter().any(|item| item == "cleanup-home"),
            "cleanup-home should not be suggested after FILE is already present"
        );
        assert!(
            completions.iter().any(|item| item == "--single"),
            "valid top-level flags should still be suggested after FILE"
        );
    }

    #[test]
    fn zsh_completions_non_empty() {
        let script = generate_completions(Shell::Zsh);
        assert!(
            !script.is_empty(),
            "zsh completion script must not be empty"
        );
        assert!(
            script.contains("mark"),
            "zsh completion script should reference 'mark'"
        );
    }

    #[test]
    fn fish_completions_non_empty() {
        let script = generate_completions(Shell::Fish);
        assert!(
            !script.is_empty(),
            "fish completion script must not be empty"
        );
        assert!(
            script.contains("mark"),
            "fish completion script should reference 'mark'"
        );
    }

    #[test]
    fn powershell_completions_non_empty() {
        let script = generate_completions(Shell::PowerShell);
        assert!(
            !script.is_empty(),
            "powershell completion script must not be empty"
        );
        assert!(
            script.contains("mark"),
            "powershell completion script should reference 'mark'"
        );
    }

    /// Verify the file argument carries a FilePath value hint so completion
    /// scripts know to complete it as a path rather than an arbitrary string.
    #[test]
    fn file_arg_has_filepath_hint() {
        let cmd = Cli::command();
        let file_arg = cmd
            .get_arguments()
            .find(|a| a.get_id() == "file")
            .expect("Cli must have a 'file' argument");
        assert_eq!(
            file_arg.get_value_hint(),
            clap::ValueHint::FilePath,
            "'file' argument must use ValueHint::FilePath for shell completions"
        );
    }

    #[test]
    fn help_describes_current_directory_fallback_when_file_is_omitted() {
        let mut cmd = Cli::command();
        let mut help = Vec::new();
        cmd.write_long_help(&mut help)
            .expect("long help should render");
        let help = String::from_utf8(help).expect("help should be valid utf-8");
        assert!(
            help.contains(
                "When FILE is omitted, mark discovers Markdown files in the current directory"
            ),
            "{help}"
        );
    }
}
