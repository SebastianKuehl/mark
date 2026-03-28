use anyhow::Result;
use clap::Parser;
use mark::{render, storage};

fn main() -> Result<()> {
    let args = mark::cli::Cli::parse();

    let paths = storage::AppPaths::resolve()?;

    if args.cleanup {
        // Cleanup-only mode (full implementation in Milestone 3).
        println!("Cleanup mode is not yet implemented (Milestone 3).");
        return Ok(());
    }

    let file = args
        .file
        .expect("file is required when not using --cleanup");

    if !file.exists() {
        anyhow::bail!("Input file not found: {}", file.display());
    }

    let markdown = std::fs::read_to_string(&file)
        .map_err(|e| anyhow::anyhow!("Failed to read {}: {e}", file.display()))?;

    let title = file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let html = render::render_markdown(&markdown, title);

    paths.ensure_rendered_dir()?;
    let out_name = storage::output_filename(&file);
    let out_path = storage::write_rendered(&paths.rendered, &out_name, &html)?;

    println!("Rendered: {}", out_path.display());

    if !args.no_open {
        // Browser opening is implemented in Milestone 3.
        println!("Note: browser opening will be available in Milestone 3.");
    }

    Ok(())
}
