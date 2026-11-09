use ab_glyph::FontRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontsMono {
    CascadiaCodeNfBold,
    CascadiaCodeNfRegular,
    FiraCodeBold,
    FiraCodeRegular,
    HackBold,
    HackRegular,
    JetBrainsMonoNlBold,
    JetBrainsMonoNlRegular,
}

impl FontsMono {
    pub fn bytes(self) -> &'static [u8] {
        match self {
            FontsMono::CascadiaCodeNfBold => {
                include_bytes!("../assets/fonts/CascadiaCodeNF-Bold.ttf")
            }
            FontsMono::CascadiaCodeNfRegular => {
                include_bytes!("../assets/fonts/CascadiaCodeNF-Regular.ttf")
            }
            FontsMono::FiraCodeBold => include_bytes!("../assets/fonts/FiraCode-Bold.ttf"),
            FontsMono::FiraCodeRegular => include_bytes!("../assets/fonts/FiraCode-Regular.ttf"),
            FontsMono::HackBold => include_bytes!("../assets/fonts/Hack-Bold.ttf"),
            FontsMono::HackRegular => include_bytes!("../assets/fonts/Hack-Regular.ttf"),
            FontsMono::JetBrainsMonoNlBold => {
                include_bytes!("../assets/fonts/JetBrainsMonoNL-Bold.ttf")
            }
            FontsMono::JetBrainsMonoNlRegular => {
                include_bytes!("../assets/fonts/JetBrainsMonoNL-Regular.ttf")
            }
        }
    }

    pub fn font_ref(self) -> Result<FontRef<'static>, ab_glyph::InvalidFont> {
        FontRef::try_from_slice(self.bytes())
    }

    pub fn name(self) -> &'static str {
        match self {
            FontsMono::CascadiaCodeNfBold => "Cascadia Code NF Bold",
            FontsMono::CascadiaCodeNfRegular => "Cascadia Code NF Regular",
            FontsMono::FiraCodeBold => "Fira Code Bold",
            FontsMono::FiraCodeRegular => "Fira Code Regular",
            FontsMono::HackBold => "Hack Bold",
            FontsMono::HackRegular => "Hack Regular",
            FontsMono::JetBrainsMonoNlBold => "JetBrains Mono NL Bold",
            FontsMono::JetBrainsMonoNlRegular => "JetBrains Mono NL Regular",
        }
    }
}
