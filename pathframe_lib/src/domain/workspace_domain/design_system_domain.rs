use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSystem {
    pub design_system_id: String,
    pub design_system_name: String,
    pub dark_mode_defined: bool,
    pub primitives: Primitives,
    pub tokens: Tokens,
    #[serde(skip_deserializing)]
    pub design_system_path: Option<String>,
}

//Primitives 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Primitives {
    pub color_palettes: Vec<ColorPalette>,
    pub single_colors: HashMap<String, String>,
    pub space: SpacePalette,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacePalette(HashMap<String,String>);

impl SpacePalette {
    pub fn parse(spaces : HashMap<String,String>) -> Result<SpacePalette> {
        Ok(SpacePalette(spaces))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub palette_name: String,
    pub shades: HashMap<String, String>,
}


//Tokens

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokens {
    pub light_default_single_tokens: BaseColorTokens,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_default_single_tokens: Option<BaseColorTokens>,
    pub colors_themes: Vec<ColorTheme>,
    pub text_tokens: TextTokens,
    pub radius_tokens: RadiusTokens,
    pub shadows: Vec<Effect>,
    pub space_tokens: SpaceTokens,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseColorTokens {
    pub bg_main: String,
    pub text_color_main_light: String,
    pub text_color_main: String,
    pub text_color_main_dark: String,
    pub border_main: String,
    pub bg_main_disabled: String,
    pub text_color_main_disabled: String,
    pub border_main_disabled: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTheme {
    pub theme_name: String,
    pub light: ColorSet,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<ColorSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSet {
    pub default: ColorState,
    pub hover: ColorState,
    pub active: ColorState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorState {
    pub bg: String,
    pub border: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTokens {
    pub p: TextTheme,
    pub h1: TextTheme,
    pub h2: TextTheme,
    pub h3: TextTheme,
    pub font: String,
    pub light_weight: u32,
    pub default_weight: u32,
    pub bold_weight: u32,
    pub font_size_sm: String,
    pub font_size_md: String,
    pub font_size_lg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTheme {
    pub font_size: String,
    pub font_weight: u32,
    pub line_height: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub effect_name: String,
    pub light: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusTokens {
    pub default: String,
    pub large: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceTokens {
    pub default_page_padding: String,
    pub default_element_padding: String,
}