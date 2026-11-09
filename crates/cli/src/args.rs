use base::fonts::FontsMono;
use clap::{ArgAction, Parser};
use clap_complete::Shell;

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupportedShell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

// 2. Map your enum variant to the actual clap_complete Shell type
impl SupportedShell {
    pub fn to_shell(self) -> Shell {
        match self {
            Self::Bash => Shell::Bash,
            Self::Elvish => Shell::Elvish,
            Self::Fish => Shell::Fish,
            Self::PowerShell => Shell::PowerShell,
            Self::Zsh => Shell::Zsh,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum ValidFonts {
    CascadiaCodeBold,
    CascadiaCodeRegular,
    FiraCodeBold,
    FiraCodeRegular,
    HackBold,
    HackRegular,
    JetBrainsMonoBold,
    JetBrainsMonoRegular,
}

impl From<ValidFonts> for FontsMono {
    fn from(value: ValidFonts) -> Self {
        match value {
            ValidFonts::CascadiaCodeBold => FontsMono::CascadiaCodeNfBold,
            ValidFonts::CascadiaCodeRegular => FontsMono::CascadiaCodeNfRegular,
            ValidFonts::FiraCodeBold => FontsMono::FiraCodeBold,
            ValidFonts::FiraCodeRegular => FontsMono::FiraCodeRegular,
            ValidFonts::HackBold => FontsMono::HackBold,
            ValidFonts::HackRegular => FontsMono::HackRegular,
            ValidFonts::JetBrainsMonoBold => FontsMono::JetBrainsMonoNlBold,
            ValidFonts::JetBrainsMonoRegular => FontsMono::JetBrainsMonoNlRegular,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum BrightnessFunction {
    Rec709,
    Rec601,
    Lightness,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum ColoringFunction {
    BlackAndWhite,
    ImageBased,
}

/// A image parser that converts images to ASCII characters,
/// capable of generating both text files and rendered PNG images.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Generate shell completions
    #[arg(long, value_name = "SHELL")]
    pub generate: Option<SupportedShell>,

    /// Filepath to the image file to parse (image.jpg)
    #[arg(short = 'p', long)]
    pub filepath: Option<String>,

    /// Verbose mode, logs on debug level
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub debug: bool,

    /// Render .txt file aside the image file (image.txt)
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub text_file: bool,

    /// Padding on X axis, both sides (left, right)
    #[arg(short = 'x', long, default_value_t = 10,value_parser = clap::value_parser!(u32).range(5..=30))]
    pub padding_x: u32,

    /// Padding on Y axis, both sides (top, bottom)
    #[arg(short = 'y', long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(5..=30))]
    pub padding_y: u32,

    /// Max side size for ASCII rendered image and txt file.
    #[arg(short, long, default_value_t = 100, value_parser = clap::value_parser!(u32).range(50..=300))]
    pub max_size: u32,

    /// Font size (for ascii image rendering)
    #[arg(short = 'z', long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(5..=30))]
    pub font_size: u32,

    /// Font family (for ascii image rendering)
    #[arg(value_enum, short, long, default_value_t=ValidFonts::FiraCodeBold)]
    pub font_family: ValidFonts,

    /// Parsing brigthness function, each one handle in different ways light pixels in during parsing.
    #[arg(value_enum, short, long,default_value_t=BrightnessFunction::Lightness)]
    pub brightness: BrightnessFunction,

    /// ASCII image coloring function (Black and White or Image Based Coloring).
    #[arg(value_enum, short, long, default_value_t=ColoringFunction::ImageBased)]
    pub coloring: ColoringFunction,
}
