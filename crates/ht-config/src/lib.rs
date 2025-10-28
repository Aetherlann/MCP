use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub appearance: AppearanceConfig,
    #[serde(default)]
    pub behavior: BehaviorConfig,
    #[serde(default)]
    pub profiles: Vec<ProfileConfig>,
    #[serde(default)]
    pub media: MediaConfig,
    #[serde(default)]
    pub layout: LayoutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub glass: GlassConfig,
    #[serde(default)]
    pub font: FontConfig,
    #[serde(default)]
    pub cursor: CursorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlassConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_blur")]
    pub blur_radius: f32,
    #[serde(default = "default_tint")]
    pub tint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    #[serde(default = "default_font")]
    pub family: String,
    #[serde(default = "default_font_size")]
    pub size: f32,
    #[serde(default = "default_true")]
    pub ligatures: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorConfig {
    #[serde(default = "default_cursor_shape")]
    pub shape: String,
    #[serde(default = "default_true")]
    pub blink: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    #[serde(default = "default_scrollback")]
    pub scrollback: usize,
    #[serde(default)]
    pub copy_on_select: bool,
    #[serde(default = "default_bell")]
    pub bell: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub shell: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: Vec<(String, String)>,
    #[serde(default)]
    pub cwd: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConfig {
    #[serde(default = "default_true")]
    pub enable_kitty: bool,
    #[serde(default = "default_true")]
    pub enable_iterm: bool,
    #[serde(default)]
    pub enable_sixel: bool,
    #[serde(default = "default_placement")]
    pub default_placement: String,
    #[serde(default)]
    pub video: VideoConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    #[serde(default = "default_true")]
    pub audio: bool,
    #[serde(default = "default_hwdecode")]
    pub hw_decode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    #[serde(default)]
    pub startup: Option<String>,
}

// Default functions
fn default_true() -> bool { true }
fn default_theme() -> String { "Everglass Dark".to_string() }
fn default_opacity() -> f32 { 0.8 }
fn default_blur() -> f32 { 24.0 }
fn default_tint() -> String { "#0b0f19".to_string() }
fn default_font() -> String { "Cascadia Code".to_string() }
fn default_font_size() -> f32 { 13.5 }
fn default_cursor_shape() -> String { "bar".to_string() }
fn default_scrollback() -> usize { 120000 }
fn default_bell() -> String { "visual".to_string() }
fn default_placement() -> String { "inline".to_string() }
fn default_hwdecode() -> String { "auto".to_string() }

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            glass: GlassConfig::default(),
            font: FontConfig::default(),
            cursor: CursorConfig::default(),
        }
    }
}

impl Default for GlassConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            opacity: default_opacity(),
            blur_radius: default_blur(),
            tint: default_tint(),
        }
    }
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: default_font(),
            size: default_font_size(),
            ligatures: true,
        }
    }
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            shape: default_cursor_shape(),
            blink: true,
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            scrollback: default_scrollback(),
            copy_on_select: false,
            bell: default_bell(),
        }
    }
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            enable_kitty: true,
            enable_iterm: true,
            enable_sixel: false,
            default_placement: default_placement(),
            video: VideoConfig::default(),
        }
    }
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            audio: true,
            hw_decode: default_hwdecode(),
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            startup: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            appearance: AppearanceConfig::default(),
            behavior: BehaviorConfig::default(),
            profiles: vec![
                ProfileConfig {
                    name: "Default".to_string(),
                    shell: if cfg!(windows) {
                        "powershell.exe".to_string()
                    } else {
                        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
                    },
                    args: vec![],
                    env: vec![("TERM".to_string(), "xterm-256color".to_string())],
                    cwd: None,
                }
            ],
            media: MediaConfig::default(),
            layout: LayoutConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;

        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("", "", "HyperTerminal")
            .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))?;

        Ok(dirs.config_dir().join("config.json"))
    }
}
