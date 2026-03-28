use clap::{ArgGroup, Parser};
use std::path::PathBuf;

/// Render a Markdown file to HTML and open it in the default browser.
#[derive(Parser, Debug)]
#[command(name = "mark", version, about, long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["file", "cleanup"])
))]
pub struct Cli {
    /// Markdown file to render
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Delete rendered files older than 30 days without rendering anything
    #[arg(long)]
    pub cleanup: bool,

    /// Render without opening the browser
    #[arg(long)]
    pub no_open: bool,
}
