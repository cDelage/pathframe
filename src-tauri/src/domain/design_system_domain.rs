use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DesignSystem {
    pub metadata: DesignSystemMetadata,
    pub palettes: Vec<Palette>,
    pub base: Base,
    pub themes: Vec<ThemeColor>,
    pub fonts: Fonts,
    pub typography: Typography,
    pub spaces: Vec<Space>,
    pub radius: Radius,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesignSystemCreationPayload {
    pub name: String,
    pub folder_path: String,
    pub dark_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DesignSystemMetadataHome {
    pub design_system_id: String,
    pub design_system_name: String,
    pub dark_mode: bool,
    pub design_system_path: PathBuf,
    pub is_tmp: bool,
    pub edit_mode: Option<bool>,
    pub can_undo: bool,
    pub can_redo: bool,
}

impl DesignSystemMetadataHome {
    pub fn from(
        metadata: DesignSystemMetadata,
        edit_mode: Option<bool>,
    ) -> DesignSystemMetadataHome {
        let DesignSystemMetadata {
            dark_mode,
            design_system_id,
            design_system_name,
            design_system_path,
            is_tmp,
            can_redo,
            can_undo,
        } = metadata;

        DesignSystemMetadataHome {
            dark_mode,
            design_system_id,
            design_system_name,
            design_system_path,
            edit_mode,
            is_tmp,
            can_redo,
            can_undo,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DesignSystemMetadata {
    pub design_system_id: String,
    pub design_system_name: String,
    pub dark_mode: bool,
    pub design_system_path: PathBuf,
    pub is_tmp: bool,
    pub can_undo: bool,
    pub can_redo: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignSystemMetadataFile {
    pub design_system_id: String,
    pub design_system_name: String,
    pub dark_mode: bool,
}

impl DesignSystemMetadataFile {
    pub fn from(design_system_metadata: &DesignSystemMetadata) -> DesignSystemMetadataFile {
        let DesignSystemMetadata {
            design_system_id,
            design_system_name,
            dark_mode,
            ..
        } = design_system_metadata;
        DesignSystemMetadataFile {
            design_system_id: design_system_id.to_string(),
            dark_mode: dark_mode.to_owned(),
            design_system_name: design_system_name.to_string(),
        }
    }
}

impl DesignSystemMetadata {
    pub fn from(
        design_system_file: &DesignSystemMetadataFile,
        path: &PathBuf,
        is_tmp: bool,
    ) -> DesignSystemMetadata {
        let DesignSystemMetadataFile {
            design_system_id,
            design_system_name,
            dark_mode,
        } = design_system_file;
        DesignSystemMetadata {
            dark_mode: dark_mode.to_owned(),
            design_system_id: design_system_id.to_string(),
            design_system_name: design_system_name.to_string(),
            design_system_path: path.to_owned(),
            is_tmp,
            can_redo: false,
            can_undo: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
    pub palette_name: String,
    pub palette_path: Option<PathBuf>,
    pub shades: Vec<Shade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalettesMetadataFile {
    pub palettes_order: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    pub background: ColorDarkable,
    pub border: ColorDarkable,
    pub text_light: ColorDarkable,
    pub text_default: ColorDarkable,
    pub text_dark: ColorDarkable,
    pub background_disabled: ColorDarkable,
    pub text_disabled: ColorDarkable,
    pub border_disabled: ColorDarkable,
}

impl Base {
    pub fn new() -> Base {
        Base {
            background: ColorDarkable {
                default: Some(String::from("palette-neutral-50")),
                dark: Some(String::from("palette-neutral-950")),
            },
            border: ColorDarkable {
                default: Some(String::from("palette-neutral-300")),
                dark: Some(String::from("palette-neutral-700")),
            },
            text_dark: ColorDarkable {
                default: Some(String::from("palette-neutral-900")),
                dark: Some(String::from("palette-neutral-100")),
            },
            text_default: ColorDarkable {
                default: Some(String::from("palette-neutral-700")),
                dark: Some(String::from("palette-neutral-300")),
            },
            text_light: ColorDarkable {
                default: Some(String::from("palette-neutral-500")),
                dark: Some(String::from("palette-neutral-500")),
            },
            background_disabled: ColorDarkable {
                default: Some(String::from("palette-neutral-200")),
                dark: Some(String::from("palette-neutral-700")),
            },
            border_disabled: ColorDarkable {
                default: Some(String::from("palette-neutral-300")),
                dark: Some(String::from("palette-neutral-600")),
            },
            text_disabled: ColorDarkable {
                default: Some(String::from("palette-neutral-500")),
                dark: Some(String::from("palette-neutral-500")),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shade {
    pub label: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadesFile(pub IndexMap<String, String>);

impl ShadesFile {
    pub fn from(shades: &Vec<Shade>) -> ShadesFile {
        use indexmap::IndexMap;
        let mut map = IndexMap::new();

        for shade in shades {
            map.insert(shade.label.clone(), shade.color.clone());
        }

        ShadesFile(map)
    }

    pub fn new() -> ShadesFile {
        let neutral_palette: Vec<Shade> = vec![
            Shade {
                label: "50".to_string(),
                color: "#FAFAFA".to_string(),
            },
            Shade {
                label: "100".to_string(),
                color: "#F5F5F5".to_string(),
            },
            Shade {
                label: "200".to_string(),
                color: "#E5E5E5".to_string(),
            },
            Shade {
                label: "300".to_string(),
                color: "#D4D4D4".to_string(),
            },
            Shade {
                label: "400".to_string(),
                color: "#A3A3A3".to_string(),
            },
            Shade {
                label: "500".to_string(),
                color: "#737373".to_string(),
            },
            Shade {
                label: "600".to_string(),
                color: "#525252".to_string(),
            },
            Shade {
                label: "700".to_string(),
                color: "#404040".to_string(),
            },
            Shade {
                label: "800".to_string(),
                color: "#262626".to_string(),
            },
            Shade {
                label: "900".to_string(),
                color: "#171717".to_string(),
            },
            Shade {
                label: "950".to_string(),
                color: "#0A0A0A".to_string(),
            },
        ];

        ShadesFile::from(&neutral_palette)
    }

    pub fn to(shades_file: &ShadesFile) -> Vec<Shade> {
        shades_file
            .0
            .iter()
            .map(|(key, value)| Shade {
                label: key.clone(),
                color: value.clone(),
            })
            .collect::<Vec<Shade>>()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemesMetadataFile {
    pub themes_order: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColor {
    pub theme_name: String,
    pub default: ThemeColorSet,
    pub hover: Option<ThemeColorSet>,
    pub active: Option<ThemeColorSet>,
    pub focus: Option<ThemeColorSet>,
}

impl ThemeColor {
    pub fn new() -> ThemeColor {
        ThemeColor {
            theme_name: String::from("neutral"),
            default: ThemeColorSet {
                background: ColorDarkable {
                    default: Some(String::from("palette-neutral-50")),
                    dark: Some(String::from("palette-neutral-950")),
                },
                border: ColorDarkable {
                    default: Some(String::from("palette-neutral-300")),
                    dark: Some(String::from("palette-neutral-700")),
                },
                text: ColorDarkable {
                    default: Some(String::from("palette-neutral-700")),
                    dark: Some(String::from("palette-neutral-300")),
                },
            },
            hover: Some(ThemeColorSet {
                background: ColorDarkable {
                    default: Some(String::from("palette-neutral-200")),
                    dark: Some(String::from("palette-neutral-800")),
                },
                border: ColorDarkable {
                    default: Some(String::from("palette-neutral-300")),
                    dark: Some(String::from("palette-neutral-700")),
                },
                text: ColorDarkable {
                    default: Some(String::from("palette-neutral-700")),
                    dark: Some(String::from("palette-neutral-100")),
                },
            }),
            active: None,
            focus: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColorFile {
    pub default_state: ThemeColorSet,
    pub hover: Option<ThemeColorSet>,
    pub active: Option<ThemeColorSet>,
    pub focus: Option<ThemeColorSet>,
}

impl ThemeColorFile {
    pub fn from(theme_color: &ThemeColor) -> ThemeColorFile {
        let ThemeColor {
            default: default_state,
            hover,
            active,
            focus,
            ..
        } = theme_color;
        ThemeColorFile {
            default_state: default_state.to_owned(),
            hover: hover.to_owned(),
            active: active.to_owned(),
            focus: focus.to_owned(),
        }
    }

    pub fn to(theme_file: &ThemeColorFile, theme_name: &str) -> ThemeColor {
        let ThemeColorFile {
            default_state,
            hover,
            active,
            focus,
        } = theme_file;
        ThemeColor {
            default: default_state.to_owned(),
            hover: hover.to_owned(),
            active: active.to_owned(),
            focus: focus.to_owned(),
            theme_name: String::from(theme_name),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColorSet {
    pub background: ColorDarkable,
    pub border: ColorDarkable,
    pub text: ColorDarkable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorDarkable {
    pub default: Option<String>,
    pub dark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fonts {
    pub default: String,
    pub additionals: Vec<AdditionalFont>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalFont {
    pub font_name: String,
    pub value: String,
}

impl Fonts {
    pub fn new() -> Fonts {
        Fonts {
            default: String::from("'Helvetica', 'Arial', sans-serif"),
            additionals: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Typography {
    paragraph: TypographyScale,
    h1: TypographyScale,
    h2: TypographyScale,
    h3: TypographyScale,
    h4: TypographyScale,
    h5: TypographyScale,
    h6: TypographyScale,
    small: TypographyScale,
    strong: TypographyScale,
    additionals_scales: Vec<AdditionalTypographyScale>,
}

impl Typography {
    pub fn new() -> Typography {
        Typography {
            h1: TypographyScale {
                font_size: "32px".to_string(),
                line_height: "40px".to_string(),
                font_weight: FontWeight::Seven,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            h2: TypographyScale {
                font_size: "28px".to_string(),
                line_height: "36px".to_string(),
                font_weight: FontWeight::Six,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            h3: TypographyScale {
                font_size: "24px".to_string(),
                line_height: "32px".to_string(),
                font_weight: FontWeight::Five,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            h4: TypographyScale {
                font_size: "20px".to_string(),
                line_height: "28px".to_string(),
                font_weight: FontWeight::Five,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            h5: TypographyScale {
                font_size: "18px".to_string(),
                line_height: "24px".to_string(),
                font_weight: FontWeight::Five,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            h6: TypographyScale {
                font_size: "16px".to_string(),
                line_height: "22px".to_string(),
                font_weight: FontWeight::Five,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            paragraph: TypographyScale {
                font_size: "14px".to_string(),
                line_height: "20px".to_string(),
                font_weight: FontWeight::Four,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            small: TypographyScale {
                font_size: "12px".to_string(),
                line_height: "16px".to_string(),
                font_weight: FontWeight::Four,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            strong: TypographyScale {
                font_size: "14px".to_string(),
                line_height: "20px".to_string(),
                font_weight: FontWeight::Seven,
                letter_spacing: TypographySpacing::Zero,
                word_spacing: TypographySpacing::Zero,
                font_style: FontStyle::Normal,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
                padding: "0".to_string(),
                margin: "0".to_string(),
            },
            additionals_scales: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalTypographyScale {
    scale_name: String,
    scale: TypographyScale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalFontWeight {
    weight_name: String,
    font_weight: FontWeight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypographyScale {
    font_size: String,
    line_height: String,
    font_weight: FontWeight,
    letter_spacing: TypographySpacing,
    word_spacing: TypographySpacing,
    font_style: FontStyle,
    text_transform: TextTransform,
    text_decoration: TextDecoration,
    padding: String,
    margin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FontStyle {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "oblique")]
    Oblique,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextTransform {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(rename = "lowercase")]
    Lowercase,
    #[serde(rename = "capitalize")]
    Capitalize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextDecoration {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "overline")]
    Overline,
    #[serde(rename = "line-through")]
    LineThrough,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(u16)]
pub enum FontWeight {
    #[serde(rename = "100")]
    One = 100,
    #[serde(rename = "200")]
    Two = 200,
    #[serde(rename = "300")]
    Three = 300,
    #[serde(rename = "400")]
    Four = 400,
    #[serde(rename = "500")]
    Five = 500,
    #[serde(rename = "600")]
    Six = 600,
    #[serde(rename = "700")]
    Seven = 700,
    #[serde(rename = "800")]
    Eight = 800,
    #[serde(rename = "900")]
    Nine = 900,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(u16)]
pub enum TypographySpacing {
    #[serde(rename = "-0.05em")]
    MinusTwo,
    #[serde(rename = "-0.02em")]
    MinusOne,
    #[serde(rename = "0em")]
    Zero,
    #[serde(rename = "0.1em")]
    One,
    #[serde(rename = "0.2em")]
    Two,
    #[serde(rename = "0.3em")]
    Three,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub space_key: String,
    pub space_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacesFile(pub IndexMap<String, String>);

impl SpacesFile {
    pub fn from(spaces: &Vec<Space>) -> SpacesFile {
        let mut map = IndexMap::new();
        for space in spaces {
            map.insert(space.space_key.clone(), space.space_value.clone());
        }
        SpacesFile(map)
    }

    pub fn new() -> SpacesFile {
        // Valeurs par défaut
        let default_spaces: Vec<Space> = vec![
            Space {
                space_key: "0".to_string(),
                space_value: "0px".to_string(),
            },
            Space {
                space_key: "1".to_string(),
                space_value: "2px".to_string(),
            },
            Space {
                space_key: "2".to_string(),
                space_value: "4px".to_string(),
            },
            Space {
                space_key: "3".to_string(),
                space_value: "8px".to_string(),
            },
            Space {
                space_key: "4".to_string(),
                space_value: "12px".to_string(),
            },
            Space {
                space_key: "5".to_string(),
                space_value: "16px".to_string(),
            },
            Space {
                space_key: "6".to_string(),
                space_value: "20px".to_string(),
            },
            Space {
                space_key: "7".to_string(),
                space_value: "28px".to_string(),
            },
            Space {
                space_key: "8".to_string(),
                space_value: "32px".to_string(),
            },
            Space {
                space_key: "9".to_string(),
                space_value: "40px".to_string(),
            },
            Space {
                space_key: "10".to_string(),
                space_value: "52px".to_string(),
            },
            Space {
                space_key: "11".to_string(),
                space_value: "64px".to_string(),
            },
            Space {
                space_key: "12".to_string(),
                space_value: "80px".to_string(),
            },
            Space {
                space_key: "13".to_string(),
                space_value: "100px".to_string(),
            },
            Space {
                space_key: "14".to_string(),
                space_value: "120px".to_string(),
            },
            Space {
                space_key: "15".to_string(),
                space_value: "160px".to_string(),
            },
        ];

        SpacesFile::from(&default_spaces)
    }

    pub fn to(spaces_file: &SpacesFile) -> Vec<Space> {
        spaces_file
            .0
            .iter()
            .map(|(key, value)| Space {
                space_key: key.clone(),
                space_value: value.clone(),
            })
            .collect::<Vec<Space>>()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Radius {
    default: String,
    additionals_radius: Vec<RadiusItem>,
}

impl Radius {
    pub fn new() -> Radius {
        Radius {
            default: "0px".to_string(),
            additionals_radius: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadiusItem {
    pub radius_key: String,
    pub radius_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effect_name: String,
    pub items: Vec<EffectItem>,
    pub bg: Option<String>,
}

impl Effect {
    pub fn new() -> Effect {
        Effect {
            effect_name: "shadow".to_string(),
            bg: None,
            items: vec![
                EffectItem {
                    effect_type: EffectType::BoxShadow,
                    effect_value: "rgba(0, 0, 0, 0.1) 0px 1px 3px 0px".to_string(),
                },
                EffectItem {
                    effect_type: EffectType::BoxShadow,
                    effect_value: "rgba(0, 0, 0, 0.06) 0px 1px 2px 0px".to_string(),
                },
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EffectItem {
    pub effect_type: EffectType,
    pub effect_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EffectType {
    BoxShadow,
    Blur,
    BackdropFilter,
}
