use crate::error::MarkError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;

/// The render theme.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            other => Err(format!(
                "invalid theme '{other}': expected 'light' or 'dark'"
            )),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

/// Persistent application configuration stored in `.mark/config.toml`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub theme: Theme,
}

impl AppConfig {
    /// Read config from `path`.  Returns a default config if the file does not
    /// exist.
    pub fn load(path: &Path) -> Result<Self, MarkError> {
        if !path.exists() {
            return Ok(AppConfig::default());
        }
        let text = std::fs::read_to_string(path)?;
        let cfg: AppConfig = toml::from_str(&text).map_err(|e| MarkError::Config(e.to_string()))?;
        Ok(cfg)
    }

    /// Persist config to `path`, creating parent directories as needed.
    pub fn save(&self, path: &Path) -> Result<(), MarkError> {
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
    fn default_theme_is_light() {
        assert_eq!(Theme::default(), Theme::Light);
        assert_eq!(AppConfig::default().theme, Theme::Light);
    }

    #[test]
    fn load_missing_file_returns_default() {
        let dir = tempdir().unwrap();
        let cfg = AppConfig::load(&dir.path().join("config.toml")).unwrap();
        assert_eq!(cfg.theme, Theme::Light);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        let cfg = AppConfig { theme: Theme::Dark };
        cfg.save(&path).unwrap();
        let loaded = AppConfig::load(&path).unwrap();
        assert_eq!(loaded.theme, Theme::Dark);
    }

    #[test]
    fn save_creates_parent_dirs() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested").join("config.toml");
        let cfg = AppConfig {
            theme: Theme::Light,
        };
        cfg.save(&path).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn theme_precedence_override_wins() {
        // Simulate: CLI override (Some) > config > default.
        // When override is present, it should win.
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
        // When no override, config should win.
        let config_theme = Theme::Dark;
        let cli_override: Option<Theme> = None;
        let resolved = match cli_override {
            Some(t) => t,
            None => config_theme,
        };
        assert_eq!(resolved, Theme::Dark);
    }
}
