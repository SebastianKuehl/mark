use std::collections::HashMap;
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Event, Options, Parser, Tag};

use crate::config::{AppearanceConfig, Theme};
use crate::copy_clean::{is_supported_language, strip_full_line_comments};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarNode {
    pub name: String,
    pub path: Option<PathBuf>,
    pub is_current: bool,
    pub children: Vec<SidebarNode>,
}

pub struct RenderChrome<'a> {
    pub breadcrumb: &'a [(String, PathBuf)],
    pub all_files: &'a [(String, PathBuf, bool)],
    pub run_dir: &'a Path,
    pub sidebar_visible: bool,
    pub appearance: AppearanceConfig,
}

/// Render Markdown source to a complete standalone HTML5 document.
///
/// The output is self-contained: no external stylesheets or scripts are
/// referenced. `title` is used as the `<title>` and shown as an `<h1>` in the
/// page header. `theme` controls the colour scheme of the output.
///
/// # Security note
///
/// Raw HTML embedded in Markdown is passed through unchanged (standard
/// pulldown-cmark behaviour). This is intentional: `mark` is designed to
/// render **local files owned by the user**. Do not use it to render
/// untrusted Markdown — the result could contain executable JavaScript. See
/// the README for details.
pub fn render_markdown(markdown: &str, title: &str, theme: Theme) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, opts);
    let mut body = String::new();
    html::push_html(&mut body, parser);
    let body = post_process_code_blocks(&body);

    build_html_document(
        title,
        &body,
        theme,
        RenderChrome {
            breadcrumb: &[],
            all_files: &[],
            run_dir: Path::new(""),
            sidebar_visible: false,
            appearance: AppearanceConfig::default(),
        },
    )
}

fn render_breadcrumb_html(title: &str, breadcrumb: &[(String, PathBuf)]) -> String {
    if breadcrumb.is_empty() {
        return String::new();
    }

    let mut html = String::from(r#"<nav class="mark-breadcrumb" aria-label="Breadcrumb">"#);
    for (name, path) in breadcrumb {
        let href = escape_html(&path.to_string_lossy());
        let display = escape_html(name);
        html.push_str(&format!(
            r#"<a class="mark-breadcrumb-link" href="{href}">{display}</a><span class="mark-breadcrumb-sep">&rsaquo;</span>"#
        ));
    }
    html.push_str(&format!(
        r#"<span class="mark-breadcrumb-current">{}</span></nav>"#,
        escape_html(title)
    ));
    html
}

fn render_sidebar_controls(sidebar_visible: bool) -> String {
    let expanded = if sidebar_visible { "true" } else { "false" };
    format!(
        r#"<div class="mark-left-control"><button type="button" id="mark-sidebar-button" class="mark-sidebar-button" aria-controls="mark-sidebar" aria-expanded="{expanded}" aria-label="Toggle sidebar (e)" title="Toggle sidebar (e)"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-panel-left h-3.5 w-3.5" aria-hidden="true"><rect width="18" height="18" x="3" y="3" rx="2"></rect><path d="M9 3v18"></path></svg></button></div>"#,
    )
}

fn render_theme_option(theme: &str, label: &str, icon: &str) -> String {
    format!(
        r#"<button type="button" class="mark-theme-option" data-theme-option="{theme}"><span class="mark-theme-option-icon" aria-hidden="true">{icon}</span><span class="mark-theme-option-label">{label}</span></button>"#
    )
}

fn format_decimal(value: f32) -> String {
    let mut rendered = format!("{value:.2}");
    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }
    rendered
}

fn render_layout_command(appearance: AppearanceConfig) -> String {
    format!(
        "mark config set-layout --font-size {} --letter-width {} --letter-radius {} --sidebar-button-radius {} --theme-button-radius {}",
        appearance.font_size_px,
        format_decimal(appearance.letter_width_in),
        appearance.letter_radius_px,
        appearance.sidebar_button_radius_px,
        appearance.theme_button_radius_px,
    )
}

fn render_theme_controls(appearance: AppearanceConfig) -> String {
    let theme_menu = [
        render_theme_option(
            "system",
            "System",
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-monitor h-3.5 w-3.5"><rect width="20" height="14" x="2" y="3" rx="2"></rect><line x1="8" x2="16" y1="21" y2="21"></line><line x1="12" x2="12" y1="17" y2="21"></line></svg>"#,
        ),
        render_theme_option(
            "light",
            "Light",
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-sun h-3.5 w-3.5"><circle cx="12" cy="12" r="4"></circle><path d="M12 2v2"></path><path d="M12 20v2"></path><path d="m4.93 4.93 1.41 1.41"></path><path d="m17.66 17.66 1.41 1.41"></path><path d="M2 12h2"></path><path d="M20 12h2"></path><path d="m6.34 17.66-1.41 1.41"></path><path d="m19.07 4.93-1.41 1.41"></path></svg>"#,
        ),
        render_theme_option(
            "dark",
            "Dark",
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-moon h-3.5 w-3.5"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9"></path></svg>"#,
        ),
    ]
    .join("");
    let layout_command = escape_html(&render_layout_command(appearance));

    format!(
        r#"<div class="mark-right-control"><button type="button" id="mark-export-pdf" class="mark-shell-button mark-export-pdf-button" aria-label="Export document as PDF" title="Export document as PDF (Primary+Shift+E)"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-download h-3.5 w-3.5" aria-hidden="true"><path d="M12 15V3"></path><path d="M7 10l5 5 5-5"></path><path d="M5 21h14"></path></svg></button><div class="mark-theme-control mark-theme-button-shell"><button type="button" id="mark-theme-toggle" class="mark-shell-button mark-theme-toggle-button" aria-label="Open config menu" title="Open config menu (c)" aria-haspopup="dialog" aria-expanded="false"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-settings h-3.5 w-3.5" aria-hidden="true"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path><circle cx="12" cy="12" r="3"></circle></svg></button><aside id="mark-theme-menu" class="mark-theme-menu" aria-label="Theme and reader layout controls"><div class="mark-theme-menu-inner"><section class="mark-theme-menu-section"><div class="mark-theme-menu-heading">Theme</div><div class="mark-theme-option-list">{theme_menu}</div></section><section class="mark-theme-menu-section"><div class="mark-theme-menu-heading">Hotkeys</div><ul class="mark-hotkey-list"><li><kbd>E</kbd><span>Toggle hierarchy</span></li><li><kbd>T</kbd><span>Toggle light/dark theme</span></li><li><kbd>C</kbd><span>Toggle config</span></li><li><kbd>Z</kbd><span>Zen mode</span></li><li><kbd>Shift</kbd><span>+</span><kbd>Primary</kbd><span>+</span><kbd>E</kbd><span>Export PDF</span></li></ul><p class="mark-layout-help">Primary means Command on macOS and Control on other platforms.</p></section><form id="mark-layout-form" class="mark-layout-form" autocomplete="off"><div class="mark-theme-menu-heading">Reader layout</div><label class="mark-layout-field"><span>Font size (px)</span><input id="mark-font-size-input" type="number" min="10" max="32" step="1" value="{font_size}"></label><label class="mark-layout-field"><span>Letter width (rem)</span><input id="mark-letter-width-input" type="number" min="5" max="12" step="0.05" value="{letter_width}"></label><label class="mark-layout-field"><span>Letter corner radius (px)</span><input id="mark-letter-radius-input" type="number" min="0" max="64" step="1" value="{letter_radius}"></label><label class="mark-layout-field"><span>Sidebar button radius (px)</span><input id="mark-sidebar-button-radius-input" type="number" min="0" max="999" step="1" value="{sidebar_button_radius}"></label><label class="mark-layout-field"><span>Theme button radius (px)</span><input id="mark-theme-button-radius-input" type="number" min="0" max="999" step="1" value="{theme_button_radius}"></label><button type="button" id="mark-save-layout" class="mark-save-layout-btn" style="margin-top:1rem" disabled>Save</button><details class="mark-layout-command-accordion"><summary class="mark-layout-command-summary"><span class="mark-layout-command-summary-label">Terminal command</span><button type="button" id="mark-copy-layout-command" class="mark-layout-copy">Copy</button></summary><div class="mark-layout-command-wrap"><code id="mark-layout-command" class="mark-layout-command">{layout_command}</code></div></details></form></div></aside></div></div>"#,
        font_size = appearance.font_size_px,
        letter_width = format_decimal(appearance.letter_width_in),
        letter_radius = appearance.letter_radius_px,
        sidebar_button_radius = appearance.sidebar_button_radius_px,
        theme_button_radius = appearance.theme_button_radius_px,
    )
}

fn render_sidebar_shell(
    all_files: &[(String, PathBuf, bool)],
    run_dir: &Path,
    sidebar_visible: bool,
) -> String {
    let hidden_class = if sidebar_visible {
        ""
    } else {
        " -translate-x-full"
    };
    let tree = build_sidebar_tree(all_files, run_dir);
    let mut nav = String::new();
    render_sidebar_nodes(&tree, &mut nav);

    format!(
        r#"<aside id="mark-sidebar" class="mark-sidebar{hidden_class}"><div class="mark-sidebar-inner"><section class="mark-sidebar-scroll"><h2 class="mark-sidebar-title">Hierarchy</h2><input type="search" id="mark-sidebar-search" class="mark-sidebar-search" placeholder="Search…" aria-label="Search hierarchy"><nav class="mark-sidebar-tree" aria-label="Rendered file tree">{nav}</nav></section></div></aside>"#,
    )
}

/// Wrap a rendered HTML body in a complete HTML5 document with embedded CSS and JS.
///
/// `breadcrumb` is an ordered list of `(display_name, html_path)` ancestors
/// from the entry-point down to (but not including) the current file. Empty for
/// the entry-point itself.
///
/// `all_files` is the full list of `(display_name, html_path, is_current)`
/// entries for the sidebar, in BFS discovery order.
fn build_html_document(title: &str, body: &str, theme: Theme, chrome: RenderChrome<'_>) -> String {
    let base_css = include_str!("style.css");
    let theme_attr = match theme {
        Theme::System => "system",
        Theme::Dark => "dark",
        Theme::Light => "light",
    };
    let html_root = match theme {
        Theme::Dark => format!(r#"<html lang="en" data-theme="{theme_attr}" class="dark">"#),
        _ => format!(r#"<html lang="en" data-theme="{theme_attr}">"#),
    };

    let extra_css = format!(
        r#"<style>
:root{{--mark-font-size:{font_size}px;--mark-letter-width:{letter_width}in;--mark-letter-radius:{letter_radius}px;--mark-sidebar-button-radius:{sidebar_button_radius}px;--mark-theme-button-radius:{theme_button_radius}px}}
.mark-left-control,.mark-right-control{{position:fixed;top:1.5rem;z-index:50}}
.mark-left-control{{left:1.5rem}}
.mark-right-control{{right:1.5rem;display:flex;align-items:flex-start;gap:.75rem}}
.mark-shell-button,.mark-sidebar-button{{display:flex;align-items:center;justify-content:center;width:3.25rem;height:3.25rem;border:1px solid var(--border);background:var(--control-bg);color:var(--muted-foreground);box-shadow:0 8px 24px rgba(0,0,0,.12);cursor:pointer;transition:transform .15s ease,background .15s ease,color .15s ease}}
.mark-sidebar-button{{border-radius:var(--mark-sidebar-button-radius)}}
.mark-theme-toggle-button{{position:absolute;top:0;right:0;z-index:70;border-radius:var(--mark-theme-button-radius)}}
.mark-export-pdf-button{{position:absolute;top:0;right:0;z-index:70;border-radius:var(--mark-theme-button-radius);transform:translateX(-100%);margin-right:8px;}}
.mark-sidebar-button:hover,.mark-shell-button:hover,.mark-layout-copy:hover{{transform:translateY(-1px);background:var(--control-hover);color:var(--foreground)}}
.mark-theme-button-shell{{position:relative}}
.mark-theme-menu{{position:fixed;top:0;right:0;z-index:60;width:min(28rem,calc(100vw - 1rem));height:100vh;border-left:1px solid var(--border);background:color-mix(in srgb,var(--card) 98%,transparent);box-shadow:-18px 0 40px rgba(0,0,0,.18);backdrop-filter:blur(18px);transform:translateX(100%);opacity:0;transition:transform .22s ease,opacity .15s ease;pointer-events:none}}
.mark-theme-menu.mark-theme-menu--open{{transform:translateX(0);opacity:1;pointer-events:auto}}
.mark-theme-menu-inner{{display:grid;gap:1rem;height:100%;overflow-y:auto;padding:6rem 1.25rem 1.5rem}}
.mark-theme-menu-section{{display:grid;gap:.65rem}}
.mark-theme-menu-heading{{font-size:.72rem;font-weight:700;letter-spacing:.08em;text-transform:uppercase;color:var(--muted-foreground)}}
.mark-theme-option-list{{display:grid;gap:.25rem}}
.mark-theme-option{{display:flex;align-items:center;gap:.65rem;width:100%;padding:.55rem .7rem;border:none;border-radius:.75rem;background:transparent;color:var(--foreground);cursor:pointer;text-align:left}}
.mark-theme-option:hover,.mark-theme-option[aria-pressed="true"]{{background:var(--muted)}}
.mark-theme-option-icon,.mark-theme-option-label{{pointer-events:none}}
.mark-theme-option-icon{{display:inline-flex;align-items:center;justify-content:center;color:var(--muted-foreground)}}
.mark-hotkey-list{{display:grid;gap:.45rem;margin:0;padding:0;list-style:none}}
.mark-hotkey-list li{{display:flex;align-items:center;gap:.4rem;flex-wrap:wrap;font-size:.82rem;color:var(--foreground)}}
.mark-hotkey-list kbd{{min-width:1.75rem;padding:.18rem .4rem;border:1px solid var(--border);border-radius:.45rem;background:var(--background);font:inherit;font-size:.75rem;font-weight:600;line-height:1.2;text-align:center}}
.mark-layout-form{{display:grid;gap:.75rem;padding-top:.2rem}}
.mark-layout-help{{margin:0;color:var(--muted-foreground);font-size:.85rem;line-height:1.45}}
.mark-layout-field{{display:grid;gap:.35rem}}
.mark-layout-field span{{font-size:.82rem;color:var(--foreground)}}
.mark-layout-field input{{width:100%;border:1px solid var(--border);border-radius:.7rem;background:var(--background);color:var(--foreground);padding:.6rem .75rem;font:inherit}}
.mark-layout-field input:focus{{outline:2px solid var(--ring);outline-offset:2px}}
.mark-layout-command-wrap{{display:grid;gap:.5rem;padding:.45rem 0 0}}
.mark-layout-copy{{border:1px solid var(--border);border-radius:.65rem;background:transparent;color:var(--muted-foreground);padding:.25rem .5rem;font:inherit;font-size:.78rem;cursor:pointer;opacity:.8}}
.mark-layout-copy:hover{{background:var(--muted);opacity:1;color:var(--foreground)}}
.mark-layout-command{{display:block;white-space:pre-wrap;word-break:break-word;font-size:.78rem;line-height:1.5}}
.mark-breadcrumb{{display:flex;flex-wrap:wrap;align-items:center;gap:.45rem;margin-bottom:1.35rem;padding-bottom:.85rem;border-bottom:1px solid var(--border);font-size:.92rem;color:var(--muted-foreground)}}
.mark-breadcrumb-link{{color:var(--link);text-decoration:none}}
.mark-breadcrumb-link:hover{{text-decoration:underline}}
.mark-breadcrumb-current{{font-weight:600;color:var(--foreground)}}
.mark-sidebar{{position:fixed;inset:0 auto 0 0;z-index:40;width:min(22rem,calc(100vw - 2rem));background:var(--card);border-right:1px solid var(--border);box-shadow:0 18px 50px rgba(0,0,0,.16);transition:transform .22s ease}}
.mark-sidebar.-translate-x-full{{transform:translateX(calc(-100% - 1.5rem))}}
.mark-sidebar-inner{{display:flex;height:100%;min-height:0;flex-direction:column;padding:6rem 1.25rem 1rem}}
.mark-sidebar-scroll{{flex:1;overflow-y:auto;padding-bottom:1rem}}
.mark-sidebar-title{{margin:0 0 1rem;font-size:.72rem;font-weight:700;letter-spacing:.08em;text-transform:uppercase;color:var(--muted-foreground)}}
.mark-sidebar-tree{{display:grid;gap:.5rem}}
.mark-sidebar-link,.mark-sidebar-current,.mark-sidebar-summary{{display:flex;min-height:2.5rem;align-items:center;padding:.55rem .75rem;border-radius:.75rem}}
.mark-sidebar-link{{color:var(--foreground);text-decoration:none}}
.mark-sidebar-link:hover,.mark-sidebar-summary:hover{{background:var(--muted)}}
.mark-sidebar-current{{background:var(--current);font-weight:600}}
.mark-sidebar-group{{display:grid;gap:.35rem}}
.mark-sidebar-group>summary::-webkit-details-marker{{display:none}}
.mark-sidebar-summary{{cursor:pointer;list-style:none;color:var(--foreground);font-weight:600}}
.mark-sidebar-group>summary::before{{content:"▾";display:inline-block;margin-right:.5rem;color:var(--muted-foreground);transition:transform .15s ease}}
.mark-sidebar-group:not([open])>summary::before{{transform:rotate(-90deg)}}
.mark-sidebar-group-children{{display:grid;gap:.35rem;margin-left:1rem;padding-left:.9rem;border-left:1px solid var(--border-soft)}}
.mark-code-block{{position:relative;margin:1em 0}}
.mark-code-toolbar{{display:flex;justify-content:flex-end;gap:.4em;padding:.45rem .6rem;background:var(--muted);border:1px solid var(--border);border-bottom:none;border-radius:.5rem .5rem 0 0}}
.mark-code-block pre{{margin-top:0;border-top-left-radius:0;border-top-right-radius:0}}
.mark-btn{{font-size:.78em;padding:.2em .6em;border:1px solid var(--border);border-radius:.4rem;background:var(--background);color:var(--foreground);cursor:pointer;transition:background .15s}}
.mark-btn:hover{{background:var(--control-hover)}}
.mark-btn.mark-copied{{color:#2a7a2a;border-color:#2a7a2a}}
.mark-btn.mark-failed{{color:#b00;border-color:#b00}}
.mark-layout-command-accordion{{border:none;background:transparent;padding:0}}
.mark-layout-command-accordion summary{{display:flex;align-items:center;gap:.75rem;list-style:none;cursor:pointer;font-size:.8rem;font-weight:600;color:var(--muted-foreground);padding:.25rem 0;user-select:none}}
.mark-layout-command-summary-label{{display:inline-flex;align-items:center;gap:.4rem;min-width:0}}
.mark-layout-command-accordion summary::-webkit-details-marker{{display:none}}
.mark-layout-command-accordion[open] summary{{color:var(--foreground)}}
.mark-layout-command-accordion summary::before{{content:"▸";display:inline-block;font-size:.7rem;transition:transform .15s ease}}
.mark-layout-command-accordion[open] summary::before{{transform:rotate(90deg)}}
.mark-layout-command-accordion .mark-layout-command-wrap{{border:none;background:transparent;padding:.5rem 0 0}}
.mark-layout-command-accordion .mark-layout-copy{{margin-left:auto}}
.mark-save-layout-btn{{width:100%;margin-top:.5rem;padding:.55rem;border:1px solid var(--border);border-radius:.85rem;background:var(--background);color:var(--foreground);cursor:pointer;font:inherit;font-size:.85rem;transition:background .15s ease,opacity .15s ease}}
.mark-save-layout-btn:not(:disabled):hover{{background:var(--control-hover);transform:translateY(-1px)}}
.mark-save-layout-btn:disabled{{opacity:.35;cursor:not-allowed}}
.mark-sidebar-search{{width:100%;padding:.5rem .75rem;border:1px solid var(--border);border-radius:.7rem;background:var(--background);color:var(--foreground);font:inherit;font-size:.85rem;margin-bottom:.75rem;box-sizing:border-box}}
.mark-sidebar-search:focus{{outline:2px solid var(--ring);outline-offset:2px}}
html.mark-zen-mode .mark-left-control,html.mark-zen-mode .mark-right-control,html.mark-zen-mode #mark-sidebar,html.mark-zen-mode .mark-theme-menu{{display:none!important}}
html.mark-zen-mode .editor-shell,html.mark-zen-mode .mark-content-wrapper,html.mark-zen-mode .mark-main-shell,html.mark-zen-mode .mark-page-width-shell{{background:var(--mark-zen-bg,var(--card))}}
html.mark-zen-mode .paper-sheet{{border:none;background:var(--mark-zen-bg,var(--card));box-shadow:none;border-radius:0}}
html.mark-zen-mode,html.mark-zen-mode body{{background:var(--mark-zen-bg,var(--card))}}
@media (max-width: 768px){{.mark-left-control{{left:1rem}}.mark-right-control{{right:1rem;gap:.5rem}}.mark-content-wrapper{{padding-top:1rem}}.paper-sheet{{padding:1.5rem}}}}
</style>"#,
        font_size = chrome.appearance.font_size_px,
        letter_width = format_decimal(chrome.appearance.letter_width_in),
        letter_radius = chrome.appearance.letter_radius_px,
        sidebar_button_radius = chrome.appearance.sidebar_button_radius_px,
        theme_button_radius = chrome.appearance.theme_button_radius_px,
    );

    let early_theme_script = r#"<script>(function(){var root=document.documentElement;var stored=null;try{stored=sessionStorage.getItem('mark-theme');}catch(e){}var theme=stored||root.getAttribute('data-theme')||'system';if(stored){root.setAttribute('data-theme',theme);}var dark=theme==='dark'||(theme==='system'&&window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches);root.classList.toggle('dark',dark);})();</script>"#;

    let enhancement_js = r#"<script>(function() {

  function flash(btn, msg, cls) {
    var original = btn.innerHTML;
    btn.textContent = msg;
    btn.classList.add(cls);
    setTimeout(function() {
      btn.innerHTML = original;
      btn.classList.remove(cls);
    }, 1800);
  }

  function copyText(text, btn, successMsg) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(function() {
        flash(btn, successMsg, 'mark-copied');
      }).catch(function() {
        flash(btn, '\u2717 Failed', 'mark-failed');
      });
      return;
    }

    try {
      var ta = document.createElement('textarea');
      ta.value = text;
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.focus();
      ta.select();
      var ok = false;
      try {
        ok = document.execCommand('copy');
      } finally {
        document.body.removeChild(ta);
      }
      flash(btn, ok ? successMsg : '\u2717 Failed', ok ? 'mark-copied' : 'mark-failed');
    } catch (error) {
      flash(btn, '\u2717 Failed', 'mark-failed');
    }
  }

  function prefersDark() {
    return !!(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
  }

  function closeThemeMenu() {
    var menu = document.getElementById('mark-theme-menu');
    var toggle = document.getElementById('mark-theme-toggle');
    if (menu) {
      menu.classList.remove('mark-theme-menu--open');
    }
    if (toggle) {
      toggle.setAttribute('aria-expanded', 'false');
    }
  }

  function updateThemeButtons(theme) {
    document.querySelectorAll('[data-theme-option]').forEach(function(button) {
      var active = button.dataset.themeOption === theme;
      button.setAttribute('aria-pressed', active ? 'true' : 'false');
    });
  }

  function setTheme(theme) {
    var root = document.documentElement;
    root.setAttribute('data-theme', theme);
    root.classList.toggle('dark', theme === 'dark' || (theme === 'system' && prefersDark()));
    updateThemeButtons(theme);
    if (root.classList.contains('mark-zen-mode')) {
      syncZenModeBackground();
    }
    try { sessionStorage.setItem('mark-theme', theme); } catch(e) {}
  }

  function toggleTheme() {
    var currentTheme = document.documentElement.getAttribute('data-theme') || 'system';
    var nextTheme = currentTheme === 'dark' ? 'light' : 'dark';
    setTheme(nextTheme);
  }

  function toggleConfigMenu() {
    var themeToggle = document.getElementById('mark-theme-toggle');
    var themeMenu = document.getElementById('mark-theme-menu');
    if (themeToggle && themeMenu) {
      var expanded = themeToggle.getAttribute('aria-expanded') === 'true';
      themeMenu.classList.toggle('mark-theme-menu--open', !expanded);
      themeToggle.setAttribute('aria-expanded', expanded ? 'false' : 'true');
    }
  }

  async function exportPdf() {
    var originalTitle = document.title;
    if (window.showSaveFilePicker) {
      try {
        var handle = await window.showSaveFilePicker({
          suggestedName: originalTitle.replace(/\.[^.]+$/, '') + '.pdf',
          types: [{ description: 'PDF document', accept: { 'application/pdf': ['.pdf'] } }]
        });
        if (handle && handle.name) {
          document.title = handle.name.replace(/\.pdf$/i, '');
        }
      } catch (error) {
        if (error && error.name === 'AbortError') {
          return;
        }
      }
    }

    var restoreTitle = function() {
      document.title = originalTitle;
      window.removeEventListener('afterprint', restoreTitle);
    };
    window.addEventListener('afterprint', restoreTitle);
    window.print();
    setTimeout(restoreTitle, 1000);
  }

  function updateSidebarState() {
    var sidebar = document.getElementById('mark-sidebar');
    var toggle = document.getElementById('mark-sidebar-button');
    if (!sidebar || !toggle) {
      return;
    }
    var visible = !sidebar.classList.contains('-translate-x-full');
    toggle.setAttribute('aria-expanded', visible ? 'true' : 'false');
  }

  function toggleSidebar() {
    var sidebar = document.getElementById('mark-sidebar');
    if (!sidebar) {
      return;
    }
    sidebar.classList.toggle('-translate-x-full');
    updateSidebarState();
  }

  function syncZenModeBackground() {
    var root = document.documentElement;
    var page = document.querySelector('.paper-sheet');
    var resolved = '';
    if (page && window.getComputedStyle) {
      resolved = window.getComputedStyle(page).getPropertyValue('background-color') || window.getComputedStyle(page).backgroundColor || '';
    }
    if (!resolved || resolved === 'transparent' || resolved === 'rgba(0, 0, 0, 0)') {
      resolved = (window.getComputedStyle ? window.getComputedStyle(root).getPropertyValue('--card') : '') || '';
    }
    root.style.setProperty('--mark-zen-bg', resolved.trim() || 'var(--card)');
  }

  function clearZenModeBackground() {
    document.documentElement.style.removeProperty('--mark-zen-bg');
  }

  function applyZenMode(enabled) {
    var root = document.documentElement;
    if (enabled) {
      syncZenModeBackground();
    }
    root.classList.toggle('mark-zen-mode', enabled);
    if (!enabled) {
      clearZenModeBackground();
    }
  }

  function toggleZenMode() {
    applyZenMode(!document.documentElement.classList.contains('mark-zen-mode'));
  }

  function isEditableTarget(target) {
    if (!target) {
      return false;
    }
    if (target.closest('textarea, select')) {
      return true;
    }
    if (target.closest('[contenteditable]:not([contenteditable="false"])')) {
      return true;
    }
    var input = target.closest('input');
    if (!input) {
      return false;
    }
    var type = (input.getAttribute('type') || 'text').toLowerCase();
    return !['checkbox', 'radio', 'button', 'submit', 'reset', 'range', 'color', 'file'].includes(type);
  }

  function commandValue(input, fallback) {
    if (!input) {
      return fallback;
    }
    var numeric = Number(input.value);
    if (!Number.isFinite(numeric)) {
      return fallback;
    }
    return String(numeric);
  }

  function normalizeLayoutInput(input, defaultVal) {
    if (input && (input.value === '' || !Number.isFinite(Number(input.value)))) {
      input.value = defaultVal;
    }
  }

  function normalizeLayoutInputs() {
    normalizeLayoutInput(document.getElementById('mark-font-size-input'), '16');
    normalizeLayoutInput(document.getElementById('mark-letter-width-input'), '8.5');
    normalizeLayoutInput(document.getElementById('mark-letter-radius-input'), '12');
    normalizeLayoutInput(document.getElementById('mark-sidebar-button-radius-input'), '999');
    normalizeLayoutInput(document.getElementById('mark-theme-button-radius-input'), '999');
  }

  function updateLayoutCommand() {
    var command = document.getElementById('mark-layout-command');
    if (!command) {
      return;
    }

    var fontSize = commandValue(document.getElementById('mark-font-size-input'), '16');
    var letterWidth = commandValue(document.getElementById('mark-letter-width-input'), '8.5');
    var letterRadius = commandValue(document.getElementById('mark-letter-radius-input'), '12');
    var sidebarRadius = commandValue(document.getElementById('mark-sidebar-button-radius-input'), '999');
    var themeRadius = commandValue(document.getElementById('mark-theme-button-radius-input'), '999');

    command.textContent = 'mark config set-layout --font-size ' + fontSize + ' --letter-width ' + letterWidth + ' --letter-radius ' + letterRadius + ' --sidebar-button-radius ' + sidebarRadius + ' --theme-button-radius ' + themeRadius;
  }

  function updateLiveLayout() {
    var root = document.documentElement;
    var fontSizeEl = document.getElementById('mark-font-size-input');
    if (fontSizeEl && fontSizeEl.value) {
      root.style.setProperty('--mark-font-size', fontSizeEl.value + 'px');
    }
    var letterWidthEl = document.getElementById('mark-letter-width-input');
    if (letterWidthEl && letterWidthEl.value) {
      root.style.setProperty('--mark-letter-width', letterWidthEl.value + 'in');
    }
    var letterRadiusEl = document.getElementById('mark-letter-radius-input');
    if (letterRadiusEl && letterRadiusEl.value) {
      root.style.setProperty('--mark-letter-radius', letterRadiusEl.value + 'px');
    }
    var sidebarRadiusEl = document.getElementById('mark-sidebar-button-radius-input');
    if (sidebarRadiusEl && sidebarRadiusEl.value) {
      root.style.setProperty('--mark-sidebar-button-radius', sidebarRadiusEl.value + 'px');
    }
    var themeRadiusEl = document.getElementById('mark-theme-button-radius-input');
    if (themeRadiusEl && themeRadiusEl.value) {
      root.style.setProperty('--mark-theme-button-radius', themeRadiusEl.value + 'px');
    }
  }

  var initialLayoutValues = {};

  function updateSaveButton() {
    var saveBtn = document.getElementById('mark-save-layout');
    if (!saveBtn) { return; }
    var form = document.getElementById('mark-layout-form');
    if (!form) { saveBtn.disabled = true; return; }
    var changed = false;
    form.querySelectorAll('input').forEach(function(input) {
      if (input.value !== initialLayoutValues[input.id]) { changed = true; }
    });
    saveBtn.disabled = !changed;
  }

  document.addEventListener('DOMContentLoaded', function() {
    document.querySelectorAll('.mark-code-block').forEach(function(block) {
      var pre = block.querySelector('pre');
      var code = pre ? pre.querySelector('code') : null;
      var copyBtn = block.querySelector('.mark-copy-btn');
      if (copyBtn && code) {
        copyBtn.addEventListener('click', function() {
          copyText(code.textContent, copyBtn, '\u2713 Copied');
        });
      }
      var cleanBtn = block.querySelector('.mark-copy-clean-btn');
      if (cleanBtn) {
        var cleanCode = block.dataset.cleanCode;
        if (cleanCode !== undefined) {
          cleanBtn.addEventListener('click', function() {
            copyText(cleanCode, cleanBtn, '\u2713 Copied clean');
          });
        }
      }
    });

    setTheme(document.documentElement.getAttribute('data-theme') || 'system');

    var sidebarToggle = document.getElementById('mark-sidebar-button');
    if (sidebarToggle) {
      sidebarToggle.addEventListener('click', function() {
        toggleSidebar();
      });
      updateSidebarState();
    }

    var themeToggle = document.getElementById('mark-theme-toggle');
    var themeMenu = document.getElementById('mark-theme-menu');
    if (themeToggle && themeMenu) {
      themeToggle.addEventListener('click', function(event) {
        event.preventDefault();
        var expanded = themeToggle.getAttribute('aria-expanded') === 'true';
        themeToggle.setAttribute('aria-expanded', expanded ? 'false' : 'true');
        themeMenu.classList.toggle('mark-theme-menu--open', !expanded);
      });

      document.querySelectorAll('[data-theme-option]').forEach(function(button) {
        button.addEventListener('click', function() {
          setTheme(button.dataset.themeOption);
          closeThemeMenu();
        });
      });

      document.addEventListener('click', function(event) {
        if (!themeMenu.classList.contains('mark-theme-menu--open')) {
          return;
        }
        if (themeMenu.contains(event.target) || themeToggle.contains(event.target)) {
          return;
        }
        closeThemeMenu();
      });
    }

    var exportPdfBtn = document.getElementById('mark-export-pdf');
    if (exportPdfBtn) {
      exportPdfBtn.addEventListener('click', function() {
        exportPdf();
      });
    }

    var layoutForm = document.getElementById('mark-layout-form');
    if (layoutForm) {
      layoutForm.querySelectorAll('input').forEach(function(input) {
        initialLayoutValues[input.id] = input.value;
      });
      layoutForm.querySelectorAll('input').forEach(function(input) {
        input.addEventListener('input', function() { normalizeLayoutInputs(); updateLayoutCommand(); updateLiveLayout(); updateSaveButton(); });
        input.addEventListener('change', function() { normalizeLayoutInputs(); updateLayoutCommand(); updateLiveLayout(); updateSaveButton(); });
      });
      updateLayoutCommand();
    }

    var copyLayoutCommand = document.getElementById('mark-copy-layout-command');
    if (copyLayoutCommand) {
      copyLayoutCommand.addEventListener('click', function() {
        var command = document.getElementById('mark-layout-command');
        if (command) {
          copyText(command.textContent, copyLayoutCommand, '\u2713 Copied command');
        }
      });
    }

    var saveLayoutBtn = document.getElementById('mark-save-layout');
    if (saveLayoutBtn) {
      saveLayoutBtn.addEventListener('click', function() {
        var command = document.getElementById('mark-layout-command');
        if (command) {
          copyText(command.textContent, saveLayoutBtn, '\u2713 Saved');
        }
      });
    }

    var media = window.matchMedia ? window.matchMedia('(prefers-color-scheme: dark)') : null;
    if (media) {
      var syncSystemTheme = function() {
        if ((document.documentElement.getAttribute('data-theme') || 'system') === 'system') {
          setTheme('system');
        }
      };
      if (media.addEventListener) {
        media.addEventListener('change', syncSystemTheme);
      } else if (media.addListener) {
        media.addListener(syncSystemTheme);
      }
    }

    document.addEventListener('keydown', function(event) {
      if (event.defaultPrevented || isEditableTarget(event.target)) {
        return;
      }
      var key = (event.key || '').toLowerCase();
      var primaryModifier = event.metaKey || event.ctrlKey;
      if (!event.altKey && primaryModifier && event.shiftKey && key === 'e') {
        exportPdf();
        event.preventDefault();
        return;
      }
      if (event.altKey || event.ctrlKey || event.metaKey) {
        return;
      }
      if (key === 'e') {
        if (document.getElementById('mark-sidebar-button')) {
          toggleSidebar();
          event.preventDefault();
        }
        return;
      }
      if (key === 't') {
        toggleTheme();
        event.preventDefault();
        return;
      }
      if (key === 'c') {
        toggleConfigMenu();
        event.preventDefault();
        return;
      }
      if (key === 'z') {
        toggleZenMode();
        event.preventDefault();
        return;
      }
      if (key === 'escape') {
        closeThemeMenu();
      }
    });

    var sidebarSearch = document.getElementById('mark-sidebar-search');
    if (sidebarSearch) {
      sidebarSearch.addEventListener('input', function() {
        var query = sidebarSearch.value.toLowerCase().trim();
        var sidebar = document.getElementById('mark-sidebar');
        if (!sidebar) { return; }
        if (!query) {
          sidebar.querySelectorAll('.mark-sidebar-link, .mark-sidebar-current, .mark-sidebar-dir').forEach(function(el) {
            el.style.display = '';
          });
          return;
        }
        sidebar.querySelectorAll('a.mark-sidebar-link, span.mark-sidebar-current').forEach(function(el) {
          var match = el.textContent.toLowerCase().indexOf(query) !== -1;
          el.style.display = match ? '' : 'none';
        });
        sidebar.querySelectorAll('.mark-sidebar-dir').forEach(function(dir) {
          var anyVisible = false;
          dir.querySelectorAll('a.mark-sidebar-link, span.mark-sidebar-current').forEach(function(el) {
            if (el.style.display !== 'none') { anyVisible = true; }
          });
          dir.style.display = anyVisible ? '' : 'none';
        });
      });
    }
  });
})();</script>"#;

    let breadcrumb_html = render_breadcrumb_html(title, chrome.breadcrumb);
    let content_html = format!(r#"{breadcrumb_html}<div class="markdown-prose">{body}</div>"#);
    let sidebar_controls_html = if chrome.all_files.is_empty() {
        String::new()
    } else {
        render_sidebar_controls(chrome.sidebar_visible)
    };
    let sidebar_shell_html = if chrome.all_files.is_empty() {
        String::new()
    } else {
        render_sidebar_shell(chrome.all_files, chrome.run_dir, chrome.sidebar_visible)
    };

    format!(
        r#"<!DOCTYPE html>
{html_root}
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
{base_css}
</style>
{extra_css}
{early_theme_script}
</head>
<body>
<div class="editor-shell">{sidebar_controls_html}{theme_controls}<div class="mark-content-wrapper"><main class="mark-main-shell"><div class="mark-page-width-shell"><article class="paper-sheet">{content_html}</article></div></main></div>{sidebar_shell_html}</div>
{enhancement_js}
</body>
</html>
"#,
        html_root = html_root,
        title = escape_html(title),
        base_css = base_css,
        extra_css = extra_css,
        early_theme_script = early_theme_script,
        sidebar_controls_html = sidebar_controls_html,
        theme_controls = render_theme_controls(chrome.appearance),
        content_html = content_html,
        sidebar_shell_html = sidebar_shell_html,
        enhancement_js = enhancement_js,
    )
}

fn render_sidebar_nodes(nodes: &[SidebarNode], out: &mut String) {
    for node in nodes {
        if node.children.is_empty() {
            let display = escape_html(&node.name);
            if node.is_current {
                out.push_str(&format!(
                    r#"<span class="mark-sidebar-link mark-sidebar-current" aria-current="page">{display}</span>"#
                ));
            } else if let Some(path) = &node.path {
                let href = escape_html(&path.to_string_lossy());
                out.push_str(&format!(
                    r#"<a class="mark-sidebar-link" href="{href}">{display}</a>"#
                ));
            }
            continue;
        }

        let display = escape_html(&format!("{}/", node.name));
        out.push_str(&format!(
            r#"<details class="mark-sidebar-dir mark-sidebar-group" open><summary class="mark-sidebar-summary">{display}</summary><div class="mark-sidebar-group-children">"#
        ));
        render_sidebar_nodes(&node.children, out);
        out.push_str("</div></details>");
    }
}

fn insert_sidebar_entry(
    nodes: &mut Vec<SidebarNode>,
    directories: &[String],
    display_name: &str,
    html_path: &Path,
    is_current: bool,
) {
    if let Some((head, tail)) = directories.split_first() {
        let index = if let Some(idx) = nodes
            .iter()
            .position(|node| node.path.is_none() && node.name == *head)
        {
            idx
        } else {
            nodes.push(SidebarNode {
                name: head.clone(),
                path: None,
                is_current: false,
                children: Vec::new(),
            });
            nodes.len() - 1
        };
        insert_sidebar_entry(
            &mut nodes[index].children,
            tail,
            display_name,
            html_path,
            is_current,
        );
    } else {
        nodes.push(SidebarNode {
            name: display_name.to_string(),
            path: Some(html_path.to_path_buf()),
            is_current,
            children: Vec::new(),
        });
    }
}

/// Build a SidebarNode tree from the flat all_files list.
///
/// Each entry's relative position in the tree is derived from its relative path
/// under the run_dir (strip run_dir prefix → relative path → tree position).
pub fn build_sidebar_tree(
    all_files: &[(String, PathBuf, bool)],
    run_dir: &Path,
) -> Vec<SidebarNode> {
    let mut tree = Vec::new();

    for (display_name, html_abs_path, is_current) in all_files {
        let relative = html_abs_path
            .strip_prefix(run_dir)
            .unwrap_or(html_abs_path.as_path());
        let directories: Vec<String> = relative
            .parent()
            .map(|parent| {
                parent
                    .components()
                    .filter_map(|component| match component {
                        std::path::Component::Normal(part) => {
                            Some(part.to_string_lossy().into_owned())
                        }
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default();

        insert_sidebar_entry(
            &mut tree,
            &directories,
            display_name,
            html_abs_path,
            *is_current,
        );
    }

    sort_sidebar_nodes(&mut tree);
    tree
}

fn sort_sidebar_nodes(nodes: &mut [SidebarNode]) {
    nodes.sort_by(|left, right| {
        let left_is_dir = left.path.is_none();
        let right_is_dir = right.path.is_none();
        left_is_dir
            .cmp(&right_is_dir)
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
            .then_with(|| left.name.cmp(&right.name))
    });

    for node in nodes.iter_mut() {
        if !node.children.is_empty() {
            sort_sidebar_nodes(&mut node.children);
        }
    }
}

/// Escape special HTML characters in a plain-text value.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Decode a small set of HTML entities using a single-pass scan so that
/// sequences like `&amp;lt;` are decoded to `&lt;` (not to `<`).
fn html_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut remaining = s;
    while let Some(amp_pos) = remaining.find('&') {
        result.push_str(&remaining[..amp_pos]);
        remaining = &remaining[amp_pos..];
        if remaining.starts_with("&amp;") {
            result.push('&');
            remaining = &remaining["&amp;".len()..];
        } else if remaining.starts_with("&lt;") {
            result.push('<');
            remaining = &remaining["&lt;".len()..];
        } else if remaining.starts_with("&gt;") {
            result.push('>');
            remaining = &remaining["&gt;".len()..];
        } else if remaining.starts_with("&quot;") {
            result.push('"');
            remaining = &remaining["&quot;".len()..];
        } else if remaining.starts_with("&#39;") {
            result.push('\'');
            remaining = &remaining["&#39;".len()..];
        } else if remaining.starts_with("&#x27;") {
            result.push('\'');
            remaining = &remaining["&#x27;".len()..];
        } else {
            // Unknown entity — pass through the `&` literally.
            result.push('&');
            remaining = &remaining[1..];
        }
    }
    result.push_str(remaining);
    result
}

/// Encode a string for safe use as an HTML attribute value.
fn html_encode_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Wrap every `<pre><code…>…</code></pre>` block in a toolbar div with copy
/// buttons.  For supported languages a `data-clean-code` attribute is added
/// and a second "Copy clean" button is included.
pub(crate) fn post_process_code_blocks(html: &str) -> String {
    let mut output = String::with_capacity(html.len() + 512);
    let mut rest = html;

    while let Some(pre_start) = rest.find("<pre>") {
        // Emit everything before this `<pre>`.
        output.push_str(&rest[..pre_start]);
        rest = &rest[pre_start..];

        // Find the matching `</pre>`.
        let Some(pre_end) = rest.find("</pre>") else {
            break;
        };
        let pre_end = pre_end + "</pre>".len();
        let pre_block = &rest[..pre_end]; // the full `<pre>…</pre>` slice
        rest = &rest[pre_end..];

        // Extract the language from `<code class="language-LANG">` if present.
        let lang: Option<&str> = pre_block.find(r#"class="language-"#).and_then(|pos| {
            let after = &pre_block[pos + r#"class="language-"#.len()..];
            let end = after.find('"')?;
            Some(&after[..end])
        });

        // Extract the raw (HTML-encoded) code text between `<code…>` and `</code>`.
        let raw_code: &str = pre_block
            .find('>')
            .and_then(|open_gt| {
                let after_open_tag = &pre_block[open_gt + 1..];
                // Skip into the `<code…>` tag.
                after_open_tag.find('>').and_then(|code_gt| {
                    let after_code_tag = &after_open_tag[code_gt + 1..];
                    let end = after_code_tag.find("</code>")?;
                    Some(&after_code_tag[..end])
                })
            })
            .unwrap_or("");

        let toolbar_and_wrapper = if let Some(lang_str) = lang {
            if is_supported_language(lang_str) {
                let decoded = html_decode(raw_code);
                let cleaned = strip_full_line_comments(lang_str, &decoded);
                let encoded_clean = html_encode_attr(&cleaned);
                format!(
                    r#"<div class="mark-code-block" data-clean-code="{encoded_clean}">
<div class="mark-code-toolbar">
<button class="mark-btn mark-copy-btn" type="button">&#x1F4CB; Copy</button>
<button class="mark-btn mark-copy-clean-btn" type="button">&#x1F9F9; Copy clean</button>
</div>
{pre_block}
</div>"#
                )
            } else {
                // Known language, but not one we support for clean-copy.
                format!(
                    r#"<div class="mark-code-block">
<div class="mark-code-toolbar">
<button class="mark-btn mark-copy-btn" type="button">&#x1F4CB; Copy</button>
</div>
{pre_block}
</div>"#
                )
            }
        } else {
            // No language annotation.
            format!(
                r#"<div class="mark-code-block">
<div class="mark-code-toolbar">
<button class="mark-btn mark-copy-btn" type="button">&#x1F4CB; Copy</button>
</div>
{pre_block}
</div>"#
            )
        };

        output.push_str(&toolbar_and_wrapper);
    }

    // Append anything remaining after the last `</pre>`.
    output.push_str(rest);
    output
}

// ── Link extraction ──────────────────────────────────────────────────────────

/// Returns `true` if the URL looks like an external or non-file reference that
/// should never be rewritten.
fn is_external_url(url: &str) -> bool {
    url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("mailto:")
        || url.starts_with("//")
        || url.starts_with('#')
}

/// Split `url` into `(base, fragment)` at the first `#`.
///
/// If there is no `#`, `fragment` is an empty string.
fn split_fragment(url: &str) -> (&str, &str) {
    match url.find('#') {
        Some(pos) => (&url[..pos], &url[pos..]),
        None => (url, ""),
    }
}

/// Returns `true` if `path` ends with `.md` or `.markdown` (case-insensitive).
fn is_md_extension(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".md") || lower.ends_with(".markdown")
}

/// Extract all local Markdown links from `markdown` that resolve to existing
/// files relative to `source_dir`.
///
/// A link is included when:
/// - it is not an external URL (`http://`, `https://`, `mailto:`, `//`, `#`),
/// - its base path (without any `#fragment`) ends with `.md` or `.markdown`
///   (case-insensitive), and
/// - the resolved path exists on disk (confirmed via
///   [`std::fs::canonicalize`]).
///
/// Image links are **not** included; only `[text](target)` / `[text][ref]`
/// style links are considered.
///
/// # Returns
/// A `Vec` of `(target_base, canonical_path)` tuples in document order,
/// deduplicated by `target_base`.  `target_base` is the link destination with
/// the fragment stripped so it can be used as the key in a rewrite map.
pub fn extract_local_md_links(markdown: &str, source_dir: &Path) -> Vec<(String, PathBuf)> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, opts);
    let mut links: Vec<(String, PathBuf)> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for event in parser {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            let url = dest_url.as_ref();
            if is_external_url(url) {
                continue;
            }
            let (base, _fragment) = split_fragment(url);
            if !is_md_extension(base) {
                continue;
            }
            if seen.contains(base) {
                continue;
            }
            // Resolve and canonicalize.
            let resolved = source_dir.join(base);
            if let Ok(canonical) = std::fs::canonicalize(&resolved) {
                if canonical.is_file() {
                    seen.insert(base.to_string());
                    links.push((base.to_string(), canonical));
                }
            }
        }
    }
    links
}

/// Extract all local **non-Markdown** asset links from `markdown` that resolve
/// to existing files relative to `source_dir`.
///
/// A link or image target is included when:
/// - it is not an external URL (`http://`, `https://`, `mailto:`, `//`, `#`),
/// - its base path (without any `#fragment`) does **not** end with `.md` or
///   `.markdown` (case-insensitive),
/// - the resolved path exists on disk (confirmed via
///   [`std::fs::canonicalize`]), **and**
/// - the resolved path is contained within `source_dir` (path-traversal guard).
///
/// Both `[text](target)` links **and** `![alt](target)` images are collected.
///
/// # Returns
/// A `Vec` of `(base_url, canonical_path)` tuples in document order,
/// deduplicated by `base_url`.  `base_url` is the link destination with the
/// `#fragment` stripped; it is used directly as the key in the rewrite map so
/// that [`render_markdown_rewriting_links`] can find it (that function also
/// strips the fragment before doing a map lookup).
///
/// Multiple distinct `base_url`s that resolve to the same canonical file are
/// each returned so that all spelling variants of the same link are rewritten.
/// Callers are responsible for idempotent copying (e.g. skip copy if
/// destination already exists).
pub fn extract_local_asset_links(markdown: &str, source_dir: &Path) -> Vec<(String, PathBuf)> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    // Canonicalize source_dir once for the containment check.  If it fails
    // (e.g. the directory no longer exists) we fall back to the raw path;
    // canonicalize per-link will still catch most traversal attempts because
    // it resolves `..` components.
    let source_dir_canonical =
        std::fs::canonicalize(source_dir).unwrap_or_else(|_| source_dir.to_path_buf());

    let parser = Parser::new_ext(markdown, opts);
    let mut links: Vec<(String, PathBuf)> = Vec::new();
    let mut seen_base: std::collections::HashSet<String> = std::collections::HashSet::new();

    for event in parser {
        let url_str: String = match event {
            Event::Start(Tag::Link { ref dest_url, .. }) => dest_url.as_ref().to_owned(),
            Event::Start(Tag::Image { ref dest_url, .. }) => dest_url.as_ref().to_owned(),
            _ => continue,
        };

        if is_external_url(&url_str) {
            continue;
        }
        let (base, _fragment) = split_fragment(&url_str);
        if is_md_extension(base) {
            continue;
        }
        if seen_base.contains(base) {
            continue;
        }
        // Resolve and canonicalize using base (fragment stripped).
        let resolved = source_dir.join(base);
        if let Ok(canonical) = std::fs::canonicalize(&resolved) {
            // Guard against path traversal: the resolved file must reside
            // inside source_dir.
            if !canonical.starts_with(&source_dir_canonical) {
                continue;
            }
            if canonical.is_file() {
                seen_base.insert(base.to_string());
                links.push((base.to_string(), canonical));
            }
        }
    }
    links
}

// ── Rendering with link rewriting ────────────────────────────────────────────

/// Render `markdown` to a complete HTML5 document, rewriting every local
/// `.md` link destination to the corresponding rendered `.html` path.
///
/// `link_map` maps a link's **base target** (the part before any `#`) to the
/// absolute [`PathBuf`] of its rendered HTML file.  Anchor fragments in the
/// original link are preserved on the rewritten `href`.
///
/// `breadcrumb` is an ordered list of `(display_name, html_path)` ancestors
/// from the entry-point down to (but not including) the current file.  Pass
/// an empty slice for the entry-point.
///
/// `all_files` is the full BFS-ordered list of `(display_name, html_path,
/// is_current)` entries used to render the sidebar.  Pass an empty slice for
/// standalone (single-file) renders.
///
/// External URLs and links whose base is not present in `link_map` are left
/// unchanged. When there are no links to rewrite, the function still preserves
/// the requested chrome so single-file renders can keep using the same shell.
///
/// The rewriting is performed by transforming pulldown-cmark link events
/// before passing them to the HTML serialiser, so it operates on the parsed
/// AST rather than the raw HTML string.
pub fn render_markdown_rewriting_links(
    markdown: &str,
    title: &str,
    theme: Theme,
    link_map: &HashMap<String, PathBuf>,
    chrome: RenderChrome<'_>,
) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    // Collect into a Vec so we own all event data before calling push_html.
    let events: Vec<Event<'_>> = Parser::new_ext(markdown, opts)
        .map(|event| match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title: link_title,
                id,
            }) => {
                let url = dest_url.as_ref();
                if is_external_url(url) {
                    return Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title: link_title,
                        id,
                    });
                }
                let (base, fragment) = split_fragment(url);
                if let Some(html_path) = link_map.get(base) {
                    let new_url: String = if fragment.is_empty() {
                        html_path.to_string_lossy().into_owned()
                    } else {
                        format!("{}{}", html_path.to_string_lossy(), fragment)
                    };
                    Event::Start(Tag::Link {
                        link_type,
                        dest_url: new_url.into(),
                        title: link_title,
                        id,
                    })
                } else {
                    Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title: link_title,
                        id,
                    })
                }
            }
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title: link_title,
                id,
            }) => {
                let url = dest_url.as_ref();
                if is_external_url(url) {
                    return Event::Start(Tag::Image {
                        link_type,
                        dest_url,
                        title: link_title,
                        id,
                    });
                }
                let (base, fragment) = split_fragment(url);
                if let Some(html_path) = link_map.get(base) {
                    let new_url: String = if fragment.is_empty() {
                        html_path.to_string_lossy().into_owned()
                    } else {
                        format!("{}{}", html_path.to_string_lossy(), fragment)
                    };
                    Event::Start(Tag::Image {
                        link_type,
                        dest_url: new_url.into(),
                        title: link_title,
                        id,
                    })
                } else {
                    Event::Start(Tag::Image {
                        link_type,
                        dest_url,
                        title: link_title,
                        id,
                    })
                }
            }
            other => other,
        })
        .collect();

    let mut body = String::new();
    html::push_html(&mut body, events.into_iter());
    let body = post_process_code_blocks(&body);
    build_html_document(title, &body, theme, chrome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppearanceConfig, Theme};

    fn chrome<'a>(
        breadcrumb: &'a [(String, PathBuf)],
        all_files: &'a [(String, PathBuf, bool)],
        run_dir: &'a Path,
        sidebar_visible: bool,
    ) -> RenderChrome<'a> {
        RenderChrome {
            breadcrumb,
            all_files,
            run_dir,
            sidebar_visible,
            appearance: AppearanceConfig::default(),
        }
    }

    #[test]
    fn render_produces_nonempty_html() {
        let html = render_markdown("# Hello\n\nWorld.", "Hello", Theme::Light);
        assert!(!html.is_empty());
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn render_contains_heading() {
        let html = render_markdown("# My Title\n", "My Title", Theme::Light);
        assert!(html.contains("<h1>My Title</h1>"));
    }

    #[test]
    fn render_contains_paragraph() {
        let html = render_markdown("Hello world\n", "test", Theme::Light);
        assert!(html.contains("<p>Hello world</p>"));
    }

    #[test]
    fn render_title_in_head() {
        let html = render_markdown("text", "My Doc", Theme::Light);
        assert!(html.contains("<title>My Doc</title>"));
    }

    #[test]
    fn render_escapes_title_html() {
        let html = render_markdown("x", "<script>alert(1)</script>", Theme::Light);
        assert!(html.contains("&lt;script&gt;"));
        // The raw injected script should not appear as an executable tag in the title.
        assert!(!html.contains("<title><script>"));
        assert!(!html.contains("<script>alert(1)"));
    }

    #[test]
    fn render_includes_embedded_css() {
        let html = render_markdown("x", "t", Theme::Light);
        assert!(html.contains("<style>"));
    }

    #[test]
    fn render_light_theme_has_data_attribute() {
        let html = render_markdown("x", "t", Theme::Light);
        assert!(html.contains(r#"data-theme="light""#));
    }

    #[test]
    fn render_system_theme_has_data_attribute() {
        let html = render_markdown("x", "t", Theme::System);
        assert!(html.contains(r#"data-theme="system""#));
    }

    #[test]
    fn render_dark_theme_has_data_attribute() {
        let html = render_markdown("x", "t", Theme::Dark);
        assert!(html.contains(r#"data-theme="dark""#));
    }

    #[test]
    fn render_dark_theme_contains_dark_css() {
        let html = render_markdown("x", "t", Theme::Dark);
        assert!(html.contains(r#"<html lang="en" data-theme="dark" class="dark">"#));
    }

    #[test]
    fn render_includes_theme_toggle_controls() {
        let html = render_markdown("x", "t", Theme::System);
        assert!(html.contains("data-theme-option=\"system\""));
        assert!(html.contains("data-theme-option=\"light\""));
        assert!(html.contains("data-theme-option=\"dark\""));
        assert!(html.contains("mark-theme-option-icon"));
        assert!(html.contains("lucide lucide-monitor h-3.5 w-3.5"));
        assert!(html.contains("lucide lucide-sun h-3.5 w-3.5"));
        assert!(html.contains("lucide lucide-moon h-3.5 w-3.5"));
        assert!(html.contains(">System</span>"));
        assert!(html.contains(">Light</span>"));
        assert!(html.contains(">Dark</span>"));
    }

    #[test]
    fn render_includes_theme_and_sidebar_client_side_scripts() {
        let html = render_markdown("x", "t", Theme::System);
        assert!(html.contains("document.addEventListener('keydown'"));
        assert!(html.contains("isEditableTarget"));
        assert!(html.contains("setTheme(button.dataset.themeOption)"));
    }

    #[test]
    fn render_uses_template_shell_and_replaces_placeholders() {
        let html = render_markdown("# Hello\n\nWorld.", "hello", Theme::Light);
        assert!(html.contains("class=\"editor-shell\""), "{html}");
        assert!(html.contains("class=\"paper-sheet"), "{html}");
        assert!(!html.contains("Placeholder file tree"), "{html}");
        assert!(!html.contains("&lt;put rendered html here&gt;"), "{html}");
    }

    #[test]
    fn code_block_has_copy_button() {
        let md = "```\nsome code\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        assert!(
            html.contains("mark-copy-btn"),
            "copy button should be present"
        );
    }

    #[test]
    fn code_block_supported_lang_has_clean_button() {
        let md = "```rust\nfn main() {}\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        assert!(
            html.contains(r#"class="mark-btn mark-copy-clean-btn""#),
            "clean button element should be present for rust"
        );
    }

    #[test]
    fn code_block_unsupported_lang_no_clean_button() {
        let md = "```html\n<p>hello</p>\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        // Check the button element itself is absent (not just the class string,
        // which also appears inside the embedded JS).
        assert!(
            !html.contains(r#"class="mark-btn mark-copy-clean-btn""#),
            "clean button element should NOT be present for html"
        );
    }

    #[test]
    fn code_block_no_lang_no_clean_button() {
        let md = "```\nsome code\n```\n";
        let html = render_markdown(md, "t", Theme::Light);
        assert!(
            !html.contains(r#"class="mark-btn mark-copy-clean-btn""#),
            "clean button element should NOT be present when no language specified"
        );
    }

    // ── extract_local_md_links tests ─────────────────────────────────────────

    #[test]
    fn extract_links_finds_local_md() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Create a target file so canonicalize succeeds.
        let target = dir.path().join("chapter.md");
        std::fs::write(&target, "# Chapter").expect("write");

        let md = "[Chapter](chapter.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0, "chapter.md");
        assert_eq!(links[0].1, target.canonicalize().unwrap());
    }

    #[test]
    fn extract_links_skips_external_urls() {
        let dir = tempfile::tempdir().expect("tempdir");
        let md = "[Google](https://google.com)\n[Local](http://localhost/foo.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "external links must not be extracted");
    }

    #[test]
    fn extract_links_skips_pure_anchors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let md = "[Section](#heading)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "pure anchor links must not be extracted");
    }

    #[test]
    fn extract_links_skips_non_md_extensions() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Create a real file so the only filter is the extension.
        let pdf = dir.path().join("report.pdf");
        std::fs::write(&pdf, "data").expect("write");
        let md = "[Report](report.pdf)\n[Image](photo.png)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "non-.md links must not be extracted");
    }

    #[test]
    fn extract_links_deduplicates_same_target() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("api.md");
        std::fs::write(&target, "# API").expect("write");
        let md = "[API](api.md)\n[API again](api.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert_eq!(links.len(), 1, "duplicate links must appear only once");
    }

    #[test]
    fn extract_links_strips_fragment_for_key() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("api.md");
        std::fs::write(&target, "# API").expect("write");
        let md = "[Section](api.md#endpoints)\n";
        let links = extract_local_md_links(md, dir.path());
        assert_eq!(links.len(), 1);
        // The key must be the base without the fragment.
        assert_eq!(links[0].0, "api.md");
    }

    #[test]
    fn extract_links_ignores_nonexistent_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        // No actual file created — canonicalize will fail.
        let md = "[Ghost](ghost.md)\n";
        let links = extract_local_md_links(md, dir.path());
        assert!(links.is_empty(), "nonexistent files must not be extracted");
    }

    // ── render_markdown_rewriting_links tests ─────────────────────────────────

    #[test]
    fn rewrite_links_replaces_md_href() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("api.md");
        std::fs::write(&target, "# API").expect("write");

        let html_path = dir.path().join("api-123-abcd1234.html");
        let mut link_map = HashMap::new();
        link_map.insert("api.md".to_string(), html_path.clone());

        let md = "[API](api.md)\n";
        let html = render_markdown_rewriting_links(
            md,
            "t",
            Theme::Light,
            &link_map,
            chrome(&[], &[], dir.path(), false),
        );
        let expected = format!("href=\"{}\"", html_path.display());
        assert!(
            html.contains(&expected),
            "expected rewritten href in html:\n{html}"
        );
        assert!(
            !html.contains("href=\"api.md\""),
            "original .md href must not survive"
        );
    }

    #[test]
    fn rewrite_links_preserves_fragment() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html_path = dir.path().join("api-123-abcd1234.html");
        let mut link_map = HashMap::new();
        link_map.insert("api.md".to_string(), html_path.clone());

        let md = "[Section](api.md#endpoints)\n";
        let html = render_markdown_rewriting_links(
            md,
            "t",
            Theme::Light,
            &link_map,
            chrome(&[], &[], dir.path(), false),
        );
        let expected = format!("href=\"{}#endpoints\"", html_path.display());
        assert!(
            html.contains(&expected),
            "fragment must be preserved:\n{html}"
        );
    }

    #[test]
    fn rewrite_links_leaves_external_urls_unchanged() {
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let md = "[Google](https://google.com)\n";
        let html = render_markdown_rewriting_links(
            md,
            "t",
            Theme::Light,
            &link_map,
            chrome(&[], &[], Path::new(""), false),
        );
        assert!(
            html.contains("href=\"https://google.com\""),
            "external URL must not be modified:\n{html}"
        );
    }

    #[test]
    fn rewrite_links_leaves_non_md_local_links_unchanged() {
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let md = "[Image](photo.png)\n";
        let html = render_markdown_rewriting_links(
            md,
            "t",
            Theme::Light,
            &link_map,
            chrome(&[], &[], Path::new(""), false),
        );
        assert!(
            html.contains("href=\"photo.png\""),
            "non-md link must not be modified:\n{html}"
        );
    }

    #[test]
    fn rewrite_links_rewrites_image_sources_when_present_in_map() {
        let dir = tempfile::tempdir().expect("tempdir");
        let asset_path = dir.path().join("assets/logo.png");
        let mut link_map = HashMap::new();
        link_map.insert("logo.png".to_string(), asset_path.clone());

        let md = "![Logo](logo.png)\n";
        let html = render_markdown_rewriting_links(
            md,
            "t",
            Theme::Light,
            &link_map,
            chrome(&[], &[], dir.path(), false),
        );
        let expected = format!("src=\"{}\"", asset_path.display());
        assert!(html.contains(&expected), "{html}");
    }

    #[test]
    fn rewrite_links_empty_map_is_identity() {
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let md = "[Chapter](chapter.md)\n";
        let html_rewrite = render_markdown_rewriting_links(
            md,
            "t",
            Theme::Light,
            &link_map,
            chrome(&[], &[], Path::new(""), false),
        );
        let html_plain = render_markdown(md, "t", Theme::Light);
        assert_eq!(
            html_rewrite, html_plain,
            "empty link_map must produce identical output to render_markdown"
        );
    }

    // ── extract_local_asset_links tests ──────────────────────────────────────

    #[test]
    fn asset_links_finds_local_non_md_link() {
        let dir = tempfile::tempdir().expect("tempdir");
        let asset = dir.path().join("data.csv");
        std::fs::write(&asset, "a,b,c").expect("write");

        let md = "[Data](data.csv)\n";
        let links = extract_local_asset_links(md, dir.path());
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0, "data.csv");
        assert_eq!(links[0].1, std::fs::canonicalize(&asset).unwrap());
    }

    #[test]
    fn asset_links_finds_image_target() {
        let dir = tempfile::tempdir().expect("tempdir");
        let img = dir.path().join("logo.png");
        std::fs::write(&img, b"\x89PNG").expect("write");

        let md = "![Logo](logo.png)\n";
        let links = extract_local_asset_links(md, dir.path());
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0, "logo.png");
    }

    #[test]
    fn asset_links_excludes_external_urls() {
        let dir = tempfile::tempdir().expect("tempdir");
        let md = "[Google](https://google.com/file.pdf)\n";
        let links = extract_local_asset_links(md, dir.path());
        assert!(links.is_empty(), "external URLs must be excluded");
    }

    #[test]
    fn asset_links_excludes_md_links() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("readme.md");
        std::fs::write(&target, "# Hi").expect("write");

        let md = "[Readme](readme.md)\n";
        let links = extract_local_asset_links(md, dir.path());
        assert!(
            links.is_empty(),
            ".md links must be excluded from asset extractor"
        );
    }

    #[test]
    fn asset_links_skips_missing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let md = "[Ghost](ghost.txt)\n";
        let links = extract_local_asset_links(md, dir.path());
        assert!(links.is_empty(), "nonexistent files must not be extracted");
    }

    #[test]
    fn asset_links_deduplicates_by_base_url() {
        let dir = tempfile::tempdir().expect("tempdir");
        let asset = dir.path().join("spec.txt");
        std::fs::write(&asset, "content").expect("write");

        // Same base URL linked twice – should deduplicate.
        let md = "[A](spec.txt)\n[B](spec.txt)\n";
        let links = extract_local_asset_links(md, dir.path());
        assert_eq!(links.len(), 1, "duplicate base URLs must be deduplicated");
    }

    #[test]
    fn asset_links_preserves_fragment_in_key() {
        let dir = tempfile::tempdir().expect("tempdir");
        let asset = dir.path().join("notes.txt");
        std::fs::write(&asset, "hello").expect("write");

        let md = "[Notes](notes.txt#section1)\n";
        let links = extract_local_asset_links(md, dir.path());
        assert_eq!(links.len(), 1);
        // The key must be the base URL (fragment stripped) so that
        // render_markdown_rewriting_links can look it up.
        assert_eq!(links[0].0, "notes.txt");
    }

    #[test]
    fn asset_links_rejects_path_traversal() {
        // File exists in parent dir but must not be collected.
        let parent = tempfile::tempdir().expect("tempdir");
        let secret = parent.path().join("secret.txt");
        std::fs::write(&secret, "top secret").expect("write");

        let child = parent.path().join("sub");
        std::fs::create_dir(&child).expect("mkdir");

        let md = "[Escape](../secret.txt)\n";
        let links = extract_local_asset_links(md, &child);
        assert!(
            links.is_empty(),
            "path traversal outside source_dir must be rejected"
        );
    }

    // ── breadcrumb + sidebar tests ────────────────────────────────────────────

    #[test]
    fn no_breadcrumb_on_entry_point() {
        // Empty breadcrumb slice → no breadcrumb nav element in output.
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Entry",
            "entry",
            Theme::Light,
            &link_map,
            chrome(&[], &[], Path::new(""), false), // no breadcrumb
        );
        assert!(
            !html.contains("aria-label=\"Breadcrumb\""),
            "entry-point must not have breadcrumb nav element:\n{html}"
        );
    }

    #[test]
    fn breadcrumb_present_on_depth_one_page() {
        let dir = tempfile::tempdir().expect("tempdir");
        let entry_html = dir.path().join("entry-abc.html");
        let breadcrumb = vec![("entry".to_string(), entry_html.clone())];
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Chapter",
            "chapter",
            Theme::Light,
            &link_map,
            chrome(&breadcrumb, &[], Path::new(""), false),
        );
        assert!(
            html.contains("aria-label=\"Breadcrumb\""),
            "depth-1 page must have breadcrumb:\n{html}"
        );
        let href = format!("href=\"{}\"", entry_html.display());
        assert!(
            html.contains(&href),
            "breadcrumb must link to entry html:\n{html}"
        );
        assert!(
            html.contains("mark-breadcrumb-current"),
            "current page must appear as non-link span:\n{html}"
        );
    }

    #[test]
    fn sidebar_present_when_all_files_provided() {
        let dir = tempfile::tempdir().expect("tempdir");
        let entry_html = dir.path().join("entry-abc.html");
        let chapter_html = dir.path().join("chapter-abc.html");
        let all_files = vec![
            ("entry".to_string(), entry_html.clone(), false),
            ("chapter".to_string(), chapter_html.clone(), true),
        ];
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Chapter",
            "chapter",
            Theme::Light,
            &link_map,
            chrome(&[], &all_files, dir.path(), false),
        );
        assert!(
            html.contains("id=\"mark-sidebar\""),
            "sidebar must be present when all_files is non-empty:\n{html}"
        );
        assert!(
            html.contains("id=\"mark-sidebar-button\""),
            "sidebar toggle button must be present:\n{html}"
        );
        assert!(
            html.contains("mark-sidebar-current"),
            "current page must be highlighted in sidebar:\n{html}"
        );
        assert!(
            html.contains("aria-label=\"Rendered file tree\""),
            "sidebar tree list must be present:\n{html}"
        );
        // Non-current pages must be links.
        let entry_href = format!("href=\"{}\"", entry_html.display());
        assert!(
            html.contains(&entry_href),
            "non-current entry must be a link in sidebar:\n{html}"
        );
        assert!(
            html.contains("title=\"Toggle sidebar (e)\""),
            "sidebar toggle should advertise the e hotkey:\n{html}"
        );
    }

    #[test]
    fn sidebar_is_hidden_by_default_when_not_requested_visible() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html_path = dir.path().join("entry-abc.html");
        let all_files = vec![("entry".to_string(), html_path, true)];
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Entry",
            "entry",
            Theme::System,
            &link_map,
            chrome(&[], &all_files, dir.path(), false),
        );
        assert!(
            html.contains("aria-expanded=\"false\""),
            "sidebar button should report collapsed state when hidden by default:\n{html}"
        );
        assert!(
            !html.contains("id=\"mark-sidebar-toggle\""),
            "stray sidebar-toggle checkbox must not be present:\n{html}"
        );
    }

    #[test]
    fn sidebar_can_start_visible_from_config_default() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html_path = dir.path().join("entry-abc.html");
        let all_files = vec![("entry".to_string(), html_path, true)];
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Entry",
            "entry",
            Theme::System,
            &link_map,
            chrome(&[], &all_files, dir.path(), true),
        );
        assert!(
            html.contains("aria-expanded=\"true\""),
            "sidebar button should report expanded state when sidebar starts visible:\n{html}"
        );
        assert!(
            !html.contains("id=\"mark-sidebar-toggle\""),
            "stray sidebar-toggle checkbox must not be present:\n{html}"
        );
    }

    #[test]
    fn sidebar_absent_when_all_files_empty() {
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Solo",
            "solo",
            Theme::Light,
            &link_map,
            chrome(&[], &[], Path::new(""), false),
        );
        assert!(
            !html.contains("id=\"mark-sidebar\""),
            "no sidebar nav element when all_files is empty:\n{html}"
        );
    }

    #[test]
    fn sidebar_present_on_dark_theme() {
        let dir = tempfile::tempdir().expect("tempdir");
        let html_path = dir.path().join("entry-abc.html");
        let all_files = vec![("entry".to_string(), html_path.clone(), true)];
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Entry",
            "entry",
            Theme::Dark,
            &link_map,
            chrome(&[], &all_files, dir.path(), false),
        );
        assert!(
            html.contains("id=\"mark-sidebar\""),
            "sidebar must be present in dark theme:\n{html}"
        );
        assert!(
            html.contains(r#"<html lang="en" data-theme="dark" class="dark">"#),
            "dark theme root class must be present:\n{html}"
        );
    }

    #[test]
    fn build_sidebar_tree_groups_nested_directories() {
        let run_dir = PathBuf::from("/rendered/overview-123-abcdef12");
        let all_files = vec![
            ("overview".to_string(), run_dir.join("overview.html"), false),
            (
                "intro".to_string(),
                run_dir.join("chapters/intro.html"),
                false,
            ),
            (
                "endpoints".to_string(),
                run_dir.join("chapters/api/endpoints.html"),
                true,
            ),
        ];

        let tree = build_sidebar_tree(&all_files, &run_dir);
        assert_eq!(tree.len(), 2, "root should contain file + top-level dir");
        assert_eq!(tree[0].name, "overview");
        assert_eq!(tree[0].path, Some(run_dir.join("overview.html")));
        assert_eq!(tree[1].name, "chapters");
        assert!(tree[1].path.is_none());
        assert_eq!(tree[1].children[0].name, "intro");
        assert_eq!(tree[1].children[1].name, "api");
        assert_eq!(tree[1].children[1].children[0].name, "endpoints");
        assert!(tree[1].children[1].children[0].is_current);
    }

    #[test]
    fn build_sidebar_tree_sorts_files_before_directories_recursively() {
        let run_dir = PathBuf::from("/rendered/overview-123-abcdef12");
        let all_files = vec![
            (
                "zeta".to_string(),
                run_dir.join("chapters/zeta.html"),
                false,
            ),
            ("overview".to_string(), run_dir.join("overview.html"), false),
            (
                "alpha".to_string(),
                run_dir.join("chapters/alpha.html"),
                false,
            ),
            (
                "appendix".to_string(),
                run_dir.join("appendix/readme.html"),
                false,
            ),
            ("beta".to_string(), run_dir.join("beta.html"), true),
        ];

        let tree = build_sidebar_tree(&all_files, &run_dir);
        assert_eq!(tree[0].name, "beta");
        assert_eq!(tree[1].name, "overview");
        assert_eq!(tree[2].name, "appendix");
        assert_eq!(tree[3].name, "chapters");
        assert_eq!(tree[3].children[0].name, "alpha");
        assert_eq!(tree[3].children[1].name, "zeta");
    }

    #[test]
    fn sidebar_renders_collapsible_directories() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let all_files = vec![
            ("overview".to_string(), run_dir.join("overview.html"), false),
            (
                "intro".to_string(),
                run_dir.join("chapters/intro.html"),
                true,
            ),
        ];
        let link_map: HashMap<String, PathBuf> = HashMap::new();

        let html = render_markdown_rewriting_links(
            "# Intro",
            "intro",
            Theme::Light,
            &link_map,
            chrome(&[], &all_files, &run_dir, false),
        );

        assert!(
            html.contains("class=\"mark-sidebar-dir mark-sidebar-group\""),
            "got: {html}"
        );
        assert!(html.contains("<details"), "got: {html}");
        assert!(html.contains(">chapters/</summary>"), "got: {html}");
    }

    #[test]
    fn c_hotkey_calls_toggle_config_menu() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("toggleConfigMenu"),
            "c hotkey handler must reference toggleConfigMenu:\n{html}"
        );
        assert!(
            !html.contains("openConfigMenu"),
            "openConfigMenu must be replaced by toggleConfigMenu:\n{html}"
        );
    }

    #[test]
    fn save_button_is_disabled_by_default() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("id=\"mark-save-layout\""),
            "save button must be present in config pane:\n{html}"
        );
        assert!(
            html.contains("id=\"mark-save-layout\" class=\"mark-save-layout-btn\""),
            "save button must be present with correct class:\n{html}"
        );
        assert!(
            html.contains("disabled>Save</button>"),
            "save button must be disabled by default:\n{html}"
        );
    }

    #[test]
    fn config_pane_has_terminal_command_accordion() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("class=\"mark-layout-command-accordion\""),
            "config pane must contain accordion details element:\n{html}"
        );
        assert!(
            html.contains("class=\"mark-layout-command-summary\""),
            "accordion must have a summary element:\n{html}"
        );
        assert!(
            !html.contains("<details class=\"mark-layout-command-accordion\" open"),
            "accordion must be collapsed by default (no open attribute):\n{html}"
        );
    }

    #[test]
    fn config_pane_keeps_copy_button_in_accordion_summary() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains(
                "<summary class=\"mark-layout-command-summary\"><span class=\"mark-layout-command-summary-label\">Terminal command</span><button type=\"button\" id=\"mark-copy-layout-command\" class=\"mark-layout-copy\">Copy</button></summary>"
            ),
            "copy button must remain visible in accordion summary:\n{html}"
        );
    }

    #[test]
    fn rendered_shell_includes_pdf_export_button_and_hook() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("id=\"mark-export-pdf\""),
            "pdf export button must be rendered:\n{html}"
        );
        assert!(
            html.contains("window.showSaveFilePicker"),
            "pdf export flow must use file picker when available:\n{html}"
        );
        assert!(
            html.contains("window.print()"),
            "pdf export flow must trigger browser print:\n{html}"
        );
        assert!(
            html.contains("primaryModifier && event.shiftKey && key === 'e'"),
            "pdf export hotkey must use an os-agnostic primary modifier shortcut:\n{html}"
        );
        assert!(
            !html.contains("setTheme(nextTheme);\n    closeThemeMenu();"),
            "theme hotkey must not close config pane:\n{html}"
        );
    }

    #[test]
    fn config_menu_lists_hotkeys() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(html.contains(">Hotkeys</div>"), "{html}");
        assert!(html.contains("<kbd>Shift</kbd><span>+</span><kbd>Primary</kbd><span>+</span><kbd>E</kbd><span>Export PDF</span>"), "{html}");
    }

    #[test]
    fn config_menu_renders_as_right_sidebar_panel() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(html.contains("<aside id=\"mark-theme-menu\""), "{html}");
        assert!(
            html.contains(".mark-theme-menu{position:fixed;top:0;right:0;z-index:60;"),
            "{html}"
        );
        assert!(html.contains("class=\"mark-theme-menu-inner\""), "{html}");
    }

    #[test]
    fn sidebar_footer_is_not_rendered() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let all_files = vec![("overview".to_string(), run_dir.join("overview.html"), true)];
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Overview",
            "overview",
            Theme::Light,
            &link_map,
            chrome(&[], &all_files, &run_dir, false),
        );
        assert!(!html.contains("mark-sidebar-footer"), "{html}");
    }

    #[test]
    fn sidebar_has_search_input() {
        let dir = tempfile::tempdir().expect("tempdir");
        let run_dir = dir.path().join("overview-123-abcdef12");
        let all_files = vec![("overview".to_string(), run_dir.join("overview.html"), true)];
        let link_map: HashMap<String, PathBuf> = HashMap::new();
        let html = render_markdown_rewriting_links(
            "# Overview",
            "overview",
            Theme::Light,
            &link_map,
            chrome(&[], &all_files, &run_dir, false),
        );
        assert!(
            html.contains("id=\"mark-sidebar-search\""),
            "sidebar must have search input:\n{html}"
        );
        assert!(
            html.contains("type=\"search\""),
            "search input must be type=search:\n{html}"
        );
    }

    #[test]
    fn zen_mode_hotkey_wiring_present() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("toggleZenMode"),
            "z hotkey handler must reference toggleZenMode:\n{html}"
        );
        assert!(
            html.contains("key === 'z'"),
            "z key must be handled:\n{html}"
        );
    }

    #[test]
    fn hotkey_list_contains_zen_mode_entry() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("<kbd>Z</kbd><span>Zen mode</span>"),
            "Z hotkey must appear in hotkey list:\n{html}"
        );
    }

    #[test]
    fn zen_mode_background_uses_synced_page_variable() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains(
                "html.mark-zen-mode,html.mark-zen-mode body{background:var(--mark-zen-bg,var(--card))}"
            ),
            "zen mode background must use the synced page variable:\n{html}"
        );
        assert!(
            html.contains(
                "html.mark-zen-mode .paper-sheet{border:none;background:var(--mark-zen-bg,var(--card));box-shadow:none;border-radius:0}"
            ),
            "zen mode paper must adopt the same background so the page becomes the letter:\n{html}"
        );
    }

    #[test]
    fn zen_mode_toggle_reads_current_page_background_and_clears_it() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("window.getComputedStyle(page).getPropertyValue('background-color')"),
            "zen mode must read the current paper background before toggling:\n{html}"
        );
        assert!(
            html.contains("root.style.setProperty('--mark-zen-bg'"),
            "zen mode must store the synced background color on the root element:\n{html}"
        );
        assert!(
            html.contains("document.documentElement.style.removeProperty('--mark-zen-bg');"),
            "zen mode must clear the synced background when disabled:\n{html}"
        );
    }

    #[test]
    fn theme_changes_resync_zen_mode_background() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("if (root.classList.contains('mark-zen-mode')) {\n      syncZenModeBackground();\n    }"),
            "theme changes must resync zen mode background:\n{html}"
        );
    }

    #[test]
    fn export_hotkey_shows_shift_before_primary() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains(
                "<kbd>Shift</kbd><span>+</span><kbd>Primary</kbd><span>+</span><kbd>E</kbd>"
            ),
            "export hotkey must show Shift before Primary:\n{html}"
        );
        assert!(
            !html.contains("<kbd>Primary</kbd><span>+</span><kbd>Shift</kbd>"),
            "old Primary+Shift order must not appear:\n{html}"
        );
    }

    #[test]
    fn set_theme_writes_to_session_storage() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("sessionStorage.setItem('mark-theme'"),
            "setTheme must write to sessionStorage:\n{html}"
        );
    }

    #[test]
    fn page_load_reads_session_storage_for_theme() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("sessionStorage.getItem('mark-theme')"),
            "early theme script must read from sessionStorage:\n{html}"
        );
    }

    #[test]
    fn save_button_appears_before_accordion_in_html() {
        let html = render_markdown("# Test", "test", Theme::System);
        let save_pos = html.find("id=\"mark-save-layout\"").expect("save button");
        let accordion_pos = html
            .find("class=\"mark-layout-command-accordion\"")
            .expect("accordion");
        assert!(
            save_pos < accordion_pos,
            "save button must appear before accordion in HTML:\nsave at {save_pos}, accordion at {accordion_pos}"
        );
    }

    #[test]
    fn reader_layout_help_paragraph_is_absent() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            !html.contains("Adjust the values below"),
            "reader-layout help paragraph must be removed:\n{html}"
        );
    }

    #[test]
    fn e_hotkey_label_reads_toggle_hierarchy() {
        let html = render_markdown("# Test", "test", Theme::System);
        assert!(
            html.contains("<kbd>E</kbd><span>Toggle hierarchy</span>"),
            "E hotkey must say Toggle hierarchy:\n{html}"
        );
        assert!(
            !html.contains("<kbd>E</kbd><span>Toggle sidebar</span>"),
            "old E hotkey label must be removed:\n{html}"
        );
    }
}
