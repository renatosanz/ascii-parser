use base::{BrightnessFn, ColoringFn, FontsMono, ParsingGradient};
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

/// A image parser that converts images to ASCII characters,
/// capable of generating both text files and rendered PNG images.
#[derive(Debug, Parser)]
#[command(name = "ascii-parser-cli", author, version, about, long_about = None)]
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

    /// Skip file collision validation
    #[arg(short='o', long="override", action = ArgAction::SetTrue)]
    pub allow_override: bool,

    /// Skip <image>.txt generation, override --text-file-path <PATH> param
    #[arg(long, action = ArgAction::SetTrue)]
    pub skip_txt: bool,

    /// Generate <image>.txt file aside the image file
    #[arg(short, long)]
    pub text_file_path: Option<String>,

    /// Render <image>_ascii.png file aside the image file
    #[arg(short, long)]
    pub ascii_image_path: Option<String>,

    /// Padding on X axis, both sides (left, right)
    #[arg(short = 'x', long, default_value_t = 3,value_parser = clap::value_parser!(u32).range(0..=30))]
    pub padding_x: u32,

    /// Padding on Y axis, both sides (top, bottom)
    #[arg(short = 'y', long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(0..=30))]
    pub padding_y: u32,

    /// Max side size for ASCII rendered image and txt file.
    #[arg(short, long, default_value_t = 100, value_parser = clap::value_parser!(u32).range(50..=400))]
    pub max_size: u32,

    /// Font size (for ascii image rendering)
    #[arg(short = 'z', long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(5..=30))]
    pub font_size: u32,

    /// Font family (for ascii image rendering)
    #[arg(value_enum, short, long, default_value_t=FontsMono::FiraCodeBold)]
    pub font_family: FontsMono,

    #[arg(value_enum, short, long, default_value_t=ParsingGradient::Standard)]
    pub gradient: ParsingGradient,

    /// Parsing brigthness function, each one handle in different ways light pixels in during parsing.
    #[arg(value_enum, short, long,default_value_t=BrightnessFn::Lightness)]
    pub brightness: BrightnessFn,

    /// ASCII image coloring function (Black and White or Image Based Coloring).
    #[arg(value_enum, short, long, default_value_t=ColoringFn::ImageBased)]
    pub coloring: ColoringFn,
}
