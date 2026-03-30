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

const PAGE_TEMPLATE: &str = include_str!("index.html");
const TEMPLATE_TITLE: &str = "<title>Mark HTML Template</title>";
const CONTENT_PLACEHOLDER: &str =
    r#"<div class="markdown-prose"><p>&lt;put rendered html here&gt;</p></div>"#;
const LEFT_CONTROL_START: &str = r#"<div class="fixed left-4 top-4 z-40 md:left-6 md:top-6">"#;
const RIGHT_CONTROL_START: &str = r#"<div class="fixed right-4 top-4 z-40 md:right-6 md:top-6">"#;
const ASIDE_START: &str = r#"<aside class="editor-scrollbar"#;
const MAIN_START: &str =
    r#"<main class="relative flex min-h-screen items-center justify-center px-4 py-24 md:px-6">"#;

fn inject_before(document: &str, marker: &str, injection: &str) -> String {
    let Some(index) = document.find(marker) else {
        return document.to_string();
    };

    let mut output = String::with_capacity(document.len() + injection.len());
    output.push_str(&document[..index]);
    output.push_str(injection);
    output.push_str(&document[index..]);
    output
}

fn replace_range(
    document: &str,
    start_marker: &str,
    end_marker: &str,
    replacement: &str,
) -> String {
    let Some(start) = document.find(start_marker) else {
        return document.to_string();
    };
    let Some(end_offset) = document[start..].find(end_marker) else {
        return document.to_string();
    };
    let end = start + end_offset;

    let mut output =
        String::with_capacity(document.len() + replacement.len().saturating_sub(end - start));
    output.push_str(&document[..start]);
    output.push_str(replacement);
    output.push_str(&document[end..]);
    output
}

fn render_breadcrumb_html(title: &str, breadcrumb: &[(String, PathBuf)]) -> String {
    if breadcrumb.is_empty() {
        return String::new();
    }

    let mut html = String::from(
        r#"<nav class="mark-breadcrumb mb-6 flex flex-wrap items-center gap-2 text-sm text-[var(--muted-foreground)]" aria-label="Breadcrumb">"#,
    );
    for (name, path) in breadcrumb {
        let href = escape_html(&path.to_string_lossy());
        let display = escape_html(name);
        html.push_str(&format!(
            r#"<a class="transition-colors hover:text-[var(--foreground)] hover:underline" href="{href}">{display}</a><span class="mark-breadcrumb-sep">&rsaquo;</span>"#
        ));
    }
    html.push_str(&format!(
        r#"<span class="mark-breadcrumb-current font-medium text-[var(--foreground)]">{}</span></nav>"#,
        escape_html(title)
    ));
    html
}

fn render_sidebar_controls(sidebar_visible: bool) -> String {
    let checked = if sidebar_visible { " checked" } else { "" };
    let expanded = if sidebar_visible { "true" } else { "false" };
    format!(
        r#"<input type="checkbox" id="mark-sidebar-toggle" class="mark-sidebar-toggle"{checked}>
<div class="absolute left-4 top-4 z-40 md:left-6 md:top-6"><div id="mark-sidebar-control-shell" class="mark-sidebar-button-shell rounded-full border border-[var(--border)] bg-[color-mix(in_srgb,var(--card)_88%,transparent)] p-1.5 shadow-sm backdrop-blur"><button class="inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--background)] text-[var(--muted-foreground)] hover:bg-[var(--muted)] hover:text-[var(--foreground)] h-8 w-8 rounded-full p-0" type="button" id="mark-sidebar-button" aria-controls="mark-sidebar" aria-expanded="{expanded}" aria-label="Toggle sidebar (e)" title="Toggle sidebar (e)"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-panel-left h-3.5 w-3.5" aria-hidden="true"><rect width="18" height="18" x="3" y="3" rx="2"></rect><path d="M9 3v18"></path></svg></button></div></div>"#,
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

fn render_theme_controls(theme_attr: &str, appearance: AppearanceConfig) -> String {
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
        r#"<div class="fixed right-4 top-4 z-40 md:right-6 md:top-6"><div class="mark-theme-control relative rounded-full border border-[var(--border)] bg-[color-mix(in_srgb,var(--card)_88%,transparent)] p-1.5 shadow-sm backdrop-blur mark-theme-button-shell"><button class="inline-flex items-center justify-center gap-2 whitespace-nowrap text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--ring)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--background)] text-[var(--muted-foreground)] hover:bg-[var(--muted)] hover:text-[var(--foreground)] h-8 w-8 rounded-full p-0" type="button" id="mark-theme-toggle" aria-label="Theme: {theme_attr}. Change theme or layout" title="Theme: {theme_attr}. Change theme or layout" aria-haspopup="dialog" aria-expanded="false"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-sun-moon h-3.5 w-3.5" aria-hidden="true"><path d="M12 2v2"></path><path d="M14.837 16.385a6 6 0 1 1-7.223-7.222c.624-.147.97.66.715 1.248a4 4 0 0 0 5.26 5.259c.589-.255 1.396.09 1.248.715"></path><path d="M16 12a4 4 0 0 0-4-4"></path><path d="m19 5-1.256 1.256"></path><path d="M20 12h2"></path></svg></button><div id="mark-theme-menu" class="mark-theme-menu" aria-label="Theme and reader layout controls" hidden><section class="mark-theme-menu-section"><div class="mark-theme-menu-heading">Theme</div><div class="mark-theme-option-list">{theme_menu}</div></section><form id="mark-layout-form" class="mark-layout-form" autocomplete="off"><div class="mark-theme-menu-heading">Reader layout</div><p class="mark-layout-help">Adjust the values below, then run the generated command in your terminal to persist them to <code>~/.mark/config.toml</code>.</p><label class="mark-layout-field"><span>Font size (px)</span><input id="mark-font-size-input" type="number" min="10" max="32" step="1" value="{font_size}"></label><label class="mark-layout-field"><span>Letter width (in)</span><input id="mark-letter-width-input" type="number" min="5" max="12" step="0.05" value="{letter_width}"></label><label class="mark-layout-field"><span>Letter corner radius (px)</span><input id="mark-letter-radius-input" type="number" min="0" max="64" step="1" value="{letter_radius}"></label><label class="mark-layout-field"><span>Sidebar button radius (px)</span><input id="mark-sidebar-button-radius-input" type="number" min="0" max="999" step="1" value="{sidebar_button_radius}"></label><label class="mark-layout-field"><span>Theme button radius (px)</span><input id="mark-theme-button-radius-input" type="number" min="0" max="999" step="1" value="{theme_button_radius}"></label><div class="mark-layout-command-wrap"><div class="mark-layout-command-header"><span>Terminal command</span><button type="button" id="mark-copy-layout-command" class="mark-layout-copy">Copy</button></div><code id="mark-layout-command" class="mark-layout-command">{layout_command}</code></div></form></div></div></div>"#,
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
    let mut nav = String::from(r#"<div class="space-y-2">"#);
    render_sidebar_nodes(&tree, &mut nav);
    nav.push_str("</div>");

    format!(
        r#"<aside id="mark-sidebar" class="mark-sidebar fixed inset-y-0 left-0 z-30 w-[22rem] overflow-hidden border-r border-[var(--border)] bg-[var(--card)] p-4 pt-20 shadow-lg transition-transform duration-200 md:p-6 md:pt-24{hidden_class}"><div class="relative flex h-full min-h-0 flex-col"><section class="mark-sidebar-scroll editor-scrollbar flex-1 overflow-y-auto"><h2 class="text-xs font-medium uppercase tracking-wide text-[var(--muted-foreground)]">Hierarchy</h2><nav aria-label="Rendered file tree" class="mt-4 text-sm">{nav}</nav></section><footer class="mark-sidebar-footer text-xs text-[var(--muted-foreground)]">Hotkeys: <span>E</span> toggles the sidebar,<br><span>T</span> toggles the theme.</footer></div></aside>"#,
    )
}

/// Wrap a rendered HTML body in the checked-in page template with embedded JS.
///
/// `breadcrumb` is an ordered list of `(display_name, html_path)` ancestors
/// from the entry-point down to (but not including) the current file. Empty for
/// the entry-point itself.
///
/// `all_files` is the full list of `(display_name, html_path, is_current)`
/// entries for the sidebar, in BFS discovery order.
fn build_html_document(title: &str, body: &str, theme: Theme, chrome: RenderChrome<'_>) -> String {
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
:root{{--mark-font-size:{font_size}px;--mark-letter-width:{letter_width}in;--mark-letter-radius:{letter_radius}px;--mark-sidebar-button-radius:{sidebar_button_radius}px;--mark-theme-button-radius:{theme_button_radius}px;--mark-sidebar-footer-height:4.85rem}}
.mark-sidebar-toggle{{display:none}}
.mark-breadcrumb{{border-bottom:1px solid var(--border);padding-bottom:1rem}}
.mark-breadcrumb a{{text-decoration:none}}
.mark-page-width-shell{{max-width:calc(var(--mark-letter-width) + 4rem)!important}}
.paper-sheet{{max-width:var(--mark-letter-width)!important;border-radius:var(--mark-letter-radius)!important;font-size:var(--mark-font-size)}}
#mark-sidebar-control-shell,#mark-sidebar-button{{border-radius:var(--mark-sidebar-button-radius)!important}}
.mark-theme-button-shell,#mark-theme-toggle{{border-radius:var(--mark-theme-button-radius)!important}}
.mark-theme-menu{{position:absolute;right:0;top:calc(100% + .75rem);display:grid;gap:1rem;min-width:min(26rem,calc(100vw - 2rem));padding:1rem;border:1px solid var(--border);border-radius:1rem;background:color-mix(in srgb,var(--card) 96%,transparent);box-shadow:0 12px 30px #0000001a;backdrop-filter:blur(18px)}}
.mark-theme-menu[hidden]{{display:none}}
.mark-theme-menu-section{{display:grid;gap:.65rem}}
.mark-theme-menu-heading{{font-size:.72rem;font-weight:700;letter-spacing:.08em;text-transform:uppercase;color:var(--muted-foreground)}}
.mark-theme-option-list{{display:grid;gap:.25rem}}
.mark-theme-option{{display:flex;align-items:center;gap:.65rem;width:100%;padding:.55rem .7rem;border-radius:.75rem;background:transparent;color:var(--foreground);cursor:pointer;text-align:left}}
.mark-theme-option:hover,.mark-theme-option[aria-pressed="true"]{{background:var(--muted)}}
.mark-theme-option-icon,.mark-theme-option-label{{pointer-events:none}}
.mark-theme-option-icon{{display:inline-flex;align-items:center;justify-content:center;color:var(--muted-foreground)}}
.mark-layout-form{{display:grid;gap:.75rem;padding-top:.2rem}}
.mark-layout-help{{margin:0;color:var(--muted-foreground);font-size:.85rem;line-height:1.45}}
.mark-layout-help code{{font-size:.82em}}
.mark-layout-field{{display:grid;gap:.35rem}}
.mark-layout-field span{{font-size:.82rem;color:var(--foreground)}}
.mark-layout-field input{{width:100%;border:1px solid var(--border);border-radius:.7rem;background:var(--background);color:var(--foreground);padding:.6rem .75rem;font:inherit}}
.mark-layout-field input:focus{{outline:2px solid var(--ring);outline-offset:2px}}
.mark-layout-command-wrap{{display:grid;gap:.5rem;border:1px solid var(--border);border-radius:.85rem;background:var(--muted);padding:.75rem}}
.mark-layout-command-header{{display:flex;align-items:center;justify-content:space-between;gap:1rem;font-size:.8rem;font-weight:600;color:var(--foreground)}}
.mark-layout-copy{{border:1px solid var(--border);border-radius:.65rem;background:var(--background);color:var(--foreground);padding:.35rem .65rem;font:inherit;cursor:pointer}}
.mark-layout-copy:hover{{background:var(--card)}}
.mark-layout-command{{display:block;white-space:pre-wrap;word-break:break-word;font-size:.78rem;line-height:1.5}}
.mark-sidebar-link,.mark-sidebar-current,.mark-sidebar-summary{{display:flex;min-height:2.5rem;align-items:center}}
.mark-sidebar-group>summary::-webkit-details-marker{{display:none}}
.mark-sidebar-group>summary::before{{content:"▾";display:inline-block;margin-right:.5rem;color:var(--muted-foreground);transition:transform .15s ease}}
.mark-sidebar-group:not([open])>summary::before{{transform:rotate(-90deg)}}
.mark-sidebar-summary{{cursor:pointer;list-style:none}}
.mark-sidebar-current{{background:var(--muted)}}
.mark-sidebar-scroll{{padding-bottom:calc(var(--mark-sidebar-footer-height) + 1rem)}}
.mark-sidebar-footer{{position:absolute;left:0;right:0;bottom:0;z-index:10;border-top:1px solid var(--border);background:color-mix(in srgb,var(--card) 97%,transparent);padding:1rem 1.5rem 1.15rem;backdrop-filter:blur(14px);box-shadow:0 -10px 24px #00000014;line-height:1.45}}
.mark-sidebar-footer span{{font-weight:600;color:var(--foreground)}}
.mark-code-block{{position:relative;margin:1em 0}}
.mark-code-toolbar{{display:flex;justify-content:flex-end;gap:.4em;padding:.45rem .6rem;background:var(--muted);border:1px solid var(--border);border-bottom:none;border-radius:.5rem .5rem 0 0}}
.mark-code-block pre{{margin-top:0;border-top-left-radius:0;border-top-right-radius:0}}
.mark-btn{{font-size:.78em;padding:.2em .6em;border:1px solid var(--border);border-radius:.4rem;background:var(--background);color:var(--foreground);cursor:pointer;transition:background .15s}}
.mark-btn:hover{{background:var(--card)}}
.mark-btn.mark-copied{{color:#2a7a2a;border-color:#2a7a2a}}
.mark-btn.mark-failed{{color:#b00;border-color:#b00}}
</style>"#,
        font_size = chrome.appearance.font_size_px,
        letter_width = format_decimal(chrome.appearance.letter_width_in),
        letter_radius = chrome.appearance.letter_radius_px,
        sidebar_button_radius = chrome.appearance.sidebar_button_radius_px,
        theme_button_radius = chrome.appearance.theme_button_radius_px,
    );

    let early_theme_script = r#"<script>(function(){var root=document.documentElement;var theme=root.getAttribute('data-theme')||'system';var dark=theme==='dark'||(theme==='system'&&window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches);root.classList.toggle('dark',dark);})();</script>"#;

    let enhancement_js = r#"<script>(function() {
  var THEMES = ['system', 'light', 'dark'];

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
      menu.hidden = true;
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

    var toggle = document.getElementById('mark-theme-toggle');
    if (toggle) {
      toggle.setAttribute('aria-label', 'Theme: ' + theme + '. Change theme or layout');
      toggle.setAttribute('title', 'Theme: ' + theme + '. Change theme or layout');
    }
  }

  function setTheme(theme) {
    var root = document.documentElement;
    root.setAttribute('data-theme', theme);
    root.classList.toggle('dark', theme === 'dark' || (theme === 'system' && prefersDark()));
    updateThemeButtons(theme);
  }

  function cycleTheme() {
    var currentTheme = document.documentElement.getAttribute('data-theme') || 'system';
    var currentIndex = THEMES.indexOf(currentTheme);
    var nextTheme = THEMES[(currentIndex + 1 + THEMES.length) % THEMES.length];
    setTheme(nextTheme);
    closeThemeMenu();
  }

  function updateSidebarState() {
    var sidebar = document.getElementById('mark-sidebar');
    var toggle = document.getElementById('mark-sidebar-button');
    var checkbox = document.getElementById('mark-sidebar-toggle');
    if (!sidebar || !toggle) {
      return;
    }
    var visible = !sidebar.classList.contains('-translate-x-full');
    toggle.setAttribute('aria-expanded', visible ? 'true' : 'false');
    checkbox && (checkbox.checked = visible);
  }

  function toggleSidebar() {
    var sidebar = document.getElementById('mark-sidebar');
    if (!sidebar) {
      return;
    }
    sidebar.classList.toggle('-translate-x-full');
    updateSidebarState();
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
        themeMenu.hidden = expanded;
      });

      document.querySelectorAll('[data-theme-option]').forEach(function(button) {
        button.addEventListener('click', function() {
          setTheme(button.dataset.themeOption);
          closeThemeMenu();
        });
      });

      document.addEventListener('click', function(event) {
        if (themeMenu.hidden) {
          return;
        }
        if (themeMenu.contains(event.target) || themeToggle.contains(event.target)) {
          return;
        }
        closeThemeMenu();
      });
    }

    var layoutForm = document.getElementById('mark-layout-form');
    if (layoutForm) {
      layoutForm.querySelectorAll('input').forEach(function(input) {
        input.addEventListener('input', updateLayoutCommand);
        input.addEventListener('change', updateLayoutCommand);
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
      if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey || isEditableTarget(event.target)) {
        return;
      }

      var key = (event.key || '').toLowerCase();
      if (key === 'e') {
        if (document.getElementById('mark-sidebar-button')) {
          toggleSidebar();
          event.preventDefault();
        }
        return;
      }

      if (key === 't') {
        cycleTheme();
        event.preventDefault();
        return;
      }

      if (key === 'escape') {
        closeThemeMenu();
      }
    });
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

    let mut document = PAGE_TEMPLATE.replacen(r#"<html lang="en">"#, &html_root, 1);
    document = document.replacen(
        TEMPLATE_TITLE,
        &format!("<title>{}</title>", escape_html(title)),
        1,
    );
    document = document.replacen(CONTENT_PLACEHOLDER, &content_html, 1);
    document = document.replacen(
        r#"<div class="w-full max-w-[calc(8.5in+4rem)]">"#,
        r#"<div class="mark-page-width-shell w-full max-w-[calc(8.5in+4rem)]">"#,
        1,
    );
    document = replace_range(
        &document,
        LEFT_CONTROL_START,
        RIGHT_CONTROL_START,
        &sidebar_controls_html,
    );
    document = replace_range(
        &document,
        RIGHT_CONTROL_START,
        ASIDE_START,
        &render_theme_controls(theme_attr, chrome.appearance),
    );
    document = replace_range(&document, ASIDE_START, MAIN_START, &sidebar_shell_html);
    document = inject_before(
        &document,
        "</head>",
        &format!("{extra_css}{early_theme_script}"),
    );
    inject_before(&document, "</body>", enhancement_js)
}

fn render_sidebar_nodes(nodes: &[SidebarNode], out: &mut String) {
    for node in nodes {
        out.push_str(r#"<div class="space-y-2">"#);
        if node.children.is_empty() {
            let display = escape_html(&node.name);
            if node.is_current {
                out.push_str(&format!(
                    r#"<span class="mark-sidebar-link mark-sidebar-current rounded-md px-3 py-2 font-medium text-[var(--foreground)]" aria-current="page">{display}</span>"#
                ));
            } else if let Some(path) = &node.path {
                let href = escape_html(&path.to_string_lossy());
                out.push_str(&format!(
                    r#"<a class="mark-sidebar-link flex items-center rounded-md px-3 py-2 text-[var(--foreground)] transition-colors hover:bg-[var(--muted)]" href="{href}">{display}</a>"#
                ));
            }
            out.push_str("</div>");
            continue;
        }

        let display = escape_html(&format!("{}/", node.name));
        out.push_str(&format!(
            r#"<details class="mark-sidebar-dir mark-sidebar-group" open><summary class="mark-sidebar-summary rounded-md px-3 py-2 text-[var(--foreground)] transition-colors hover:bg-[var(--muted)]">{display}</summary><div class="ml-4 border-l border-[var(--border)] pl-3"><div class="space-y-2">"#
        ));
        render_sidebar_nodes(&node.children, out);
        out.push_str("</div></div></details></div>");
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
            html.contains("mark-sidebar-toggle"),
            "sidebar toggle must be present:\n{html}"
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
            html.contains("id=\"mark-sidebar-toggle\" class=\"mark-sidebar-toggle\">"),
            "sidebar checkbox should start unchecked when hidden by default:\n{html}"
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
            html.contains("id=\"mark-sidebar-toggle\" class=\"mark-sidebar-toggle\" checked>"),
            "sidebar checkbox should reflect a visible default:\n{html}"
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
}
