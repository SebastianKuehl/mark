use crate::config::Theme;
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

    /// Override the render theme for this invocation (light or dark)
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
    /// Set the persistent render theme (light or dark).
    ///
    /// Example: mark config set-theme dark
    SetTheme {
        /// Theme to use: light or dark
        theme: Theme,
    },
}
