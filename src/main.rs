use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use mark::{
    browser, cache, cleanup, cleanup_home,
    cli::{Commands, ConfigAction},
    config::{AppConfig, Theme},
    render, storage,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = mark::cli::Cli::parse();

    if args.version {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

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

    // Handle `cleanup-home` subcommand.
    if let Some(Commands::CleanupHome { yes }) = args.command {
        let target = cleanup_home::resolve_app_dir()?;
        cleanup_home::validate_target(&target)?;

        if !target.exists() {
            println!("Nothing to do: '{}' does not exist.", target.display());
            return Ok(());
        }

        if !yes {
            eprint!(
                "This will permanently delete '{}' and ALL its contents.\n\
                 Type 'yes' to confirm: ",
                target.display()
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim() != "yes" {
                println!("Aborted.");
                return Ok(());
            }
        }

        cleanup_home::delete_app_dir(&target)?;
        println!("Deleted '{}'.", target.display());
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
        cleanup::prune_render_cache(&paths.render_cache);
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

    // Resolve theme: CLI override > persisted config > default light.
    let cfg = AppConfig::load(&paths.config)?;
    let theme: Theme = args.theme.unwrap_or(cfg.theme);

    // ── Phase 1: BFS discovery of all transitively reachable .md files ────────
    //
    // We canonicalize paths so that `./a.md` and `a.md` are treated as the
    // same file, and so circular references (A → B → A) terminate safely.

    let entry_canonical = std::fs::canonicalize(&file)
        .map_err(|e| anyhow::anyhow!("Failed to canonicalize {}: {e}", file.display()))?;

    // ── Render cache: check if re-render is needed ────────────────────────────
    //
    // We load the cache here so it's available both for the early-exit check
    // and for the post-render update at the bottom of main.
    let mut render_cache = cache::RenderCache::load(paths.render_cache.clone());

    // Get the entry file's current mtime as Unix seconds.
    let entry_mtime_secs: Option<u64> = std::fs::metadata(&entry_canonical)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs())
        });

    // Only consult the cache when --no-open is NOT set (non-interactive re-renders
    // should always proceed without prompting).
    if !args.no_open {
        if let (Some(mtime), Some(cached)) = (entry_mtime_secs, render_cache.get(&entry_canonical))
        {
            if cached.source_mtime_secs == mtime && cached.rendered_html.exists() {
                eprint!(
                    "Already rendered: {}\nRe-render? [y/N]: ",
                    cached.rendered_html.display()
                );
                let mut line = String::new();
                std::io::stdin().read_line(&mut line)?;
                if !matches!(line.trim().to_ascii_lowercase().as_str(), "y") {
                    // Open the existing rendered file and exit — no re-render.
                    if let Err(e) = browser::open_browser(&cached.rendered_html) {
                        eprintln!("Warning: {e}");
                    }
                    return Ok(());
                }
                // User answered "y" — fall through to full render below.
            }
        }
    }

    let mut visited: HashSet<PathBuf> = HashSet::new();
    visited.insert(entry_canonical.clone());

    // Ordered list of canonical paths: entry first, then linked files in BFS order.
    let mut ordered: Vec<PathBuf> = vec![entry_canonical.clone()];

    // Cache markdown content keyed by canonical path.
    let mut content_cache: HashMap<PathBuf, String> = HashMap::new();
    content_cache.insert(entry_canonical.clone(), markdown.clone());

    // Track the BFS discovery parent of each file (for breadcrumb construction).
    let mut parent_map: HashMap<PathBuf, PathBuf> = HashMap::new();

    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_back(entry_canonical.clone());

    while let Some(current) = queue.pop_front() {
        let content = content_cache.get(&current).cloned().unwrap_or_default();
        let source_dir = current
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));

        let links = render::extract_local_md_links(&content, source_dir);
        for (_base, canonical) in links {
            if visited.contains(&canonical) {
                continue;
            }
            visited.insert(canonical.clone());
            match std::fs::read_to_string(&canonical) {
                Ok(md) => {
                    content_cache.insert(canonical.clone(), md);
                    parent_map.insert(canonical.clone(), current.clone());
                    ordered.push(canonical.clone());
                    queue.push_back(canonical);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: could not read linked file {}: {e}",
                        canonical.display()
                    );
                }
            }
        }
    }

    // ── Phase 2: assign output filenames to every file up-front ───────────────
    //
    // We generate names before rendering so that each file's link_map can
    // reference the final HTML paths of its targets.

    let mut output_name_map: HashMap<PathBuf, String> = HashMap::new();
    for canonical in &ordered {
        output_name_map.insert(canonical.clone(), storage::output_filename(canonical));
    }

    // ── Phase 3: render + write each file with link rewriting ─────────────────

    paths.ensure_rendered_dir()?;

    // Clean up stale rendered files before writing the new ones.
    match cleanup::cleanup_old_files(&paths.rendered) {
        Ok(n) if n > 0 => println!("Cleaned up {n} old rendered file(s)."),
        Ok(_) => {}
        Err(e) => eprintln!("Warning: cleanup failed: {e}"),
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut entry_out_path: Option<PathBuf> = None;

    // Build the full sidebar list once: (display_name, rendered_html_path, is_current=false).
    // We'll flip is_current per file during rendering.
    let all_files_base: Vec<(String, PathBuf)> = ordered
        .iter()
        .map(|canonical| {
            let name = canonical
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output")
                .to_string();
            let out_name = output_name_map.get(canonical).expect("name was assigned");
            (name, paths.rendered.join(out_name))
        })
        .collect();

    for (idx, canonical) in ordered.iter().enumerate() {
        let content = content_cache.get(canonical).cloned().unwrap_or_default();
        let stem = canonical
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let source_dir = canonical
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));

        // Build a rewrite map: base_target → absolute rendered HTML path.
        let links = render::extract_local_md_links(&content, source_dir);
        let mut link_map: HashMap<String, PathBuf> = HashMap::new();
        for (base, target_canonical) in links {
            if let Some(html_name) = output_name_map.get(&target_canonical) {
                link_map.insert(base, paths.rendered.join(html_name));
            }
        }

        // Copy non-Markdown asset files and add them to the rewrite map.
        let asset_links = render::extract_local_asset_links(&content, source_dir);
        for (original_url, asset_canonical) in asset_links {
            let file_name = match asset_canonical.file_name() {
                Some(n) => n.to_owned(),
                None => continue,
            };
            let dest = paths.rendered.join(&file_name);
            if !dest.exists() {
                if let Err(e) = std::fs::copy(&asset_canonical, &dest) {
                    eprintln!(
                        "Warning: could not copy asset {}: {e}",
                        asset_canonical.display()
                    );
                    continue;
                }
            }
            link_map.insert(original_url, dest);
        }

        // Build the sidebar list with is_current flagged for this file.
        let all_files: Vec<(String, PathBuf, bool)> = all_files_base
            .iter()
            .enumerate()
            .map(|(i, (name, path))| (name.clone(), path.clone(), i == idx))
            .collect();

        // Build the breadcrumb: walk parent_map from this file up to entry,
        // then reverse so it reads entry → … → parent.
        let breadcrumb: Vec<(String, PathBuf)> = if idx == 0 {
            vec![]
        } else {
            let mut chain: Vec<(String, PathBuf)> = Vec::new();
            let mut cursor = canonical.clone();
            while let Some(parent_canonical) = parent_map.get(&cursor) {
                let parent_name = parent_canonical
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?")
                    .to_string();
                let parent_out_name = output_name_map
                    .get(parent_canonical)
                    .expect("parent name was assigned");
                chain.push((parent_name, paths.rendered.join(parent_out_name)));
                cursor = parent_canonical.clone();
            }
            chain.reverse();
            chain
        };

        let html = render::render_markdown_rewriting_links(
            &content,
            stem,
            theme,
            &link_map,
            &breadcrumb,
            &all_files,
        );
        let out_name = output_name_map.get(canonical).expect("name was assigned");
        let out_path = storage::write_rendered(&paths.rendered, out_name, &html)?;

        if idx == 0 {
            entry_out_path = Some(out_path);
        } else {
            // Print a summary line for each additional file.
            let display_src = canonical
                .strip_prefix(&cwd)
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| canonical.display().to_string());
            println!("  → rendered: {display_src} → {}", out_path.display());
        }
    }

    let out_path = entry_out_path.expect("entry file was rendered");
    println!("Rendered: {}", out_path.display());

    // ── Update render cache with the new entry-point output ──────────────────
    if let Some(mtime) = entry_mtime_secs {
        render_cache.set(
            &entry_canonical,
            cache::CacheEntry {
                rendered_html: out_path.clone(),
                source_mtime_secs: mtime,
            },
        );
        render_cache.save();
    }

    if !args.no_open {
        if let Err(e) = browser::open_browser(&out_path) {
            eprintln!("Warning: {e}");
        }
    }

    Ok(())
}
