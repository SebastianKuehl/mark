use crate::error::MarkError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;

/// The render theme.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "system" => Ok(Theme::System),
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            other => Err(format!(
                "invalid theme '{other}': expected 'system', 'light', or 'dark'"
            )),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::System => write!(f, "system"),
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

/// The render mode to use when materializing output.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenderMode {
    Single,
    #[default]
    Recursive,
}

impl FromStr for RenderMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "single" => Ok(RenderMode::Single),
            "recursive" => Ok(RenderMode::Recursive),
            other => Err(format!(
                "invalid render mode '{other}': expected 'single' or 'recursive'"
            )),
        }
    }
}

impl std::fmt::Display for RenderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderMode::Single => write!(f, "single"),
            RenderMode::Recursive => write!(f, "recursive"),
        }
    }
}

/// The default sidebar visibility when rendering recursive output.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SidebarVisibility {
    #[default]
    Hidden,
    Visible,
}

impl FromStr for SidebarVisibility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hidden" => Ok(SidebarVisibility::Hidden),
            "visible" => Ok(SidebarVisibility::Visible),
            other => Err(format!(
                "invalid sidebar visibility '{other}': expected 'hidden' or 'visible'"
            )),
        }
    }
}

impl std::fmt::Display for SidebarVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SidebarVisibility::Hidden => write!(f, "hidden"),
            SidebarVisibility::Visible => write!(f, "visible"),
        }
    }
}

/// Persistent appearance controls for the rendered reader shell.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AppearanceConfig {
    #[serde(default = "default_font_size_px")]
    pub font_size_px: u16,
    #[serde(default = "default_letter_width_in")]
    pub letter_width_in: f32,
    #[serde(default = "default_letter_radius_px")]
    pub letter_radius_px: u16,
    #[serde(default = "default_sidebar_button_radius_px")]
    pub sidebar_button_radius_px: u16,
    #[serde(default = "default_theme_button_radius_px")]
    pub theme_button_radius_px: u16,
}

const fn default_font_size_px() -> u16 {
    16
}

const fn default_letter_radius_px() -> u16 {
    12
}

const fn default_sidebar_button_radius_px() -> u16 {
    999
}

const fn default_theme_button_radius_px() -> u16 {
    999
}

fn default_letter_width_in() -> f32 {
    8.5
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            font_size_px: default_font_size_px(),
            letter_width_in: default_letter_width_in(),
            letter_radius_px: default_letter_radius_px(),
            sidebar_button_radius_px: default_sidebar_button_radius_px(),
            theme_button_radius_px: default_theme_button_radius_px(),
        }
    }
}

impl AppearanceConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !(10..=32).contains(&self.font_size_px) {
            return Err(format!(
                "invalid font size '{}': expected a value between 10 and 32 px",
                self.font_size_px
            ));
        }
        if !(5.0..=12.0).contains(&self.letter_width_in) {
            return Err(format!(
                "invalid letter width '{}': expected a value between 5.0 and 12.0 inches",
                self.letter_width_in
            ));
        }
        if self.letter_radius_px > 64 {
            return Err(format!(
                "invalid letter radius '{}': expected a value between 0 and 64 px",
                self.letter_radius_px
            ));
        }
        if self.sidebar_button_radius_px > 999 {
            return Err(format!(
                "invalid sidebar button radius '{}': expected a value between 0 and 999 px",
                self.sidebar_button_radius_px
            ));
        }
        if self.theme_button_radius_px > 999 {
            return Err(format!(
                "invalid theme button radius '{}': expected a value between 0 and 999 px",
                self.theme_button_radius_px
            ));
        }
        Ok(())
    }
}

/// Persistent application configuration stored in `.mark/config.toml`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub render_mode: RenderMode,
    #[serde(default)]
    pub sidebar: SidebarVisibility,
    #[serde(default)]
    pub appearance: AppearanceConfig,
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), MarkError> {
        self.appearance.validate().map_err(MarkError::Config)
    }

    /// Read config from `path`.  Returns a default config if the file does not
    /// exist.
    pub fn load(path: &Path) -> Result<Self, MarkError> {
        if !path.exists() {
            return Ok(AppConfig::default());
        }
        let text = std::fs::read_to_string(path)?;
        let cfg: AppConfig = toml::from_str(&text).map_err(|e| MarkError::Config(e.to_string()))?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Persist config to `path`, creating parent directories as needed.
    pub fn save(&self, path: &Path) -> Result<(), MarkError> {
        self.validate()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = toml::to_string(self).map_err(|e| MarkError::Config(e.to_string()))?;
        std::fs::write(path, text.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn theme_from_str_valid() {
        assert_eq!("system".parse::<Theme>().unwrap(), Theme::System);
        assert_eq!("light".parse::<Theme>().unwrap(), Theme::Light);
        assert_eq!("dark".parse::<Theme>().unwrap(), Theme::Dark);
        assert_eq!("Dark".parse::<Theme>().unwrap(), Theme::Dark);
    }

    #[test]
    fn theme_from_str_invalid() {
        assert!("blue".parse::<Theme>().is_err());
        assert!("".parse::<Theme>().is_err());
    }

    #[test]
    fn render_mode_from_str_valid() {
        assert_eq!("single".parse::<RenderMode>().unwrap(), RenderMode::Single);
        assert_eq!(
            "recursive".parse::<RenderMode>().unwrap(),
            RenderMode::Recursive
        );
    }

    #[test]
    fn render_mode_from_str_invalid() {
        assert!("loop".parse::<RenderMode>().is_err());
        assert!("".parse::<RenderMode>().is_err());
    }

    #[test]
    fn sidebar_visibility_from_str_valid() {
        assert_eq!(
            "hidden".parse::<SidebarVisibility>().unwrap(),
            SidebarVisibility::Hidden
        );
        assert_eq!(
            "visible".parse::<SidebarVisibility>().unwrap(),
            SidebarVisibility::Visible
        );
    }

    #[test]
    fn sidebar_visibility_from_str_invalid() {
        assert!("open".parse::<SidebarVisibility>().is_err());
        assert!("".parse::<SidebarVisibility>().is_err());
    }

    #[test]
    fn default_config_uses_system_recursive_hidden_and_default_appearance() {
        assert_eq!(Theme::default(), Theme::System);
        assert_eq!(AppConfig::default().theme, Theme::System);
        assert_eq!(AppConfig::default().render_mode, RenderMode::Recursive);
        assert_eq!(AppConfig::default().sidebar, SidebarVisibility::Hidden);
        assert_eq!(AppConfig::default().appearance.font_size_px, 16);
        assert!((AppConfig::default().appearance.letter_width_in - 8.5).abs() < f32::EPSILON);
    }

    #[test]
    fn load_missing_file_returns_default() {
        let dir = tempdir().unwrap();
        let cfg = AppConfig::load(&dir.path().join("config.toml")).unwrap();
        assert_eq!(cfg.theme, Theme::System);
        assert_eq!(cfg.render_mode, RenderMode::Recursive);
        assert_eq!(cfg.sidebar, SidebarVisibility::Hidden);
        assert_eq!(cfg.appearance.letter_radius_px, 12);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        let cfg = AppConfig {
            theme: Theme::Dark,
            render_mode: RenderMode::Single,
            sidebar: SidebarVisibility::Visible,
            appearance: AppearanceConfig {
                font_size_px: 18,
                letter_width_in: 7.75,
                letter_radius_px: 20,
                sidebar_button_radius_px: 18,
                theme_button_radius_px: 14,
            },
        };
        cfg.save(&path).unwrap();
        let loaded = AppConfig::load(&path).unwrap();
        assert_eq!(loaded.theme, Theme::Dark);
        assert_eq!(loaded.render_mode, RenderMode::Single);
        assert_eq!(loaded.sidebar, SidebarVisibility::Visible);
        assert_eq!(loaded.appearance, cfg.appearance);
    }

    #[test]
    fn save_creates_parent_dirs() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested").join("config.toml");
        let cfg = AppConfig {
            theme: Theme::System,
            render_mode: RenderMode::Recursive,
            sidebar: SidebarVisibility::Hidden,
            appearance: AppearanceConfig::default(),
        };
        cfg.save(&path).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn invalid_appearance_is_rejected() {
        let cfg = AppConfig {
            theme: Theme::System,
            render_mode: RenderMode::Recursive,
            sidebar: SidebarVisibility::Hidden,
            appearance: AppearanceConfig {
                font_size_px: 9,
                ..AppearanceConfig::default()
            },
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn theme_precedence_override_wins() {
        let config_theme = Theme::Dark;
        let cli_override: Option<Theme> = Some(Theme::Light);
        let resolved = match cli_override {
            Some(t) => t,
            None => config_theme,
        };
        assert_eq!(resolved, Theme::Light);
    }

    #[test]
    fn theme_precedence_config_wins_over_default() {
        let config_theme = Theme::Dark;
        let cli_override: Option<Theme> = None;
        let resolved = match cli_override {
            Some(t) => t,
            None => config_theme,
        };
        assert_eq!(resolved, Theme::Dark);
    }
}
