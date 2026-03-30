use crate::config::{RenderMode, SidebarVisibility, Theme};
use clap::{ArgGroup, Args, Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

/// Render a Markdown file to HTML and open it in the default browser.
#[derive(Parser, Debug)]
#[command(
    name = "mark",
    version = concat!("v", env!("CARGO_PKG_VERSION")),
    long_version = concat!("v", env!("CARGO_PKG_VERSION")),
    about,
    long_about = None,
    after_help = "When FILE is omitted, mark discovers Markdown files in the current directory and opens the first discovered page.",
    disable_version_flag = true,
    override_usage = "mark [OPTIONS] [FILE]\n       mark [OPTIONS] <COMMAND>",
)]
pub struct Cli {
    /// Markdown file or directory to render. When omitted, discover Markdown files in the current directory.
    #[arg(value_name = "FILE", value_hint = clap::ValueHint::AnyPath)]
    pub file: Option<PathBuf>,

    /// Render without opening the browser
    #[arg(long, short = 'n')]
    pub no_open: bool,

    /// Render only the requested file.
    #[arg(long, short = 's', conflicts_with = "recursive")]
    pub single: bool,

    /// Recursively render linked Markdown files.
    #[arg(long, short = 'r', conflicts_with = "single")]
    pub recursive: bool,

    /// Override the render theme for this invocation (system, light, or dark)
    #[arg(long, short = 't', value_name = "THEME")]
    pub theme: Option<Theme>,

    /// Print version
    #[arg(long, short = 'v', action = clap::ArgAction::SetTrue)]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completion scripts and print them to stdout.
    ///
    /// Example: mark completions bash >> ~/.bash_completion
    Completions {
        /// Shell to generate completions for.
        shell: Shell,
    },
    /// Manage persistent mark configuration.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Remove app data from ~/.mark using an explicit wipe mode.
    ///
    /// Exactly one wipe target is required:
    ///   --all         Delete the entire ~/.mark directory
    ///   --config      Delete only ~/.mark/config.toml
    ///   --renders     Delete only ~/.mark/rendered
    ///   --old-renders Delete render output older than 30 days
    ///
    /// A confirmation prompt is shown for --all by default; pass --yes to skip it.
    Wipe(WipeArgs),
    /// Export a Markdown file directly to PDF via headless browser print.
    ///
    /// Example: mark pdf docs/file.md out/file.pdf
    Pdf {
        /// Markdown source file to render and export.
        #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
        source: PathBuf,
        /// Destination PDF path.
        #[arg(value_name = "OUTPUT", value_hint = clap::ValueHint::AnyPath)]
        output: PathBuf,
    },
}

#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("wipe_target")
        .args(["all", "config", "renders", "old_renders"])
        .required(true)
        .multiple(false)
))]
pub struct WipeArgs {
    /// Delete the entire ~/.mark directory.
    #[arg(long)]
    pub all: bool,

    /// Delete only ~/.mark/config.toml.
    #[arg(long)]
    pub config: bool,

    /// Delete only ~/.mark/rendered.
    #[arg(long)]
    pub renders: bool,

    /// Delete only render output older than 30 days.
    #[arg(long = "old-renders")]
    pub old_renders: bool,

    /// Skip the confirmation prompt for destructive wipes.
    #[arg(long)]
    pub yes: bool,
}

/// Config sub-actions.
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Set the persistent render theme (system, light, or dark).
    ///
    /// Example: mark config set-theme system
    SetTheme {
        /// Theme to use: system, light, or dark
        theme: Theme,
    },
    /// Set the persistent render mode.
    ///
    /// Example: mark config set-render-mode single
    SetRenderMode {
        /// Render mode to use by default: single or recursive
        mode: RenderMode,
    },
    /// Set the persistent sidebar visibility.
    ///
    /// Example: mark config set-sidebar hidden
    SetSidebar {
        /// Sidebar visibility to use by default: hidden or visible
        sidebar: SidebarVisibility,
    },
    /// Set persistent reader appearance settings.
    ///
    /// Example:
    /// mark config set-layout --font-size 17 --letter-width 8.5 --letter-radius 14 --sidebar-button-radius 999 --theme-button-radius 999
    SetLayout {
        /// Base font size for the rendered letter sheet, in px.
        #[arg(long, value_name = "PX")]
        font_size: u16,
        /// Maximum letter width, in inches.
        #[arg(long, value_name = "IN")]
        letter_width: f32,
        /// Corner radius for the rendered letter sheet, in px.
        #[arg(long, value_name = "PX")]
        letter_radius: u16,
        /// Corner radius for the sidebar toggle button, in px.
        #[arg(long, value_name = "PX")]
        sidebar_button_radius: u16,
        /// Corner radius for the theme/settings button, in px.
        #[arg(long, value_name = "PX")]
        theme_button_radius: u16,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn single_and_recursive_flags_conflict() {
        let result = Cli::try_parse_from(["mark", "--single", "--recursive", "notes.md"]);
        assert!(result.is_err(), "single and recursive should conflict");
    }

    #[test]
    fn file_parses_as_file_render() {
        let cli = Cli::try_parse_from(["mark", "notes.md"]).unwrap();
        assert_eq!(cli.file, Some(PathBuf::from("notes.md")));
        assert!(cli.command.is_none());
    }

    #[test]
    fn directory_parses_as_render_target() {
        let cli = Cli::try_parse_from(["mark", "docs"]).unwrap();
        assert_eq!(cli.file, Some(PathBuf::from("docs")));
        assert!(cli.command.is_none());
    }

    #[test]
    fn no_file_parses_as_current_directory_render() {
        let cli = Cli::try_parse_from(["mark"]).unwrap();
        assert!(cli.file.is_none());
        assert!(cli.command.is_none());
    }

    #[test]
    fn pdf_subcommand_parses_source_and_output() {
        let cli = Cli::try_parse_from(["mark", "pdf", "docs/file.md", "out/file.pdf"]).unwrap();
        match cli.command {
            Some(Commands::Pdf { source, output }) => {
                assert_eq!(source, PathBuf::from("docs/file.md"));
                assert_eq!(output, PathBuf::from("out/file.pdf"));
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn short_flags_parse() {
        let cli = Cli::try_parse_from(["mark", "-n", "-t", "dark", "notes.md"]).unwrap();
        assert!(cli.no_open);
        assert_eq!(cli.theme, Some(Theme::Dark));

        let cli = Cli::try_parse_from(["mark", "-v"]).unwrap();
        assert!(cli.version);
    }

    #[test]
    fn wipe_requires_exactly_one_target() {
        let err = Cli::try_parse_from(["mark", "wipe"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);

        let err = Cli::try_parse_from(["mark", "wipe", "--config", "--renders"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn wipe_modes_parse() {
        let cli = Cli::try_parse_from(["mark", "wipe", "--old-renders"]).unwrap();
        match cli.command {
            Some(Commands::Wipe(WipeArgs {
                old_renders: true,
                yes: false,
                ..
            })) => {}
            other => panic!("unexpected command: {other:?}"),
        }

        let cli = Cli::try_parse_from(["mark", "wipe", "--all", "--yes"]).unwrap();
        match cli.command {
            Some(Commands::Wipe(WipeArgs {
                all: true,
                yes: true,
                ..
            })) => {}
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn root_cleanup_flag_is_rejected() {
        let err = Cli::try_parse_from(["mark", "--cleanup"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn file_and_command_are_mutually_exclusive() {
        // clap 4 does not support conflicts_with against a subcommand field,
        // so mixed invocations parse successfully at the clap layer.
        // main() performs manual validation and rejects the combination at runtime.
        // Here we confirm that clap does accept the parse (both fields are set),
        // so that the runtime check in main() has something to act on.
        let cli =
            Cli::try_parse_from(["mark", "notes.md", "config", "set-theme", "light"]).unwrap();
        assert!(
            cli.file.is_some() && cli.command.is_some(),
            "clap must populate both fields so main() can detect and reject the combination"
        );
    }

    #[test]
    fn config_supports_render_mode_sidebar_and_layout_commands() {
        let cli = Cli::try_parse_from(["mark", "config", "set-render-mode", "single"]).unwrap();
        match cli.command {
            Some(Commands::Config {
                action: ConfigAction::SetRenderMode { mode },
            }) => assert_eq!(mode, RenderMode::Single),
            other => panic!("unexpected command: {other:?}"),
        }

        let cli = Cli::try_parse_from(["mark", "config", "set-sidebar", "visible"]).unwrap();
        match cli.command {
            Some(Commands::Config {
                action: ConfigAction::SetSidebar { sidebar },
            }) => assert_eq!(sidebar, SidebarVisibility::Visible),
            other => panic!("unexpected command: {other:?}"),
        }

        let cli = Cli::try_parse_from([
            "mark",
            "config",
            "set-layout",
            "--font-size",
            "18",
            "--letter-width",
            "7.75",
            "--letter-radius",
            "20",
            "--sidebar-button-radius",
            "18",
            "--theme-button-radius",
            "14",
        ])
        .unwrap();
        match cli.command {
            Some(Commands::Config {
                action:
                    ConfigAction::SetLayout {
                        font_size,
                        letter_width,
                        letter_radius,
                        sidebar_button_radius,
                        theme_button_radius,
                    },
            }) => {
                assert_eq!(font_size, 18);
                assert!((letter_width - 7.75).abs() < f32::EPSILON);
                assert_eq!(letter_radius, 20);
                assert_eq!(sidebar_button_radius, 18);
                assert_eq!(theme_button_radius, 14);
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn help_lists_wipe_modes_and_hides_legacy_cleanup_surface() {
        let mut cmd = Cli::command();
        let mut wipe = cmd
            .find_subcommand_mut("wipe")
            .expect("wipe subcommand should exist")
            .clone();
        let mut help = Vec::new();
        wipe.write_long_help(&mut help).expect("help");
        let help = String::from_utf8(help).expect("utf8");

        assert!(help.contains("wipe"), "{help}");
        assert!(help.contains("--old-renders"), "{help}");
        assert!(help.contains("--config"), "{help}");
        assert!(!help.contains("cleanup-home"), "{help}");
        assert!(!help.contains("--cleanup"), "{help}");
    }
}
