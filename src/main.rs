use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use mark::{
    browser, cleanup,
    cli::{Commands, ConfigAction},
    config::{AppConfig, Theme},
    render, storage,
};

fn main() -> Result<()> {
    let args = mark::cli::Cli::parse();

    // Handle the `completions` subcommand before anything else.
    if let Some(Commands::Completions { shell }) = args.command {
        let mut cmd = mark::cli::Cli::command();
        generate(shell, &mut cmd, "mark", &mut std::io::stdout());
        return Ok(());
    }

    // Handle `config` subcommands.
    if let Some(Commands::Config { action }) = args.command {
        let paths = storage::AppPaths::resolve()?;
        match action {
            ConfigAction::SetTheme { theme } => {
                let mut cfg = AppConfig::load(&paths.config)?;
                cfg.theme = theme;
                cfg.save(&paths.config)?;
                println!("Theme set to '{theme}'.");
            }
        }
        return Ok(());
    }

    // Without a subcommand, replicate the old ArgGroup semantics:
    // exactly one of FILE or --cleanup must be provided.
    if args.file.is_some() && args.cleanup {
        let mut cmd = mark::cli::Cli::command();
        cmd.error(
            clap::error::ErrorKind::ArgumentConflict,
            "FILE and --cleanup cannot be used together",
        )
        .exit();
    }
    if args.file.is_none() && !args.cleanup {
        let mut cmd = mark::cli::Cli::command();
        cmd.error(
            clap::error::ErrorKind::MissingRequiredArgument,
            "either FILE or --cleanup is required",
        )
        .exit();
    }

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

    // Resolve theme: CLI override > persisted config > default light.
    let cfg = AppConfig::load(&paths.config)?;
    let theme: Theme = args.theme.unwrap_or(cfg.theme);

    let html = render::render_markdown(&markdown, title, theme);

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
