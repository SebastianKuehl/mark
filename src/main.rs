use anyhow::Result;
use clap::Parser;
use mark::{browser, cleanup, render, storage};

fn main() -> Result<()> {
    let args = mark::cli::Cli::parse();

    let paths = storage::AppPaths::resolve()?;

    if args.cleanup {
        paths.ensure_rendered_dir()?;
        let deleted = cleanup::cleanup_old_files(&paths.rendered)?;
        println!("Cleanup complete: {deleted} file(s) removed.");
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

    // Clean up old rendered files before writing the new one.
    match cleanup::cleanup_old_files(&paths.rendered) {
        Ok(n) if n > 0 => println!("Cleaned up {n} old rendered file(s)."),
        Ok(_) => {}
        Err(e) => eprintln!("Warning: cleanup failed: {e}"),
    }

    let out_name = storage::output_filename(&file);
    let out_path = storage::write_rendered(&paths.rendered, &out_name, &html)?;

    println!("Rendered: {}", out_path.display());

    if !args.no_open {
        if let Err(e) = browser::open_browser(&out_path) {
            eprintln!("Warning: {e}");
        }
    }

    Ok(())
}
