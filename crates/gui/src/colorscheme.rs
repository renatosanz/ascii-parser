#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorScheme {
    // Base Brand Colors
    ColorGreen100,
    ColorGreen300,
    ColorGreen500,
    ColorGreen800,

    // Neutral Palette
    ColorWhite,
    ColorGray50,
    ColorGray100,
    ColorGray200,
    ColorGray400,
    ColorGray600,
    ColorGray800,
    ColorGray900,

    // Semantic: Backgrounds
    BgApp,
    BgSurface,
    BgSurfaceHover,
    BgBrandSubtle,
    BgBrand,
    BgBrandHover,

    // Semantic: Text
    TextPrimary,
    TextSecondary,
    TextMuted,
    TextOnBrand,
    TextBrand,

    // Semantic: Borders & Dividers
    BorderSubtle,
    BorderStrong,
    BorderBrand,

    // Semantic: Interactive States
    FocusRing,
    FocusRingOffset,

    // Status Colors
    ColorSuccessText,
    ColorSuccessBg,
    ColorErrorText,
    ColorErrorBg,
    ColorWarningText,
    ColorWarningBg,
    ColorInfoText,
    ColorInfoBg,

    // Dark Mode Overrides
    BgAppDark,
    BgSurfaceDark,
    BgSurfaceHoverDark,
    BgBrandSubtleDark,
    TextPrimaryDark,
    TextSecondaryDark,
    TextMutedDark,
    BorderSubtleDark,
    BorderStrongDark,
}

impl From<ColorScheme> for u32 {
    fn from(token: ColorScheme) -> Self {
        use ColorScheme::*;
        match token {
            ColorGreen100 => 0xE4EDE6,
            ColorGreen300 => 0xA8C6B0,
            ColorGreen500 => 0x2F7A53,
            ColorGreen800 => 0x174A32,

            ColorWhite => 0xFFFFFF,
            ColorGray50 => 0xF7F6F2,
            ColorGray100 => 0xECEBE5,
            ColorGray200 => 0xDEDDD6,
            ColorGray400 => 0xA9AFA6,
            ColorGray600 => 0x565D56,
            ColorGray800 => 0x2B302B,
            ColorGray900 => 0x171A17,

            BgApp => 0xF3F2ED,
            BgSurface => 0xFFFFFF,
            BgSurfaceHover => 0xECEBE5,
            BgBrandSubtle => 0xE4EDE6,
            BgBrand => 0x1F5C3F,
            BgBrandHover => 0x174A32,

            TextPrimary => 0x171A17,
            TextSecondary => 0x565D56,
            TextMuted => 0x8A9088,
            TextOnBrand => 0xFFFFFF,
            TextBrand => 0x1F5C3F,

            BorderSubtle => 0xDEDDD6,
            BorderStrong => 0xB9B8AF,
            BorderBrand => 0x1F5C3F,

            FocusRing => 0x1F5C3F,
            FocusRingOffset => 0xFFFFFF,

            ColorSuccessText => 0x1F5C3F,
            ColorSuccessBg => 0xE4EDE6,
            ColorErrorText => 0xA32A2A,
            ColorErrorBg => 0xF7E4E1,
            ColorWarningText => 0x8A5A16,
            ColorWarningBg => 0xF6EBD8,
            ColorInfoText => 0x275E75,
            ColorInfoBg => 0xE2EEF2,

            BgAppDark => 0x171A17,
            BgSurfaceDark => 0x2B302B,
            BgSurfaceHoverDark => 0x3A403A,
            BgBrandSubtleDark => 0x1B3D2B,
            TextPrimaryDark => 0xF7F6F2,
            TextSecondaryDark => 0xB9B8AF,
            TextMutedDark => 0x8A9088,
            BorderSubtleDark => 0x3A403A,
            BorderStrongDark => 0x565D56,
        }
    }
}

/// Convenience helper: format the token as a `#RRGGBB` string.
impl ColorScheme {
    pub fn to_hex_string(self) -> String {
        format!("#{:06X}", u32::from(self))
    }
}
