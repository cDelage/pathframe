use super::design_system_domain::{
    BackgroundColorToken, BaseColorTokens, BorderColorToken, ColorPalette, ColorSet, ColorState,
    ColorTheme, DesignSystem, Effect, EffectType, Primitives, RadiusTokens, SingleColors,
    SpacePalette, SpaceTokens, TextColorToken, TextTheme, TextTokens, Tokens,
};

pub trait ToStylesheet {
    fn to_stylesheet(&self) -> String;
}

pub struct StylesheetLightDark {
    pub light: String,
    pub dark: String,
}
pub trait ToStylesheetDarkable {
    fn to_stylesheet(&self) -> StylesheetLightDark;
}

impl StylesheetLightDark {
    pub fn get_light_stylesheet(stylesheets: &Vec<StylesheetLightDark>) -> String {
        stylesheets
            .iter()
            .map(|effect| effect.light.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_dark_stylesheet(stylesheets: &Vec<StylesheetLightDark>) -> String {
        stylesheets
            .iter()
            .map(|effect| effect.dark.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl ToStylesheet for DesignSystem {
    fn to_stylesheet(&self) -> String {
        let DesignSystem {
            primitives, tokens, ..
        } = self;
        let primitives_stylesheet: String = primitives.to_stylesheet();
        let StylesheetLightDark {dark, light} = tokens.to_stylesheet();
        format!("
            :root {{
                {primitives_stylesheet}
                {light}
            }}
            @media (prefers-color-scheme: dark) {{
            :root {{
                {dark}
            }}
            }}
        ")
    }
}

impl ToStylesheet for Primitives {
    fn to_stylesheet(&self) -> String {
        let color_palettes_stylesheet: &String = &self
            .color_palettes
            .iter()
            .map(|palette| palette.to_stylesheet())
            .collect::<Vec<String>>()
            .join("\n");
        let single_colors_stylesheet: &String = &self.single_colors.to_stylesheet();
        let spaces_stylesheet: &String = &self.space.to_stylesheet();
        format!(
            "{}\n{}\n{}",
            color_palettes_stylesheet, single_colors_stylesheet, spaces_stylesheet
        )
    }
}

impl ToStylesheet for ColorPalette {
    fn to_stylesheet(&self) -> String {
        let color_key: &String = &self.palette_name;
        self.shades
            .0
            .iter()
            .map(|(key, value)| format!("--color-{color_key}-{key}:{value}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl ToStylesheet for SingleColors {
    fn to_stylesheet(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("--color-single-{key}:{value}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl ToStylesheet for SpacePalette {
    fn to_stylesheet(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("--space-{key}:{value}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl ToStylesheetDarkable for Tokens {
    fn to_stylesheet(&self) -> StylesheetLightDark {
        let Tokens {
            light_default_single_tokens,
            dark_default_single_tokens,
            colors_themes,
            text_tokens,
            radius_tokens,
            space_tokens,
            effects,
        } = self;
        let light_default_tokens_stylesheet: String = light_default_single_tokens.to_stylesheet();
        let dark_default_single_tokens_stylesheet: String = match dark_default_single_tokens {
            Some(tokens) => tokens.to_stylesheet(),
            None => String::new(),
        };
        let colors_themes_stylesheets: Vec<StylesheetLightDark> = colors_themes
            .iter()
            .map(|color_theme| {
                let ColorTheme {
                    light,
                    dark,
                    theme_name,
                } = color_theme;
                StylesheetLightDark {
                    light: light.to_stylesheet(theme_name),
                    dark: match dark {
                        Some(theme) => theme.to_stylesheet(theme_name),
                        None => String::new(),
                    },
                }
            })
            .collect::<Vec<StylesheetLightDark>>();
        let color_themes_light_stylesheet: String =
            StylesheetLightDark::get_light_stylesheet(&colors_themes_stylesheets);
        let color_themes_dark_stylesheet: String =
            StylesheetLightDark::get_dark_stylesheet(&colors_themes_stylesheets);

        let text_tokens_stylesheet: String = text_tokens.to_stylesheet();
        let radius_tokens_stylesheet: String = radius_tokens.to_stylesheet();
        let space_tokens_stylesheet: String = space_tokens.to_stylesheet();
        let effect_stylesheets: Vec<StylesheetLightDark> = effects
            .iter()
            .map(|effect| effect.to_stylesheet())
            .collect::<Vec<StylesheetLightDark>>();
        let light_effects: String = StylesheetLightDark::get_light_stylesheet(&effect_stylesheets);
        let dark_effects = StylesheetLightDark::get_dark_stylesheet(&effect_stylesheets);

        StylesheetLightDark {
            light: format!("{light_default_tokens_stylesheet}\n{color_themes_light_stylesheet}\n{text_tokens_stylesheet}\n{radius_tokens_stylesheet}\n{space_tokens_stylesheet}\n{light_effects}"),
            dark: format!("{dark_default_single_tokens_stylesheet}\n{color_themes_dark_stylesheet}\n{dark_effects}"),
        }
    }
}

impl ToStylesheet for BaseColorTokens {
    fn to_stylesheet(&self) -> String {
        let BaseColorTokens {
            bg_main,
            text_color_main,
            text_color_main_dark,
            text_color_main_light,
            border_main,
            bg_main_disabled,
            border_main_disabled,
            text_color_main_disabled,
        } = &self;
        format!(
            "--bg-main:{bg_main};
            --text-color-main:{text_color_main};
            --text-color-main-dark:{text_color_main_dark};
            --text-color-main-light:{text_color_main_light};
            --border-main:{border_main};
            --bg-main-disabled:{bg_main_disabled};
            --border-main-disabled:{border_main_disabled};
            --text-color-main-disabled:{text_color_main_disabled};"
        )
    }
}

impl ToStylesheetDarkable for ColorTheme {
    fn to_stylesheet(&self) -> StylesheetLightDark {
        let ColorTheme {
            light,
            dark,
            theme_name,
        } = &self;
        let light_stylesheet: String = light.to_stylesheet(theme_name);
        let dark_stylesheet: String = match dark {
            Some(color_set) => color_set.to_stylesheet(theme_name),
            None => String::new(),
        };
        StylesheetLightDark {
            light: light_stylesheet,
            dark: dark_stylesheet,
        }
    }
}

impl ColorSet {
    fn to_stylesheet(&self, theme_name: &str) -> String {
        let default_stylesheet: &String = &self.default.to_stylesheet(theme_name, None, "");
        let hover_stylesheet: &String = &self.hover.to_stylesheet(theme_name, Some("hover"), ":");
        let active_stylesheet: &String =
            &self.active.to_stylesheet(theme_name, Some("active"), ":");
        let active_by_class_stylesheet: &String =
            &self.active.to_stylesheet(theme_name, Some("active"), ".");
        format!("{default_stylesheet}\n{hover_stylesheet}\n{active_stylesheet}\n{active_by_class_stylesheet}")
    }
}

fn get_class_state_headers(color_state: Option<&str>, state_prefix: &str) -> (String, String) {
    match color_state {
        Some(state) => (format!(".\\{state}\\:"), format!("{state_prefix}{state}")),
        None => (String::from("."), String::new()),
    }
}

impl ColorState {
    fn to_stylesheet(&self, theme_name: &str, state: Option<&str>, state_prefix: &str) -> String {
        let ColorState { bg, border, text } = &self;
        let (class_prefix, class_sufix): (String, String) =
            get_class_state_headers(state, state_prefix);

        let bg_stylesheet: String = bg.to_stylesheet(&class_prefix, &theme_name, &class_sufix);
        let text_color_stylesheet: String =
            text.to_stylesheet(&class_prefix, &theme_name, &class_sufix);
        let border_stylesheet: String =
            border.to_stylesheet(&class_prefix, &theme_name, &class_sufix);
        let bg_value = &bg.0;
        let text_value: &String = &text.0;

        format!(
            "{class_prefix}theme-{theme_name}{class_sufix}{{
            background:var({bg_value});
            color:var({text_value});
        }}
        {bg_stylesheet}\n{text_color_stylesheet}\n{border_stylesheet}"
        )
    }
}

impl BackgroundColorToken {
    pub fn to_stylesheet(&self, class_prefix: &str, theme_name: &str, class_sufix: &str) -> String {
        let color = &self.0;
        format!(
            "{class_prefix}bg-theme-{theme_name}{class_sufix}{{
            background: var({color});
        }}"
        )
    }
}

impl TextColorToken {
    pub fn to_stylesheet(&self, class_prefix: &str, theme_name: &str, class_sufix: &str) -> String {
        let color = &self.0;
        format!(
            "{class_prefix}text-color-theme-{theme_name}{class_sufix}{{
            color: var({color});
        }}"
        )
    }
}

impl BorderColorToken {
    pub fn to_stylesheet(&self, class_prefix: &str, theme_name: &str, class_sufix: &str) -> String {
        let is_gradient: &bool = &self.0.starts_with("--gradient");
        let full =
            &self.get_class_boilerplate(&is_gradient, "", &class_prefix, &class_sufix, theme_name);
        let top = &self.get_class_boilerplate(
            &is_gradient,
            "-top",
            &class_prefix,
            &class_sufix,
            theme_name,
        );
        let right = &self.get_class_boilerplate(
            &is_gradient,
            "-right",
            &class_prefix,
            &class_sufix,
            theme_name,
        );
        let bottom = &self.get_class_boilerplate(
            &is_gradient,
            "-bottom",
            &class_prefix,
            &class_sufix,
            theme_name,
        );
        let left = &self.get_class_boilerplate(
            &is_gradient,
            "-left",
            &class_prefix,
            &class_sufix,
            theme_name,
        );
        format!("{full}\n{top}\n{right}\n{bottom}\n{left}")
    }

    fn get_class_boilerplate(
        &self,
        is_gradient: &bool,
        border_sufix: &str,
        class_prefix: &str,
        class_state: &str,
        theme_name: &str,
    ) -> String {
        let token = &self.0;
        let class = format!("{class_prefix}border{border_sufix}-{theme_name}{class_state}");
        if *is_gradient {
            format!(
                "{class}{{border{border_sufix}: 1px solid;
        border-image-slice: 1;
        border-width: 1px;
        border-image-source: var({token});}}"
            )
        } else {
            format!("{class}{{border{border_sufix}: 1px solid var({token});}}")
        }
    }
}

impl ToStylesheet for TextTokens {
    fn to_stylesheet(&self) -> String {
        let TextTokens {
            bold_weight,
            default_weight,
            font,
            font_size_lg,
            font_size_md,
            font_size_sm,
            h1,
            h2,
            h3,
            light_weight,
            p,
        } = &self;
        let p_stylesheet: String = p.to_stylesheet("p");
        let h1_stylesheet: String = h1.to_stylesheet("h1");
        let h2_stylesheet: String = h2.to_stylesheet("h2");
        let h3_stylesheet: String = h3.to_stylesheet("h3");
        format!(
            "font-weight:{default_weight};
            font-family:{font};
            font-size:{font_size_md};
            {p_stylesheet}\n{h1_stylesheet}\n{h2_stylesheet}\n{h3_stylesheet}
            .font-light{{
                font-weight:{light_weight};
            }}
            .font-normal{{
                font-weight:{default_weight};
            }}
            .font-bold{{
                font-weight:{bold_weight};
            }}
            .text-sm{{
                font-size:{font_size_sm};
            }}
            .text-normal{{
                font-size:{font_size_md};
            }}
            .text-lg{{
                font-size:{font_size_lg};
            }}"
        )
    }
}

impl TextTheme {
    fn to_stylesheet(&self, class: &str) -> String {
        let TextTheme {
            font_size,
            font_weight,
            line_height,
        } = &self;
        format!(
            "{class}{{
            font-size:{font_size};
            font-weight:{font_weight};
            line_height:{line_height};
        }}"
        )
    }
}

impl ToStylesheetDarkable for Effect {
    fn to_stylesheet(&self) -> StylesheetLightDark {
        let Effect {
            effect_name,
            light,
            dark,
            effect_type,
        } = &self;
        match effect_type {
            EffectType::Shadow => {
                let dark_stylesheet: String = match dark {
                    Some(token) => format!(".{effect_name}{{box-shadow:{token}}}"),
                    None => String::new(),
                };
                StylesheetLightDark {
                    light: format!(".{effect_name}{{box-shadow:{light}}}"),
                    dark: dark_stylesheet,
                }
            }
        }
    }
}

impl ToStylesheet for RadiusTokens {
    fn to_stylesheet(&self) -> String {
        let RadiusTokens { default, large } = self;
        format!(
            ".rounded{{border-radius:{default};}}
        .rounded-lg{{border-radius:{large};}}"
        )
    }
}

impl ToStylesheet for SpaceTokens {
    fn to_stylesheet(&self) -> String {
        let SpaceTokens {
            default_page_padding,
            default_element_padding,
        } = self;
        format!(
            "
            --page-padding:{default_page_padding};
            --element-padding:{default_element_padding};
        "
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domain::workspace_domain::design_system_domain::Shades;

    use super::*;

    #[test]
    fn test_color_palette_to_stylesheet() {
        let mut palette = ColorPalette {
            palette_name: String::from("primary"),
            shades: Shades(HashMap::new()),
        };

        palette
            .shades
            .0
            .insert(String::from("50"), String::from("#FFFFFF"));
        palette
            .shades
            .0
            .insert(String::from("950"), String::from("#DDDDDD"));

        let expected_contain = "--color-primary-50:#FFFFFF";
        let expected_contain2 = "--color-primary-950:#DDDDDD";

        let palette_to_stylesheet: String = palette.to_stylesheet();

        assert!(palette_to_stylesheet.contains(expected_contain));
        assert!(palette_to_stylesheet.contains(expected_contain2));
    }

    #[test]
    fn test_single_colors_to_stylesheet() {
        let mut single_colors = SingleColors(HashMap::new());

        single_colors
            .0
            .insert(String::from("white"), String::from("#ffffff"));
        single_colors
            .0
            .insert(String::from("red"), String::from("#e24"));

        let stylesheet: String = single_colors.to_stylesheet();

        let contain_1 = "--color-single-white:#ffffff";
        let contain_2 = "--color-single-red:#e24";

        assert!(stylesheet.contains(contain_1));
        assert!(stylesheet.contains(contain_2));
    }

    #[test]
    fn test_space_palette_to_stylesheet() {
        let mut single_colors = SpacePalette(HashMap::new());

        single_colors
            .0
            .insert(String::from("0"), String::from("10px"));
        single_colors
            .0
            .insert(String::from("1"), String::from("20px"));

        let stylesheet: String = single_colors.to_stylesheet();

        let contain_1 = "--space-0:10px";
        let contain_2 = "--space-1:20px";

        assert!(stylesheet.contains(contain_1));
        assert!(stylesheet.contains(contain_2));
    }

    #[test]
    fn test_base_color_to_stylesheet() {
        let tokens = BaseColorTokens {
            bg_main: "#FFFFFF".to_string(),
            text_color_main: "#000000".to_string(),
            text_color_main_dark: "#333333".to_string(),
            text_color_main_light: "#666666".to_string(),
            border_main: "#CCCCCC".to_string(),
            bg_main_disabled: "#F0F0F0".to_string(),
            border_main_disabled: "#AAAAAA".to_string(),
            text_color_main_disabled: "#BBBBBB".to_string(),
        };

        let stylesheet = tokens.to_stylesheet();

        let expected_stylesheet = "--bg-main:#FFFFFF;\
            --text-color-main:#000000;\
            --text-color-main-dark:#333333;\
            --text-color-main-light:#666666;\
            --border-main:#CCCCCC;\
            --bg-main-disabled:#F0F0F0;\
            --border-main-disabled:#AAAAAA;\
            --text-color-main-disabled:#BBBBBB;"
            .replace("\n", "")
            .replace(" ", "");

        assert_eq!(
            stylesheet.replace("\n", "").replace(" ", ""),
            expected_stylesheet
        );
    }

    #[test]
    fn test_text_tokens_to_stylesheet() {
        let text_tokens = TextTokens {
            p: TextTheme {
                font_size: String::from("10px"),
                font_weight: 500,
                line_height: String::from("12px"),
            },
            h1: TextTheme {
                font_size: String::from("18px"),
                font_weight: 500,
                line_height: String::from("20px"),
            },
            h2: TextTheme {
                font_size: String::from("20px"),
                font_weight: 500,
                line_height: String::from("24px"),
            },
            h3: TextTheme {
                font_size: String::from("24px"),
                font_weight: 500,
                line_height: String::from("28px"),
            },
            bold_weight: 700,
            default_weight: 500,
            light_weight: 400,
            font: String::from("Arial"),
            font_size_lg: String::from("28px"),
            font_size_md: String::from("24px"),
            font_size_sm: String::from("20px"),
        };

        let stylesheet = text_tokens.to_stylesheet();

        assert_eq!(
            "font-weight:500;
            font-family:Arial;
            font-size:24px;
            p{
            font-size:10px;
            font-weight:500;
            line_height:12px;
        }
h1{
            font-size:18px;
            font-weight:500;
            line_height:20px;
        }
h2{
            font-size:20px;
            font-weight:500;
            line_height:24px;
        }
h3{
            font-size:24px;
            font-weight:500;
            line_height:28px;
        }
            .font-light{
                font-weight:400;
            }
            .font-normal{
                font-weight:500;
            }
            .font-bold{
                font-weight:700;
            }
            .text-sm{
                font-size:20px;
            }
            .text-normal{
                font-size:24px;
            }
            .text-lg{
                font-size:28px;
            }"
            .trim(),
            stylesheet.trim()
        );
    }
}
