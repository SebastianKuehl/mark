use crate::config::{RenderMode, SidebarVisibility, Theme};
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

/// Render a Markdown file to HTML and open it in the default browser.
#[derive(Parser, Debug)]
#[command(name = "mark", version = concat!("v", env!("CARGO_PKG_VERSION")), long_version = concat!("v", env!("CARGO_PKG_VERSION")), about, long_about = None, disable_version_flag = true)]
pub struct Cli {
    /// Markdown file to render
    #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Delete rendered files older than 30 days without rendering anything
    #[arg(long)]
    pub cleanup: bool,

    /// Render without opening the browser
    #[arg(long)]
    pub no_open: bool,

    /// Render only the requested file.
    #[arg(long, short = 's', conflicts_with = "recursive")]
    pub single: bool,

    /// Recursively render linked Markdown files.
    #[arg(long, short = 'r', conflicts_with = "single")]
    pub recursive: bool,

    /// Override the render theme for this invocation (system, light, or dark)
    #[arg(long, value_name = "THEME")]
    pub theme: Option<Theme>,

    /// Print version
    #[arg(long, short = 'V', action = clap::ArgAction::SetTrue)]
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
    /// Remove the entire ~/.mark app directory from the home folder.
    ///
    /// This is a destructive operation that deletes ALL mark data including
    /// rendered files, configuration, and the installed binary.  It is
    /// separate from `mark --cleanup`, which only removes old rendered files.
    ///
    /// A confirmation prompt is shown by default; pass --yes to skip it.
    CleanupHome {
        /// Skip the confirmation prompt (for non-interactive use).
        #[arg(long)]
        yes: bool,
    },
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_and_recursive_flags_conflict() {
        let result = Cli::try_parse_from(["mark", "--single", "--recursive", "notes.md"]);
        assert!(result.is_err(), "single and recursive should conflict");
    }

    #[test]
    fn config_supports_render_mode_and_sidebar_commands() {
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
    }
}
