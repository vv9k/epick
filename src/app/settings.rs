use crate::color::{ChromaticAdaptationMethod, ColorHarmony, Illuminant, RgbWorkingSpace};

use crate::app::ui::layout::HarmonyLayout;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

fn enabled() -> bool {
    true
}

fn is_false(it: &bool) -> bool {
    !*it
}

fn is_true(it: &bool) -> bool {
    *it
}

fn is_default_harmony_layout(it: &HarmonyLayout) -> bool {
    *it == HarmonyLayout::default()
}

fn is_default_harmony(it: &ColorHarmony) -> bool {
    *it == ColorHarmony::default()
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ColorSpaceSettings {
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub rgb: bool,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub cmyk: bool,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub hsv: bool,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub hsl: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub luv: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub lch_uv: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub lab: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub lch_ab: bool,
}

impl Default for ColorSpaceSettings {
    fn default() -> Self {
        Self {
            rgb: true,
            cmyk: true,
            hsv: true,
            hsl: true,
            luv: false,
            lch_uv: false,
            lab: false,
            lch_ab: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default)]
    pub color_display_format: DisplayFmtEnum,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub custom_display_fmt_str: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub custom_clipboard_fmt_str: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub saved_color_formats: Vec<String>,
    #[serde(default)]
    pub color_spaces: ColorSpaceSettings,
    #[serde(default)]
    pub rgb_working_space: RgbWorkingSpace,
    #[serde(default)]
    pub chromatic_adaptation_method: ChromaticAdaptationMethod,
    #[serde(default)]
    pub illuminant: Illuminant,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub cache_colors: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default_harmony")]
    pub color_harmony: ColorHarmony,
    #[serde(default = "enabled")]
    #[serde(skip_serializing_if = "is_true")]
    pub is_dark_mode: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default_harmony_layout")]
    pub harmony_layout: HarmonyLayout,
}

impl Default for Settings {
    fn default() -> Self {
        let ws = RgbWorkingSpace::default();
        Self {
            color_display_format: DisplayFmtEnum::default(),
            custom_display_fmt_str: String::new(),
            custom_clipboard_fmt_str: String::new(),
            saved_color_formats: vec![],
            color_spaces: ColorSpaceSettings::default(),
            rgb_working_space: ws,
            chromatic_adaptation_method: ChromaticAdaptationMethod::default(),
            illuminant: ws.reference_illuminant(),
            cache_colors: true,
            color_harmony: ColorHarmony::default(),
            is_dark_mode: true,
            harmony_layout: HarmonyLayout::default(),
        }
    }
}

impl Settings {
    pub const STORAGE_KEY: &'static str = "epick.saved.settings";
    pub const FILE_NAME: &'static str = "settings.yaml";

    pub fn from_yaml_str(yaml: &str) -> Result<Self> {
        serde_yaml::from_str(yaml).context("failed to deserialize settings from YAML")
    }

    pub fn as_yaml_str(&self) -> Result<String> {
        serde_yaml::to_string(&self).context("failed to serialize settings as YAML")
    }

    /// Loads the settings from the configuration file located at `path`. The configuration file is
    /// expected to be a valid YAML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let data = fs::read(path).context("failed to read configuration file")?;
        serde_yaml::from_slice(&data).context("failed to deserialize configuration")
    }

    /// Saves this settings as YAML file in the provided `path`.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = serde_yaml::to_vec(&self).context("failed to serialize settings")?;
        fs::write(path, &data).context("failed to write settings to file")
    }

    /// Returns system directory where configuration should be placed joined by the `name` parameter.
    pub fn dir(name: impl AsRef<str>) -> Option<PathBuf> {
        let name = name.as_ref();
        if let Some(dir) = dirs::config_dir() {
            return Some(dir.join(name));
        }

        if let Some(dir) = dirs::home_dir() {
            return Some(dir.join(name));
        }

        None
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum DisplayFmtEnum {
    #[serde(rename = "hex")]
    Hex,
    #[serde(rename = "hex-uppercase")]
    HexUppercase,
    #[serde(rename = "css-rgb")]
    CssRgb,
    #[serde(rename = "css-hsl")]
    CssHsl,
    #[serde(rename = "custom")]
    Custom,
}

impl Default for DisplayFmtEnum {
    fn default() -> Self {
        DisplayFmtEnum::Hex
    }
}

impl AsRef<str> for DisplayFmtEnum {
    fn as_ref(&self) -> &str {
        use DisplayFmtEnum::*;
        match &self {
            Hex => "hex",
            HexUppercase => "hex uppercase",
            CssRgb => "css rgb",
            CssHsl => "css hsl",
            Custom => "custom",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::settings::{DisplayFmtEnum, Settings};
    use crate::color::{ChromaticAdaptationMethod, ColorHarmony, Illuminant, RgbWorkingSpace};
    use std::fs;

    #[test]
    fn loads_settings() {
        let tmp = tempdir::TempDir::new("settings-test").unwrap();
        let settings_str = r#"---
color_display_format: custom
custom_display_fmt_str: "{r} {g} {b}"
color_spaces:
  hsv: false
  luv: true
  lab: true
rgb_working_space: Adobe
chromatic_adaptation_method: VonKries
illuminant: D50
"#;
        let path = tmp.path().join("settings.yaml");
        fs::write(&path, settings_str).unwrap();

        let settings = Settings::load(&path).unwrap();
        assert_eq!(settings.illuminant, Illuminant::D50);
        assert_eq!(settings.rgb_working_space, RgbWorkingSpace::Adobe);
        assert_eq!(settings.color_display_format, DisplayFmtEnum::Custom,);
        assert_eq!(settings.custom_display_fmt_str, "{r} {g} {b}");
        assert_eq!(
            settings.chromatic_adaptation_method,
            ChromaticAdaptationMethod::VonKries
        );
        assert_eq!(settings.color_harmony, ColorHarmony::default());

        assert!(settings.color_spaces.rgb);
        assert!(settings.color_spaces.cmyk);
        assert!(settings.color_spaces.hsl);
        assert!(settings.color_spaces.luv);
        assert!(settings.color_spaces.lab);
        assert!(!settings.color_spaces.hsv);
        assert!(!settings.color_spaces.lch_uv);
        assert!(!settings.color_spaces.lch_ab);
        assert!(settings.cache_colors);

        let path = tmp.path().join("new_settings.yaml");
        settings.save(&path).unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), settings_str);
    }
}
