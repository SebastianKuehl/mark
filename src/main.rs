use anyhow::Result;
use clap::Parser;

mod browser;
mod cleanup;
mod cli;
mod error;
mod render;
mod storage;

fn main() -> Result<()> {
    let args = cli::Cli::parse();

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

    paths.ensure_rendered_dir()?;
    let out_name = storage::output_filename(&file);
    let out_path = paths.rendered.join(&out_name);

    println!("Output: {}", out_path.display());

    // Rendering, browser opening, and cleanup are implemented in Milestone 2 & 3.
    println!("Rendering and browser opening will be available in Milestone 2/3.");

    Ok(())
}
