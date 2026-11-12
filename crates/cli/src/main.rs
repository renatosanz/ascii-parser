use clap::{CommandFactory, Parser};
use clap_complete::aot::generate;
use cli::{AppError, AppResult, args::Args, validation::is_image};
use std::{
    io::{self},
    path::{Path, PathBuf},
};

fn main() -> AppResult<()> {
    let args = Args::parse();

    if args.debug {
        simple_logger::init_with_level(log::Level::Trace).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Error).unwrap();
    }

    // generate the completations for the CLI
    if let Some(supported_shell) = args.generate {
        let shell = supported_shell.to_shell();
        let mut cmd = Args::command(); // extract the Clap Command struct
        let bin_name = cmd.get_name().to_string();

        // stream the completion script directly to standard output
        generate(shell, &mut cmd, bin_name, &mut io::stdout());
        return Ok(());
    }

    // check supported filetypes by extensions and mime type
    assert!(infer::is_supported("jpg"));
    assert!(infer::is_supported("png"));
    assert!(infer::is_mime_supported("image/png"));

    // filepath is required just if there's no --generate param
    if args.filepath.is_none() {
        let _ = Args::command().print_help();
        std::process::exit(1);
    }

    let filepath = args.filepath.unwrap();
    let original_image_path = Path::new(&filepath);

    original_image_path
        .exists()
        .ok_or(AppError::InvalidFile("Non existing file."))?;

    // verify the filetype
    if !is_image(original_image_path).map_err(|e| {
        log::error!("Error reading file: {}", e);
        AppError::InvalidFileType
    })? {
        log::info!("Invalid image type!");
        return Err(AppError::InvalidFileType);
    }

    log::info!("Valid image type: {original_image_path:?}");

    let ascii_image_path = match args.ascii_image_path {
        Some(p) => PathBuf::from(p),
        None => {
            let stem = original_image_path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or(AppError::InvalidFile("Invalid file stem."))?;
            original_image_path.with_file_name(format!("{stem}_ascii.png"))
        }
    };

    let text_file_path = match args.text_file_path {
        Some(p) => Some(PathBuf::from(p)),
        None => {
            // if --skip-txt flags is active override the path recieved
            if args.skip_txt {
                None
            } else {
                let stem = original_image_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or(AppError::InvalidFile("Invalid file stem."))?;
                Some(original_image_path.with_file_name(format!("{stem}.txt")))
            }
        }
    };

    log::info!("ascii_image_path {:?}", ascii_image_path);
    log::info!("text_file_path {:?}", text_file_path);

    // verify if there's some collision
    if !args.allow_override {
        if ascii_image_path.exists() {
            return Err(AppError::FileCollision(ascii_image_path));
        }
        if let Some(path) = text_file_path.clone() {
            if path.exists() {
                return Err(AppError::FileCollision(path));
            }
        }
    }

    base::process_image(
        original_image_path,
        &ascii_image_path,
        text_file_path,
        args.max_size,
        args.font_family,
        args.font_size as f32,
        args.padding_x,
        args.padding_y,
        args.gradient,
        args.coloring,
        args.brightness,
    )
    .map_err(|e| AppError::ProcessingError(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use base::{
        fonts::FontsMono,
        parser::{BrightnessFn, ParsingGradient},
        render::ColoringFn,
    };
    use clap::error::ErrorKind;
    use cli::args::SupportedShell;

    fn parse(args: &[&str]) -> Result<Args, clap::Error> {
        Args::try_parse_from(args)
    }

    #[test]
    fn uses_expected_defaults() -> Result<(), clap::Error> {
        let args = parse(&["cli"])?;

        assert_eq!(args.generate, None);
        assert_eq!(args.filepath, None);
        assert!(!args.debug);
        assert_eq!(args.text_file_path, None);
        assert_eq!(args.ascii_image_path, None);
        assert_eq!(args.padding_x, 3);
        assert_eq!(args.padding_y, 3);
        assert_eq!(args.max_size, 100);
        assert_eq!(args.font_size, 10);
        assert_eq!(args.font_family, FontsMono::FiraCodeBold);
        assert_eq!(args.gradient, ParsingGradient::Standard);
        assert_eq!(args.brightness, BrightnessFn::Lightness);
        assert_eq!(args.coloring, ColoringFn::ImageBased);

        Ok(())
    }

    #[test]
    fn parses_filepath_with_short_and_long_flags() -> Result<(), clap::Error> {
        let short = parse(&["cli", "-p", "image.jpg"])?;
        assert_eq!(short.filepath.as_deref(), Some("image.jpg"));

        let long = parse(&["cli", "--filepath", "photo.png"])?;
        assert_eq!(long.filepath.as_deref(), Some("photo.png"));

        Ok(())
    }

    #[test]
    fn parses_debug_and_output_paths() -> Result<(), clap::Error> {
        let short = parse(&["cli", "-d", "-t", "out.txt", "-a", "out.png"])?;
        assert!(short.debug);
        assert_eq!(short.text_file_path.as_deref(), Some("out.txt"));
        assert_eq!(short.ascii_image_path.as_deref(), Some("out.png"));

        let long = parse(&[
            "cli",
            "--debug",
            "--text-file-path",
            "out.txt",
            "--ascii-image-path",
            "out.png",
        ])?;
        assert!(long.debug);
        assert_eq!(long.text_file_path.as_deref(), Some("out.txt"));
        assert_eq!(long.ascii_image_path.as_deref(), Some("out.png"));

        Ok(())
    }

    #[test]
    fn parses_padding_with_short_and_long_flags() -> Result<(), clap::Error> {
        let short = parse(&["cli", "-x", "15", "-y", "20"])?;
        assert_eq!(short.padding_x, 15);
        assert_eq!(short.padding_y, 20);

        let long = parse(&["cli", "--padding-x", "25", "--padding-y", "30"])?;
        assert_eq!(long.padding_x, 25);
        assert_eq!(long.padding_y, 30);

        Ok(())
    }

    #[test]
    fn parses_max_size_and_font_size() -> Result<(), clap::Error> {
        let args = parse(&["cli", "--max-size", "250", "--font-size", "18"])?;
        assert_eq!(args.max_size, 250);
        assert_eq!(args.font_size, 18);

        let short = parse(&["cli", "-m", "200", "-z", "12"])?;
        assert_eq!(short.max_size, 200);
        assert_eq!(short.font_size, 12);

        Ok(())
    }

    #[test]
    fn accepts_range_boundaries() -> Result<(), clap::Error> {
        let args = parse(&[
            "cli",
            "--padding-x",
            "0",
            "--padding-y",
            "30",
            "--max-size",
            "50",
            "--font-size",
            "30",
        ])?;
        assert_eq!(args.padding_x, 0);
        assert_eq!(args.padding_y, 30);
        assert_eq!(args.max_size, 50);
        assert_eq!(args.font_size, 30);

        let args = parse(&[
            "cli",
            "--padding-x",
            "30",
            "--padding-y",
            "0",
            "--max-size",
            "400",
            "--font-size",
            "5",
        ])?;
        assert_eq!(args.padding_x, 30);
        assert_eq!(args.padding_y, 0);
        assert_eq!(args.max_size, 400);
        assert_eq!(args.font_size, 5);

        Ok(())
    }

    #[test]
    fn rejects_values_outside_allowed_ranges() {
        let invalid = [
            ["cli", "--padding-x", "31"],
            ["cli", "--padding-y", "31"],
            ["cli", "--max-size", "49"],
            ["cli", "--max-size", "401"],
            ["cli", "--font-size", "4"],
            ["cli", "--font-size", "31"],
        ];

        for args in invalid {
            assert!(
                Args::try_parse_from(args).is_err(),
                "expected {args:?} to be rejected"
            );
        }
    }

    #[test]
    fn parses_all_font_families() -> Result<(), clap::Error> {
        let cases = [
            ("cascadia-code-bold", FontsMono::CascadiaCodeBold),
            ("cascadia-code-regular", FontsMono::CascadiaCodeRegular),
            ("fira-code-bold", FontsMono::FiraCodeBold),
            ("fira-code-regular", FontsMono::FiraCodeRegular),
            ("hack-bold", FontsMono::HackBold),
            ("hack-regular", FontsMono::HackRegular),
            ("jet-brains-mono-bold", FontsMono::JetBrainsMonoBold),
            ("jet-brains-mono-regular", FontsMono::JetBrainsMonoRegular),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--font-family", value])?;
            assert_eq!(args.font_family, expected);
        }

        let short = parse(&["cli", "-f", "hack-regular"])?;
        assert_eq!(short.font_family, FontsMono::HackRegular);

        Ok(())
    }

    #[test]
    fn parses_all_brightness_functions() -> Result<(), clap::Error> {
        let cases = [
            ("rec709", BrightnessFn::Rec709),
            ("rec601", BrightnessFn::Rec601),
            ("lightness", BrightnessFn::Lightness),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--brightness", value])?;
            assert_eq!(args.brightness, expected);
        }

        let short = parse(&["cli", "-b", "rec601"])?;
        assert_eq!(short.brightness, BrightnessFn::Rec601);

        Ok(())
    }

    #[test]
    fn parses_all_coloring_functions() -> Result<(), clap::Error> {
        let cases = [
            ("black-and-white", ColoringFn::BlackAndWhite),
            ("image-based", ColoringFn::ImageBased),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--coloring", value])?;
            assert_eq!(args.coloring, expected);
        }

        let short = parse(&["cli", "-c", "black-and-white"])?;
        assert_eq!(short.coloring, ColoringFn::BlackAndWhite);

        Ok(())
    }

    #[test]
    fn parses_all_gradients() -> Result<(), clap::Error> {
        let cases = [
            ("standard", ParsingGradient::Standard),
            ("jp2a", ParsingGradient::JP2A),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--gradient", value])?;
            assert_eq!(args.gradient, expected);
        }

        let short = parse(&["cli", "-g", "jp2a"])?;
        assert_eq!(short.gradient, ParsingGradient::JP2A);

        Ok(())
    }

    #[test]
    fn parses_all_supported_shells() -> Result<(), clap::Error> {
        let cases = [
            ("bash", SupportedShell::Bash),
            ("elvish", SupportedShell::Elvish),
            ("fish", SupportedShell::Fish),
            ("power-shell", SupportedShell::PowerShell),
            ("zsh", SupportedShell::Zsh),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--generate", value])?;
            assert_eq!(args.generate, Some(expected));
        }

        Ok(())
    }

    #[test]
    fn maps_supported_shells_to_completion_shells() {
        use clap_complete::Shell;

        assert_eq!(SupportedShell::Bash.to_shell(), Shell::Bash);
        assert_eq!(SupportedShell::Elvish.to_shell(), Shell::Elvish);
        assert_eq!(SupportedShell::Fish.to_shell(), Shell::Fish);
        assert_eq!(SupportedShell::PowerShell.to_shell(), Shell::PowerShell);
        assert_eq!(SupportedShell::Zsh.to_shell(), Shell::Zsh);
    }

    #[test]
    fn rejects_unknown_arguments_and_invalid_enum_values() {
        let invalid: &[&[&str]] = &[
            &["cli", "--unknown"],
            &["cli", "--font-family", "comic-sans"],
            &["cli", "--brightness", "average"],
            &["cli", "--coloring", "rainbow"],
            &["cli", "--generate", "tcsh"],
        ];

        for &args in invalid {
            assert!(
                Args::try_parse_from(args).is_err(),
                "expected {args:?} to be rejected"
            );
        }
    }

    #[test]
    fn rejects_option_without_value() {
        let Err(err) = Args::try_parse_from(["cli", "--filepath"]) else {
            panic!("expected parsing to fail when an option has no value");
        };
        assert_eq!(err.kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn parses_a_full_set_of_arguments() -> Result<(), clap::Error> {
        let args = parse(&[
            "cli",
            "-p",
            "image.jpg",
            "-d",
            "-t",
            "image.txt",
            "-a",
            "image_ascii.png",
            "-x",
            "12",
            "-y",
            "14",
            "-m",
            "120",
            "-z",
            "16",
            "-f",
            "fira-code-regular",
            "-g",
            "jp2a",
            "-b",
            "rec709",
            "-c",
            "black-and-white",
        ])?;

        assert_eq!(args.filepath.as_deref(), Some("image.jpg"));
        assert!(args.debug);
        assert_eq!(args.text_file_path.as_deref(), Some("image.txt"));
        assert_eq!(args.ascii_image_path.as_deref(), Some("image_ascii.png"));
        assert_eq!(args.padding_x, 12);
        assert_eq!(args.padding_y, 14);
        assert_eq!(args.max_size, 120);
        assert_eq!(args.font_size, 16);
        assert_eq!(args.font_family, FontsMono::FiraCodeRegular);
        assert_eq!(args.gradient, ParsingGradient::JP2A);
        assert_eq!(args.brightness, BrightnessFn::Rec709);
        assert_eq!(args.coloring, ColoringFn::BlackAndWhite);

        Ok(())
    }

    #[test]
    fn supports_help_and_version_flags() {
        let Err(help) = Args::try_parse_from(["cli", "--help"]) else {
            panic!("expected --help to display help");
        };
        assert_eq!(help.kind(), ErrorKind::DisplayHelp);

        let Err(version) = Args::try_parse_from(["cli", "--version"]) else {
            panic!("expected --version to display the version");
        };
        assert_eq!(version.kind(), ErrorKind::DisplayVersion);
    }
}
