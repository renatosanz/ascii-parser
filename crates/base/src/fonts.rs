use ab_glyph::FontRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum FontsMono {
    CascadiaCodeBold,
    CascadiaCodeRegular,
    FiraCodeBold,
    FiraCodeRegular,
    HackBold,
    HackRegular,
    JetBrainsMonoBold,
    JetBrainsMonoRegular,
}

impl FontsMono {
    pub fn bytes(self) -> &'static [u8] {
        match self {
            FontsMono::CascadiaCodeBold => {
                include_bytes!("../assets/fonts/CascadiaCodeNF-Bold.ttf")
            }
            FontsMono::CascadiaCodeRegular => {
                include_bytes!("../assets/fonts/CascadiaCodeNF-Regular.ttf")
            }
            FontsMono::FiraCodeBold => include_bytes!("../assets/fonts/FiraCode-Bold.ttf"),
            FontsMono::FiraCodeRegular => include_bytes!("../assets/fonts/FiraCode-Regular.ttf"),
            FontsMono::HackBold => include_bytes!("../assets/fonts/Hack-Bold.ttf"),
            FontsMono::HackRegular => include_bytes!("../assets/fonts/Hack-Regular.ttf"),
            FontsMono::JetBrainsMonoBold => {
                include_bytes!("../assets/fonts/JetBrainsMonoNL-Bold.ttf")
            }
            FontsMono::JetBrainsMonoRegular => {
                include_bytes!("../assets/fonts/JetBrainsMonoNL-Regular.ttf")
            }
        }
    }

    pub fn font_ref(self) -> Result<FontRef<'static>, ab_glyph::InvalidFont> {
        log::info!("Loading font: {}", self.name());
        FontRef::try_from_slice(self.bytes())
    }

    pub fn name(self) -> &'static str {
        match self {
            FontsMono::CascadiaCodeBold => "Cascadia Code NF Bold",
            FontsMono::CascadiaCodeRegular => "Cascadia Code NF Regular",
            FontsMono::FiraCodeBold => "Fira Code Bold",
            FontsMono::FiraCodeRegular => "Fira Code Regular",
            FontsMono::HackBold => "Hack Bold",
            FontsMono::HackRegular => "Hack Regular",
            FontsMono::JetBrainsMonoBold => "JetBrains Mono NL Bold",
            FontsMono::JetBrainsMonoRegular => "JetBrains Mono NL Regular",
        }
    }
}
