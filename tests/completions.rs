/// Tests for shell completion generation (Milestone 6).
///
/// Each test verifies that:
/// 1. The completion script for a given shell is non-empty.
/// 2. The output contains the binary name, confirming it is a real script.
#[cfg(test)]
mod completion_generation {
    use clap::CommandFactory;
    use clap_complete::{generate, Shell};
    use mark::cli::Cli;

    fn generate_completions(shell: Shell) -> String {
        let mut cmd = Cli::command();
        let mut buf: Vec<u8> = Vec::new();
        generate(shell, &mut cmd, "mark", &mut buf);
        String::from_utf8(buf).expect("completion output should be valid UTF-8")
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
}
