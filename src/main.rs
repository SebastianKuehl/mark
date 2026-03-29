use anyhow::Result;
use clap::{CommandFactory, Parser};
use mark::{
    browser, cache, cleanup, cleanup_home,
    cli::{Commands, ConfigAction},
    completions,
    config::{AppConfig, RenderMode, SidebarVisibility, Theme},
    render, storage,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

fn stable_path_hash(path: &Path) -> u32 {
    path.as_os_str()
        .as_encoded_bytes()
        .iter()
        .fold(0u32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u32))
}

fn relative_path_in_run(
    entry_dir: &Path,
    file_canonical: &Path,
    extension: Option<&str>,
) -> PathBuf {
    let relative = file_canonical
        .strip_prefix(entry_dir)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| {
            let stem = file_canonical
                .file_stem()
                .and_then(|s| s.to_str())
                .or_else(|| file_canonical.file_name().and_then(|s| s.to_str()))
                .unwrap_or("output");
            let fallback_extension = extension
                .map(ToOwned::to_owned)
                .or_else(|| {
                    file_canonical
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(ToOwned::to_owned)
                })
                .unwrap_or_default();
            let file_name = if fallback_extension.is_empty() {
                format!("{stem}-{:08x}", stable_path_hash(file_canonical))
            } else {
                format!(
                    "{stem}-{:08x}.{}",
                    stable_path_hash(file_canonical),
                    fallback_extension
                )
            };
            PathBuf::from("_external").join(file_name)
        });

    match extension {
        Some(ext) => relative.with_extension(ext),
        None => relative,
    }
}

fn output_path_for_run(run_dir: &Path, entry_dir: &Path, file_canonical: &Path) -> PathBuf {
    run_dir.join(relative_path_in_run(
        entry_dir,
        file_canonical,
        Some("html"),
    ))
}

fn asset_output_path_for_run(
    run_dir: &Path,
    entry_dir: &Path,
    asset_canonical: &Path,
    reserved_paths: &HashSet<PathBuf>,
) -> PathBuf {
    let relative = relative_path_in_run(entry_dir, asset_canonical, None);
    let dest = run_dir.join(&relative);
    if !reserved_paths.contains(&dest) {
        return dest;
    }

    let stem = asset_canonical
        .file_stem()
        .and_then(|s| s.to_str())
        .or_else(|| asset_canonical.file_name().and_then(|s| s.to_str()))
        .unwrap_or("asset");
    let ext = asset_canonical
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let collision_name = if ext.is_empty() {
        format!("{stem}-{:08x}", stable_path_hash(asset_canonical))
    } else {
        format!("{stem}-{:08x}.{ext}", stable_path_hash(asset_canonical))
    };
    run_dir.join(PathBuf::from("_assets").join(collision_name))
}

fn copy_asset_into_run(asset_canonical: &Path, dest: &Path) -> Result<()> {
    use std::io;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut src = std::fs::File::open(asset_canonical)?;
    let mut out = std::fs::File::create(dest)?;
    io::copy(&mut src, &mut out)?;
    Ok(())
}

fn resolve_render_mode(args: &mark::cli::Cli, cfg: &AppConfig) -> RenderMode {
    if args.single {
        RenderMode::Single
    } else if args.recursive {
        RenderMode::Recursive
    } else {
        cfg.render_mode
    }
}

fn format_skipped_links_note(skipped_links: &[String]) -> Option<String> {
    if skipped_links.is_empty() {
        None
    } else {
        Some(format!(
            "Note: single mode skipped local Markdown links: {}",
            skipped_links.join(", ")
        ))
    }
}

fn main() -> Result<()> {
    let args = mark::cli::Cli::parse();

    if args.version {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Handle the `completions` subcommand before anything else.
    if let Some(Commands::Completions { shell }) = args.command {
        print!("{}", completions::render(shell));
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
            ConfigAction::SetRenderMode { mode } => {
                let mut cfg = AppConfig::load(&paths.config)?;
                cfg.render_mode = mode;
                cfg.save(&paths.config)?;
                println!("Render mode set to '{mode}'.");
            }
            ConfigAction::SetSidebar { sidebar } => {
                let mut cfg = AppConfig::load(&paths.config)?;
                cfg.sidebar = sidebar;
                cfg.save(&paths.config)?;
                println!("Sidebar default set to '{sidebar}'.");
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
        println!("Cleanup complete: {deleted} run dir(s) removed.");
        return Ok(());
    }

    // Resolve output settings: CLI override > persisted config > defaults.
    let cfg = AppConfig::load(&paths.config)?;
    let render_mode = resolve_render_mode(&args, &cfg);
    let sidebar = cfg.sidebar;
    let theme: Theme = args.theme.unwrap_or(cfg.theme);

    let file = args
        .file
        .expect("file is required when not using --cleanup");

    if !file.exists() {
        anyhow::bail!("Input file not found: {}", file.display());
    }

    let markdown = std::fs::read_to_string(&file)
        .map_err(|e| anyhow::anyhow!("Failed to read {}: {e}", file.display()))?;

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
            let cached_entry_html = output_path_for_run(
                &cached.rendered_html,
                entry_canonical.parent().unwrap_or_else(|| Path::new(".")),
                &entry_canonical,
            );
            if render_mode == RenderMode::Single
                && cached.source_mtime_secs == mtime
                && cached.rendered_html.exists()
                && cached_entry_html.exists()
                && cache::RenderCache::matches_options(cached, theme, render_mode, sidebar)
            {
                eprint!(
                    "Already rendered: {}\nRe-render? [y/N]: ",
                    cached_entry_html.display()
                );
                let mut line = String::new();
                std::io::stdin().read_line(&mut line)?;
                if !matches!(line.trim().to_ascii_lowercase().as_str(), "y") {
                    // Open the existing rendered file and exit — no re-render.
                    if let Err(e) = browser::open_browser(&cached_entry_html) {
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

    let skipped_md_links: Vec<String> = if render_mode == RenderMode::Single {
        render::extract_local_md_links(
            &markdown,
            entry_canonical.parent().unwrap_or_else(|| Path::new(".")),
        )
        .into_iter()
        .map(|(base, _)| base)
        .collect()
    } else {
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

        Vec::new()
    };

    // ── Phase 2: assign output paths inside a per-run directory up-front ──────
    //
    // We generate paths before rendering so that each file's link_map can
    // reference the final HTML paths of its targets.

    let entry_dir = entry_canonical
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let run_dir = storage::make_run_dir(&paths.rendered, &entry_canonical)?;
    let mut output_path_map: HashMap<PathBuf, PathBuf> = HashMap::new();
    for canonical in &ordered {
        let output_path = output_path_for_run(&run_dir, &entry_dir, canonical);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        output_path_map.insert(canonical.clone(), output_path);
    }
    let reserved_output_paths: HashSet<PathBuf> = output_path_map.values().cloned().collect();

    // ── Phase 3: render + write each file with link rewriting ─────────────────

    paths.ensure_rendered_dir()?;

    // Clean up stale rendered runs before writing the new one.
    match cleanup::cleanup_old_files(&paths.rendered) {
        Ok(n) if n > 0 => println!("Cleaned up {n} old rendered run(s)."),
        Ok(_) => {}
        Err(e) => eprintln!("Warning: cleanup failed: {e}"),
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut entry_out_path: Option<PathBuf> = None;

    // Build the full sidebar list once: (display_name, rendered_html_path, is_current=false).
    // We'll flip is_current per file during rendering.
    let all_files_base: Vec<(String, PathBuf)> = if render_mode == RenderMode::Single {
        Vec::new()
    } else {
        ordered
            .iter()
            .map(|canonical| {
                let name = canonical
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output")
                    .to_string();
                let out_path = output_path_map
                    .get(canonical)
                    .expect("output path was assigned");
                (name, out_path.clone())
            })
            .collect()
    };

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
        let mut link_map: HashMap<String, PathBuf> = HashMap::new();
        if render_mode == RenderMode::Recursive {
            let links = render::extract_local_md_links(&content, source_dir);
            for (base, target_canonical) in links {
                if let Some(output_path) = output_path_map.get(&target_canonical) {
                    link_map.insert(base, output_path.clone());
                }
            }
        }

        // Copy non-Markdown asset files and add them to the rewrite map.
        let asset_links = render::extract_local_asset_links(&content, source_dir);
        for (original_url, asset_canonical) in asset_links {
            let dest = asset_output_path_for_run(
                &run_dir,
                &entry_dir,
                &asset_canonical,
                &reserved_output_paths,
            );
            if !dest.exists() {
                if let Err(e) = copy_asset_into_run(&asset_canonical, &dest) {
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
        let breadcrumb: Vec<(String, PathBuf)> = if idx == 0 || render_mode == RenderMode::Single {
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
                let parent_out_path = output_path_map
                    .get(parent_canonical)
                    .expect("parent output path was assigned");
                chain.push((parent_name, parent_out_path.clone()));
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
            render::RenderChrome {
                breadcrumb: &breadcrumb,
                all_files: &all_files,
                run_dir: &run_dir,
                sidebar_visible: sidebar == SidebarVisibility::Visible,
            },
        );
        let out_path = output_path_map
            .get(canonical)
            .expect("output path was assigned")
            .clone();
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&out_path, html.as_bytes())?;

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
    if let Some(note) = format_skipped_links_note(&skipped_md_links) {
        println!("{note}");
    }

    // ── Update render cache with the new entry-point output ──────────────────
    if let Some(mtime) = entry_mtime_secs {
        render_cache.set(
            &entry_canonical,
            cache::CacheEntry {
                rendered_html: run_dir.clone(),
                source_mtime_secs: mtime,
                theme: Some(theme),
                render_mode: Some(render_mode),
                sidebar: Some(sidebar),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn output_path_for_run_preserves_relative_hierarchy() {
        let run_dir = PathBuf::from("/rendered/overview-123-abcdef12");
        let entry_dir = Path::new("/docs");
        let chapter = Path::new("/docs/chapters/api/endpoints.md");
        let output = output_path_for_run(&run_dir, entry_dir, chapter);
        assert_eq!(
            output,
            PathBuf::from("/rendered/overview-123-abcdef12/chapters/api/endpoints.html")
        );
    }

    #[test]
    fn output_path_for_run_falls_back_to_file_name() {
        let run_dir = PathBuf::from("/rendered/overview-123-abcdef12");
        let entry_dir = Path::new("/docs");
        let outside = Path::new("/other/shared.md");
        let output = output_path_for_run(&run_dir, entry_dir, outside);
        assert_eq!(
            output,
            PathBuf::from(format!(
                "/rendered/overview-123-abcdef12/_external/shared-{:08x}.html",
                stable_path_hash(outside)
            ))
        );
    }

    #[test]
    fn output_path_for_run_disambiguates_external_name_collisions() {
        let run_dir = PathBuf::from("/rendered/overview-123-abcdef12");
        let entry_dir = Path::new("/docs");
        let a = output_path_for_run(&run_dir, entry_dir, Path::new("/tmp/a/readme.md"));
        let b = output_path_for_run(&run_dir, entry_dir, Path::new("/tmp/b/readme.md"));
        assert_ne!(a, b, "external files with same stem must not collide");
    }

    #[test]
    fn relative_path_in_run_disambiguates_external_asset_collisions() {
        let entry_dir = Path::new("/docs");
        let a = relative_path_in_run(entry_dir, Path::new("/tmp/a/logo.png"), None);
        let b = relative_path_in_run(entry_dir, Path::new("/tmp/b/logo.png"), None);
        assert_ne!(a, b, "external assets with same stem must not collide");
        assert!(a.starts_with("_external"));
        assert!(b.starts_with("_external"));
    }

    #[test]
    fn copy_asset_into_run_refreshes_mtime() {
        let dir = tempfile::tempdir().expect("tempdir");
        let source = dir.path().join("logo.png");
        let dest = dir.path().join("run/assets/logo.png");
        std::fs::write(&source, b"png").expect("write source");

        let status = Command::new("touch")
            .arg("-t")
            .arg("200001010101")
            .arg(&source)
            .status()
            .expect("run touch");
        assert!(status.success(), "touch should succeed");

        copy_asset_into_run(&source, &dest).expect("copy asset");
        let source_mtime = std::fs::metadata(&source)
            .expect("source metadata")
            .modified()
            .expect("source mtime");
        let dest_mtime = std::fs::metadata(&dest)
            .expect("dest metadata")
            .modified()
            .expect("dest mtime");
        assert!(
            dest_mtime > source_mtime,
            "copied asset should get a fresh mtime so cleanup doesn't purge new runs"
        );
    }

    #[test]
    fn asset_output_path_for_run_avoids_rendered_html_collision() {
        let run_dir = PathBuf::from("/rendered/overview-123-abcdef12");
        let entry_dir = Path::new("/docs");
        let reserved = HashSet::from([run_dir.join("chapters/intro.html")]);
        let asset = Path::new("/docs/chapters/intro.html");
        let dest = asset_output_path_for_run(&run_dir, entry_dir, asset, &reserved);
        assert_ne!(dest, run_dir.join("chapters/intro.html"));
        assert!(dest.starts_with(run_dir.join("_assets")));
    }

    #[test]
    fn resolve_render_mode_prefers_cli_over_config() {
        let cfg = AppConfig {
            theme: Theme::System,
            render_mode: RenderMode::Recursive,
            sidebar: SidebarVisibility::Hidden,
        };
        let cli = mark::cli::Cli::parse_from(["mark", "--single", "notes.md"]);
        assert_eq!(resolve_render_mode(&cli, &cfg), RenderMode::Single);
    }

    #[test]
    fn resolve_render_mode_falls_back_to_config() {
        let cfg = AppConfig {
            theme: Theme::System,
            render_mode: RenderMode::Single,
            sidebar: SidebarVisibility::Hidden,
        };
        let cli = mark::cli::Cli::parse_from(["mark", "notes.md"]);
        assert_eq!(resolve_render_mode(&cli, &cfg), RenderMode::Single);
    }

    #[test]
    fn skipped_links_note_is_omitted_when_empty() {
        assert_eq!(format_skipped_links_note(&[]), None);
    }

    #[test]
    fn skipped_links_note_lists_links() {
        let note =
            format_skipped_links_note(&["guide.md".to_string(), "nested/api.md".to_string()])
                .expect("note");
        assert!(note.contains("guide.md"));
        assert!(note.contains("nested/api.md"));
    }
}
