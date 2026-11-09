use clap::{CommandFactory, Parser};
use clap_complete::aot::generate;
use cli::args::Args;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Hello {:?}!", args);

    if let Some(supported_shell) = args.generate {
        let shell = supported_shell.to_shell();
        let mut cmd = Args::command(); // Extract the Clap Command struct
        let bin_name = cmd.get_name().to_string();

        // Stream the completion script directly to standard output
        generate(shell, &mut cmd, bin_name, &mut io::stdout());
        return Ok(());
    }

    if args.filepath.is_none() {
        Args::command().print_help()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;
    use cli::args::{BrightnessFunction, ColoringFunction, SupportedShell, ValidFonts};

    fn parse(args: &[&str]) -> Result<Args, clap::Error> {
        Args::try_parse_from(args)
    }

    #[test]
    fn uses_expected_defaults() -> Result<(), clap::Error> {
        let args = parse(&["cli"])?;

        assert_eq!(args.generate, None);
        assert_eq!(args.filepath, None);
        assert!(!args.debug);
        assert!(!args.text_file);
        assert_eq!(args.padding_x, 10);
        assert_eq!(args.padding_y, 10);
        assert_eq!(args.max_size, 100);
        assert_eq!(args.font_size, 10);
        assert_eq!(args.font_family, ValidFonts::FiraCodeBold);
        assert_eq!(args.brightness, BrightnessFunction::Lightness);
        assert_eq!(args.coloring, ColoringFunction::ImageBased);

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
    fn parses_boolean_flags() -> Result<(), clap::Error> {
        let short = parse(&["cli", "-d", "-t"])?;
        assert!(short.debug);
        assert!(short.text_file);

        let long = parse(&["cli", "--debug", "--text-file"])?;
        assert!(long.debug);
        assert!(long.text_file);

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
            "5",
            "--padding-y",
            "30",
            "--max-size",
            "50",
            "--font-size",
            "30",
        ])?;
        assert_eq!(args.padding_x, 5);
        assert_eq!(args.padding_y, 30);
        assert_eq!(args.max_size, 50);
        assert_eq!(args.font_size, 30);

        let args = parse(&[
            "cli",
            "--padding-x",
            "30",
            "--padding-y",
            "5",
            "--max-size",
            "300",
            "--font-size",
            "5",
        ])?;
        assert_eq!(args.padding_x, 30);
        assert_eq!(args.padding_y, 5);
        assert_eq!(args.max_size, 300);
        assert_eq!(args.font_size, 5);

        Ok(())
    }

    #[test]
    fn rejects_values_outside_allowed_ranges() {
        let invalid = [
            ["cli", "--padding-x", "4"],
            ["cli", "--padding-x", "31"],
            ["cli", "--padding-y", "4"],
            ["cli", "--padding-y", "31"],
            ["cli", "--max-size", "49"],
            ["cli", "--max-size", "301"],
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
            ("cascadia-code-bold", ValidFonts::CascadiaCodeBold),
            ("cascadia-code-regular", ValidFonts::CascadiaCodeRegular),
            ("fira-code-bold", ValidFonts::FiraCodeBold),
            ("fira-code-regular", ValidFonts::FiraCodeRegular),
            ("hack-bold", ValidFonts::HackBold),
            ("hack-regular", ValidFonts::HackRegular),
            ("jet-brains-mono-bold", ValidFonts::JetBrainsMonoBold),
            ("jet-brains-mono-regular", ValidFonts::JetBrainsMonoRegular),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--font-family", value])?;
            assert_eq!(args.font_family, expected);
        }

        let short = parse(&["cli", "-f", "hack-regular"])?;
        assert_eq!(short.font_family, ValidFonts::HackRegular);

        Ok(())
    }

    #[test]
    fn parses_all_brightness_functions() -> Result<(), clap::Error> {
        let cases = [
            ("rec709", BrightnessFunction::Rec709),
            ("rec601", BrightnessFunction::Rec601),
            ("lightness", BrightnessFunction::Lightness),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--brightness", value])?;
            assert_eq!(args.brightness, expected);
        }

        let short = parse(&["cli", "-b", "rec601"])?;
        assert_eq!(short.brightness, BrightnessFunction::Rec601);

        Ok(())
    }

    #[test]
    fn parses_all_coloring_functions() -> Result<(), clap::Error> {
        let cases = [
            ("black-and-white", ColoringFunction::BlackAndWhite),
            ("image-based", ColoringFunction::ImageBased),
        ];

        for (value, expected) in cases {
            let args = parse(&["cli", "--coloring", value])?;
            assert_eq!(args.coloring, expected);
        }

        let short = parse(&["cli", "-c", "black-and-white"])?;
        assert_eq!(short.coloring, ColoringFunction::BlackAndWhite);

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
            "-b",
            "rec709",
            "-c",
            "black-and-white",
        ])?;

        assert_eq!(args.filepath.as_deref(), Some("image.jpg"));
        assert!(args.debug);
        assert!(args.text_file);
        assert_eq!(args.padding_x, 12);
        assert_eq!(args.padding_y, 14);
        assert_eq!(args.max_size, 120);
        assert_eq!(args.font_size, 16);
        assert_eq!(args.font_family, ValidFonts::FiraCodeRegular);
        assert_eq!(args.brightness, BrightnessFunction::Rec709);
        assert_eq!(args.coloring, ColoringFunction::BlackAndWhite);

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
