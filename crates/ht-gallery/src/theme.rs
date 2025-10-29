// Gallery theme system - colors, typography, spacing

use crate::types::Color;
use serde::{Deserialize, Serialize};

/// Gallery visual theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryTheme {
    pub name: String,

    // Cards
    pub card_background: Color,
    pub card_border: Color,
    pub card_hover_overlay: Color,
    pub card_selected_border: Color,

    // Text
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_hint: Color,

    // Actions
    pub action_primary: Color,
    pub action_hover: Color,
    pub action_success: Color,
    pub action_warning: Color,
    pub action_error: Color,

    // Accents
    pub accent_gradient_start: Color,
    pub accent_gradient_end: Color,
    pub glass_tint: Color,
    pub shadow_color: Color,

    // Spacing
    pub spacing: SpacingTheme,

    // Typography
    pub typography: TypographyTheme,

    // Effects
    pub effects: EffectsTheme,
}

impl Default for GalleryTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl GalleryTheme {
    /// Dark theme (default)
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),

            // Cards - Dark glass aesthetic
            card_background: Color::rgba(18, 18, 18, 230), // Almost black with slight transparency
            card_border: Color::rgba(255, 255, 255, 25),  // Subtle white border
            card_hover_overlay: Color::rgba(255, 255, 255, 13), // 5% white overlay on hover
            card_selected_border: Color::rgba(100, 150, 255, 200), // Blue selection

            // Text - High contrast
            text_primary: Color::rgba(255, 255, 255, 242),   // 95% white
            text_secondary: Color::rgba(255, 255, 255, 179), // 70% white
            text_hint: Color::rgba(255, 255, 255, 128),      // 50% white

            // Actions - Beautiful blues and greens
            action_primary: Color::rgb(100, 149, 237),   // Cornflower blue
            action_hover: Color::rgb(123, 165, 255),     // Lighter blue
            action_success: Color::rgb(76, 175, 80),     // Green
            action_warning: Color::rgb(255, 152, 0),     // Orange
            action_error: Color::rgb(244, 67, 54),       // Red

            // Accents
            accent_gradient_start: Color::rgba(100, 149, 237, 180),
            accent_gradient_end: Color::rgba(147, 51, 234, 180), // Purple
            glass_tint: Color::rgba(11, 15, 25, 230),            // Dark blue tint
            shadow_color: Color::rgba(0, 0, 0, 128),             // 50% black

            spacing: SpacingTheme::default(),
            typography: TypographyTheme::default(),
            effects: EffectsTheme::default(),
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),

            // Cards - Light glass
            card_background: Color::rgba(255, 255, 255, 230),
            card_border: Color::rgba(0, 0, 0, 25),
            card_hover_overlay: Color::rgba(0, 0, 0, 13),
            card_selected_border: Color::rgba(25, 118, 210, 255),

            // Text
            text_primary: Color::rgba(0, 0, 0, 219),   // 86% black
            text_secondary: Color::rgba(0, 0, 0, 153), // 60% black
            text_hint: Color::rgba(0, 0, 0, 102),      // 40% black

            // Actions
            action_primary: Color::rgb(25, 118, 210),
            action_hover: Color::rgb(66, 165, 245),
            action_success: Color::rgb(67, 160, 71),
            action_warning: Color::rgb(251, 140, 0),
            action_error: Color::rgb(229, 57, 53),

            // Accents
            accent_gradient_start: Color::rgba(25, 118, 210, 180),
            accent_gradient_end: Color::rgba(156, 39, 176, 180),
            glass_tint: Color::rgba(240, 242, 245, 230),
            shadow_color: Color::rgba(0, 0, 0, 64),

            spacing: SpacingTheme::default(),
            typography: TypographyTheme::default(),
            effects: EffectsTheme::default(),
        }
    }

    /// High contrast theme for accessibility
    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),

            card_background: Color::rgb(0, 0, 0),
            card_border: Color::rgb(255, 255, 255),
            card_hover_overlay: Color::rgba(255, 255, 255, 51),
            card_selected_border: Color::rgb(255, 255, 0), // Yellow for visibility

            text_primary: Color::rgb(255, 255, 255),
            text_secondary: Color::rgb(200, 200, 200),
            text_hint: Color::rgb(150, 150, 150),

            action_primary: Color::rgb(0, 255, 255),   // Cyan
            action_hover: Color::rgb(128, 255, 255),
            action_success: Color::rgb(0, 255, 0),
            action_warning: Color::rgb(255, 255, 0),
            action_error: Color::rgb(255, 0, 0),

            accent_gradient_start: Color::rgb(0, 255, 255),
            accent_gradient_end: Color::rgb(255, 0, 255),
            glass_tint: Color::rgb(0, 0, 0),
            shadow_color: Color::rgba(255, 255, 255, 128),

            spacing: SpacingTheme::default(),
            typography: TypographyTheme::default(),
            effects: EffectsTheme {
                blur_radius: 0.0, // No blur for high contrast
                shadow_enabled: true,
                ..Default::default()
            },
        }
    }

    /// Get shadow color with specified opacity
    pub fn shadow(&self, opacity: f32) -> Color {
        let mut color = self.shadow_color;
        color.a = opacity;
        color
    }

    /// Get elevation shadow for given level (0-4)
    pub fn elevation_shadow(&self, level: f32) -> ShadowStyle {
        let level = level.clamp(0.0, 4.0);

        match level as i32 {
            0 => ShadowStyle::none(),
            1 => ShadowStyle {
                offset_y: 2.0,
                blur_radius: 4.0,
                opacity: 0.1,
            },
            2 => ShadowStyle {
                offset_y: 4.0,
                blur_radius: 8.0,
                opacity: 0.15,
            },
            3 => ShadowStyle {
                offset_y: 8.0,
                blur_radius: 16.0,
                opacity: 0.2,
            },
            4 => ShadowStyle {
                offset_y: 16.0,
                blur_radius: 32.0,
                opacity: 0.25,
            },
            _ => {
                // Interpolate between levels
                let lower = level.floor();
                let upper = level.ceil();
                let t = level - lower;

                let lower_shadow = self.elevation_shadow(lower);
                let upper_shadow = self.elevation_shadow(upper);

                ShadowStyle {
                    offset_y: lower_shadow.offset_y + (upper_shadow.offset_y - lower_shadow.offset_y) * t,
                    blur_radius: lower_shadow.blur_radius + (upper_shadow.blur_radius - lower_shadow.blur_radius) * t,
                    opacity: lower_shadow.opacity + (upper_shadow.opacity - lower_shadow.opacity) * t,
                }
            }
        }
    }
}

/// Spacing theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingTheme {
    pub xs: f32,      // 4px
    pub sm: f32,      // 8px
    pub md: f32,      // 12px
    pub lg: f32,      // 16px
    pub xl: f32,      // 24px
    pub xxl: f32,     // 32px

    pub card_padding: f32,      // 12px
    pub card_spacing: f32,      // 12px
    pub gallery_padding: f32,   // 16px
    pub button_spacing: f32,    // 8px
}

impl Default for SpacingTheme {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            xxl: 32.0,

            card_padding: 12.0,
            card_spacing: 12.0,
            gallery_padding: 16.0,
            button_spacing: 8.0,
        }
    }
}

/// Typography theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyTheme {
    pub font_family: String,

    // Sizes
    pub size_xs: f32,      // 10px
    pub size_sm: f32,      // 11px
    pub size_md: f32,      // 12px
    pub size_lg: f32,      // 14px
    pub size_xl: f32,      // 16px
    pub size_xxl: f32,     // 20px

    // Specific uses
    pub card_title_size: f32,      // 14px
    pub card_metadata_size: f32,   // 11px
    pub card_description_size: f32, // 12px
    pub action_label_size: f32,    // 12px
    pub gallery_title_size: f32,   // 16px

    // Line heights
    pub line_height_tight: f32,   // 1.2
    pub line_height_normal: f32,  // 1.5
    pub line_height_relaxed: f32, // 1.75
}

impl Default for TypographyTheme {
    fn default() -> Self {
        Self {
            font_family: "Inter, 'SF Pro', -apple-system, sans-serif".to_string(),

            size_xs: 10.0,
            size_sm: 11.0,
            size_md: 12.0,
            size_lg: 14.0,
            size_xl: 16.0,
            size_xxl: 20.0,

            card_title_size: 14.0,
            card_metadata_size: 11.0,
            card_description_size: 12.0,
            action_label_size: 12.0,
            gallery_title_size: 16.0,

            line_height_tight: 1.2,
            line_height_normal: 1.5,
            line_height_relaxed: 1.75,
        }
    }
}

/// Visual effects theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsTheme {
    pub blur_radius: f32,
    pub border_radius: f32,
    pub shadow_enabled: bool,
    pub animations_enabled: bool,
    pub hover_scale: f32,
    pub transition_duration_ms: u32,
}

impl Default for EffectsTheme {
    fn default() -> Self {
        Self {
            blur_radius: 24.0,
            border_radius: 8.0,
            shadow_enabled: true,
            animations_enabled: true,
            hover_scale: 1.03,
            transition_duration_ms: 200,
        }
    }
}

/// Shadow styling
#[derive(Debug, Clone, Copy)]
pub struct ShadowStyle {
    pub offset_y: f32,
    pub blur_radius: f32,
    pub opacity: f32,
}

impl ShadowStyle {
    pub fn none() -> Self {
        Self {
            offset_y: 0.0,
            blur_radius: 0.0,
            opacity: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_themes() {
        let dark = GalleryTheme::dark();
        let light = GalleryTheme::light();
        let hc = GalleryTheme::high_contrast();

        assert_eq!(dark.name, "Dark");
        assert_eq!(light.name, "Light");
        assert_eq!(hc.name, "High Contrast");
    }

    #[test]
    fn test_elevation_shadow() {
        let theme = GalleryTheme::dark();

        let shadow0 = theme.elevation_shadow(0.0);
        let shadow1 = theme.elevation_shadow(1.0);
        let shadow2 = theme.elevation_shadow(2.0);

        assert_eq!(shadow0.offset_y, 0.0);
        assert!(shadow1.offset_y > shadow0.offset_y);
        assert!(shadow2.offset_y > shadow1.offset_y);
    }

    #[test]
    fn test_spacing() {
        let spacing = SpacingTheme::default();

        assert_eq!(spacing.xs, 4.0);
        assert_eq!(spacing.sm, 8.0);
        assert_eq!(spacing.md, 12.0);
        assert!(spacing.xl > spacing.lg);
    }
}
