use base::fonts::FontsMono;

use clap::{ArgAction, Parser};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum ValidCLIFonts {
    CascadiaCodeBold,
    CascadiaCodeRegular,
    FiraCodeBold,
    FiraCodeRegular,
    HackBold,
    HackRegular,
    JetBrainsMonoBold,
    JetBrainsMonoRegular,
}

impl From<ValidCLIFonts> for FontsMono {
    fn from(value: ValidCLIFonts) -> Self {
        match value {
            ValidCLIFonts::CascadiaCodeBold => FontsMono::CascadiaCodeNfBold,
            ValidCLIFonts::CascadiaCodeRegular => FontsMono::CascadiaCodeNfRegular,
            ValidCLIFonts::FiraCodeBold => FontsMono::FiraCodeBold,
            ValidCLIFonts::FiraCodeRegular => FontsMono::FiraCodeRegular,
            ValidCLIFonts::HackBold => FontsMono::HackBold,
            ValidCLIFonts::HackRegular => FontsMono::HackRegular,
            ValidCLIFonts::JetBrainsMonoBold => FontsMono::JetBrainsMonoNlBold,
            ValidCLIFonts::JetBrainsMonoRegular => FontsMono::JetBrainsMonoNlRegular,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum BrightnessFunction {
    Rec709,
    Rec601,
    Lightness,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum ColoringFunction {
    BlackAndWhite,
    ImageBased,
}

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filepath to the image file to parse (image.jpg)
    #[arg(index = 1)]
    filepath: String,

    /// Verbose mode, logs on debug level
    #[arg(short, long, action = ArgAction::SetTrue)]
    debug: bool,

    /// Render .txt file aside the image file (image.txt)
    #[arg(short, long, action = ArgAction::SetTrue)]
    text_file: bool,

    /// Padding on X axis, both sides (left, right)
    #[arg(short = 'x', long, default_value_t = 10,value_parser = clap::value_parser!(u32).range(5..=30))]
    padding_x: u32,

    /// Padding on Y axis, both sides (top, bottom)
    #[arg(short = 'y', long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(5..=30))]
    padding_y: u32,

    /// Max side size for ASCII rendered image and txt file.
    #[arg(short, long, default_value_t = 100, value_parser = clap::value_parser!(u32).range(50..=300))]
    max_size: u32,

    /// Font size (for ascii image rendering)
    #[arg(short = 'z', long, default_value_t = 10, value_parser = clap::value_parser!(u32).range(5..=30))]
    font_size: u32,

    /// Font family (for ascii image rendering)
    #[arg(value_enum, short, long, default_value_t=ValidCLIFonts::FiraCodeBold)]
    font_family: ValidCLIFonts,

    /// Parsing brigthness function, each one handle in different ways light pixels in during parsing.
    #[arg(value_enum, short, long,default_value_t=BrightnessFunction::Lightness)]
    brightness: BrightnessFunction,

    /// ASCII image coloring function (Black and White or Image Based Coloring).
    #[arg(value_enum, short, long, default_value_t=ColoringFunction::ImageBased)]
    coloring: ColoringFunction,
}

fn main() {
    let args = Args::parse();
    println!("Hello {:?}!", args);
}
