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
            ColorGreen100 => 0xE8F5E9,
            ColorGreen300 => 0xA5D6A7,
            ColorGreen500 => 0x66BB6A,
            ColorGreen800 => 0x1B5E20,

            ColorWhite => 0xFFFFFF,
            ColorGray50 => 0xF9FAFB,
            ColorGray100 => 0xF3F4F6,
            ColorGray200 => 0xE5E7EB,
            ColorGray400 => 0x9CA3AF,
            ColorGray600 => 0x4B5563,
            ColorGray800 => 0x1F2937,
            ColorGray900 => 0x111827,

            BgApp => 0xF9FAFB,
            BgSurface => 0xFFFFFF,
            BgSurfaceHover => 0xF3F4F6,
            BgBrandSubtle => 0xE8F5E9,
            BgBrand => 0x66BB6A,
            BgBrandHover => 0x1B5E20,

            TextPrimary => 0x111827,
            TextSecondary => 0x4B5563,
            TextMuted => 0x9CA3AF,
            TextOnBrand => 0xFFFFFF,
            TextBrand => 0x1B5E20,

            BorderSubtle => 0xE5E7EB,
            BorderStrong => 0x9CA3AF,
            BorderBrand => 0x66BB6A,

            FocusRing => 0x66BB6A,
            FocusRingOffset => 0xFFFFFF,

            ColorSuccessText => 0x1B5E20,
            ColorSuccessBg => 0xE8F5E9,
            ColorErrorText => 0x991B1B,
            ColorErrorBg => 0xFEE2E2,
            ColorWarningText => 0x92400E,
            ColorWarningBg => 0xFEF3C7,
            ColorInfoText => 0x075985,
            ColorInfoBg => 0xE0F2FE,

            BgAppDark => 0x111827,
            BgSurfaceDark => 0x1F2937,
            BgSurfaceHoverDark => 0x4B5563,
            BgBrandSubtleDark => 0x153B18,
            TextPrimaryDark => 0xF9FAFB,
            TextSecondaryDark => 0x9CA3AF,
            TextMutedDark => 0x4B5563,
            BorderSubtleDark => 0x4B5563,
            BorderStrongDark => 0x9CA3AF,
        }
    }
}

/// Convenience helper: format the token as a `#RRGGBB` string.
impl ColorScheme {
    pub fn to_hex_string(self) -> String {
        format!("#{:06X}", u32::from(self))
    }
}
