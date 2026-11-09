fn main() {}

#[cfg(test)]
mod test {
    use ab_glyph::{Font, PxScale};
    use base::{
        fonts::FontsMono,
        image::write_to_image,
        parser::{self, JP2A_GRADIENT, brightness_lightness, brightness_rec601, brightness_rec709},
        reductor,
        render::{self, black_and_white, image_coloring},
    };
    use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    struct TestGuard {
        cleanup: bool,
        test_dir: PathBuf,
    }

    impl TestGuard {
        fn path(&self, filename: &str) -> String {
            self.test_dir.join(filename).to_string_lossy().into_owned()
        }
    }

    impl Drop for TestGuard {
        fn drop(&mut self) {
            // clean all the test generated files
            if self.cleanup {
                println!("Cleaning up after the test.");
                if let Err(err) = fs::remove_dir_all(&self.test_dir) {
                    eprintln!("Failed to clean up test directory: {err}");
                }
            }
        }
    }

    fn setup(test_name: &str, cleanup: bool) -> Result<TestGuard, std::io::Error> {
        // create necessary stuff for tests, using a directory per test so
        // tests can run in parallel without deleting each other's files.
        println!("Setup: Preparing for the test.");

        let test_dir = Path::new("/tmp/ascii-parser-test").join(test_name);
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir)?;
        }
        fs::create_dir_all(&test_dir)?;

        Ok(TestGuard { cleanup, test_dir })
    }

    fn assert_approx_eq(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-6,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    pub fn image_reduction_test_square_ratio() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = setup("image_reduction_square_ratio", true)?;

        let img = image::open("./test/bic.jpg")?; // open image
        let (original_w, original_h) = img.dimensions();
        assert!(original_w > 0, "source image should have a width");
        assert!(original_h > 0, "source image should have a height");

        let max_dim = 200;
        let img_reducted = reductor::adaptative_reduction(&img, max_dim, 1 as f32); //reduce it

        // save it in the /tmp/ascii-parser dir
        let image_path = _guard.path("reduction_square.jpg");
        img_reducted.save(&image_path)?;

        // validate the reducted image
        let factor = max_dim as f32 / original_w.max(original_h) as f32;
        let expected_w = ((original_w as f32) * factor).round().max(1.0) as u32;
        let expected_h = ((original_h as f32) * factor).round().max(1.0) as u32;

        assert_eq!(
            img_reducted.dimensions(),
            (expected_w, expected_h),
            "reduction should scale the image by the expected factor"
        );
        assert_eq!(
            img_reducted.width().max(img_reducted.height()),
            max_dim,
            "the biggest side should be exactly max_dim"
        );
        assert!(
            img_reducted.width() <= max_dim && img_reducted.height() <= max_dim,
            "neither side should exceed max_dim"
        );

        // the saved file should exist and contain the reduced image
        assert!(Path::new(&image_path).exists());
        let saved = image::open(&image_path)?;
        assert_eq!(saved.dimensions(), img_reducted.dimensions());

        Ok(())
    }

    #[test]
    pub fn image_reduction_test_rectangle_ratio() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = setup("image_reduction_test_rectangle_ratio", true)?;

        let img = image::open("./test/bg1.jpg")?; // open image
        let (original_w, original_h) = img.dimensions();
        assert!(original_w > 0, "source image should have a width");
        assert!(original_h > 0, "source image should have a height");

        let max_dim = 200;
        let font_ratio = 0.5;
        let img_reducted = reductor::adaptative_reduction(&img, max_dim, font_ratio); //reduce it

        // save it in the /tmp/ascii-parser dir
        let image_path = _guard.path("reduction_rectagle.jpg");
        img_reducted.save(&image_path)?;

        // validate the reducted image
        let factor = max_dim as f32 / original_w.max(original_h) as f32;
        let expected_w = ((original_w as f32) * factor).round().max(1.0) as u32;
        let expected_h = ((original_h as f32) * factor * font_ratio).round().max(1.0) as u32;

        assert_eq!(
            img_reducted.dimensions(),
            (expected_w, expected_h),
            "font ratio should be applied to the height only"
        );
        assert!(
            img_reducted.width() <= max_dim && img_reducted.height() <= max_dim,
            "neither side should exceed max_dim"
        );

        assert!(Path::new(&image_path).exists());
        let saved = image::open(&image_path)?;
        assert_eq!(saved.dimensions(), img_reducted.dimensions());

        Ok(())
    }

    #[test]
    pub fn export_reducted_rectangle_image_test() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = setup("export_reducted_rectangle_image_test", true)?;

        let img = image::open("./test/bg1.jpg")?; // open image

        let img_reducted = reductor::adaptative_reduction(&img, 200, 1 as f32); //reduce it
        // save it in the /tmp/ascii-parser dir
        let image_path = _guard.path("reduction_rectagle.jpg");
        img_reducted.save(&image_path)?;

        let text_path = _guard.path("reduction_rectagle.txt");
        let text_rows = parser::export_to_text_file(
            &img_reducted,
            Some(text_path.clone()),
            brightness_rec709,
            JP2A_GRADIENT,
        )?;

        let (width, height) = img_reducted.dimensions();
        assert_eq!(
            text_rows.len(),
            height as usize,
            "there should be one text row per image row"
        );
        assert!(
            text_rows
                .iter()
                .all(|row| row.chars().count() == width as usize),
            "every text row should have one character per image column"
        );
        assert!(
            text_rows
                .iter()
                .flat_map(|row| row.chars())
                .all(|c| JP2A_GRADIENT.contains(c)),
            "every generated character should belong to the gradient"
        );

        assert!(Path::new(&image_path).exists());
        assert!(Path::new(&text_path).exists());
        let saved_text = fs::read_to_string(&text_path)?;
        assert_eq!(
            saved_text,
            text_rows.join("\n"),
            "the exported file should match the returned rows"
        );
        assert!(!saved_text.is_empty());

        Ok(())
    }

    #[test]
    pub fn export_reducted_square_image_test() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = setup("export_reducted_square_image_test", true)?;

        let img = image::open("./test/bic.jpg")?; // open image

        let img_reducted = reductor::adaptative_reduction(&img, 200, 1 as f32); //reduce it
        // save it in the /tmp/ascii-parser dir
        let image_path = _guard.path("square_rectagle.jpg");
        img_reducted.save(&image_path)?;

        let text_path = _guard.path("square_rectagle.txt");
        let text_rows = parser::export_to_text_file(
            &img_reducted,
            Some(text_path.clone()),
            brightness_rec709,
            JP2A_GRADIENT,
        )?;

        let (width, height) = img_reducted.dimensions();
        assert_eq!(
            text_rows.len(),
            height as usize,
            "there should be one text row per image row"
        );
        assert!(
            text_rows
                .iter()
                .all(|row| row.chars().count() == width as usize),
            "every text row should have one character per image column"
        );

        assert!(Path::new(&image_path).exists());
        assert!(Path::new(&text_path).exists());
        let saved_text = fs::read_to_string(&text_path)?;
        assert_eq!(saved_text, text_rows.join("\n"));

        Ok(())
    }

    #[test]
    pub fn render_reducted_ascii_image_test() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = setup("render_reducted_ascii_image_test", true)?;

        // load font file (ensure you have a .ttf file in your project directory)
        // include_bytes! embeds the font file directly into your compiled binary
        let font = FontsMono::FiraCodeBold.font_ref()?;
        let font_size = 10 as f32;
        let padding_x = font_size as u32;
        let padding_y = font_size as u32;

        // scale the font to get the relation between the height and width on each char of the font
        let scaled = font.as_scaled(PxScale::from(font_size));
        let (advance, line_height, ratio) = render::advance_height_ratio(scaled, 'H');

        assert!(advance > 0.0, "character advance should be positive");
        assert!(line_height > 0.0, "line height should be positive");
        assert!(ratio > 0.0, "font ratio should be positive");
        assert_approx_eq(ratio, advance / line_height);

        let img = image::open("./test/angel.jpg")?; // open image
        let img_reducted = reductor::adaptative_reduction(&img, 400, ratio); //reduce it

        // save it in the /tmp/ascii-parser dir
        let image_path = _guard.path("square_rectagle.jpg");
        img_reducted.save(&image_path)?;

        // export to txt file
        let text_rows =
            parser::export_to_text_file(&img_reducted, None, brightness_lightness, JP2A_GRADIENT)?;

        assert!(!text_rows.is_empty(), "there should be text rows to render");
        assert_eq!(text_rows.len(), img_reducted.height() as usize);
        assert!(
            text_rows
                .iter()
                .all(|row| row.chars().count() == img_reducted.width() as usize),
            "every text row should match the reduced image width"
        );

        let char_w = advance as u32;
        let char_h = line_height as u32;
        let expected_width = text_rows[0].len() as u32 * char_w + padding_x * 2;
        let expected_height = text_rows.len() as u32 * char_h + padding_y * 2;

        let black_and_white_path = _guard.path("text_output_black_and_white.png");
        write_to_image(
            &img_reducted,
            black_and_white_path.clone(),
            &text_rows,
            &font,
            padding_x,
            padding_y,
            char_w,
            char_h,
            scaled,
            font_size,
            black_and_white,
        )?;

        let colored_path = _guard.path("text_output_colored.png");
        write_to_image(
            &img_reducted,
            colored_path.clone(),
            &text_rows,
            &font,
            padding_x,
            padding_y,
            char_w,
            char_h,
            scaled,
            font_size,
            image_coloring,
        )?;

        for path in [&black_and_white_path, &colored_path] {
            assert!(Path::new(path).exists(), "{path} should have been created");

            let rendered = image::open(path)?;
            assert_eq!(
                rendered.dimensions(),
                (expected_width, expected_height),
                "rendered image should include padding and every character cell"
            );

            let non_black_pixels = rendered
                .to_rgba8()
                .pixels()
                .filter(|p| p[0] != 0 || p[1] != 0 || p[2] != 0)
                .count();
            assert!(
                non_black_pixels > 0,
                "{path} should contain rendered glyph pixels"
            );
        }

        // the black and white renderer should only produce grayscale pixels
        let bw = image::open(&black_and_white_path)?.to_rgba8();
        assert!(
            bw.pixels().all(|p| p[0] == p[1] && p[1] == p[2]),
            "black and white output should be grayscale"
        );
        assert!(
            bw.pixels().any(|p| p[0] > 0),
            "black and white output should contain visible text"
        );

        Ok(())
    }

    #[test]
    fn brightness_functions_return_expected_values() {
        let black = Rgba([0, 0, 0, 255]);
        let white = Rgba([255, 255, 255, 255]);
        let red = Rgba([255, 0, 0, 255]);
        let green = Rgba([0, 255, 0, 255]);
        let blue = Rgba([0, 0, 255, 255]);

        for pixel in [black, white, red, green, blue] {
            for brightness in [
                brightness_rec709(pixel),
                brightness_rec601(pixel),
                brightness_lightness(pixel),
            ] {
                assert!(
                    (0.0..=1.0).contains(&brightness),
                    "brightness should be normalized, got {brightness}"
                );
            }
        }

        assert_approx_eq(brightness_rec709(black), 0.0);
        assert_approx_eq(brightness_rec601(black), 0.0);
        assert_approx_eq(brightness_lightness(black), 0.0);

        assert_approx_eq(brightness_rec709(white), 1.0);
        assert_approx_eq(brightness_rec601(white), 1.0);
        assert_approx_eq(brightness_lightness(white), 1.0);

        assert_approx_eq(brightness_rec709(red), 0.2126);
        assert_approx_eq(brightness_rec601(red), 0.299);
        assert_approx_eq(brightness_lightness(red), 0.5);

        assert_approx_eq(brightness_rec709(green), 0.7152);
        assert_approx_eq(brightness_rec601(green), 0.587);
        assert_approx_eq(brightness_lightness(green), 0.5);

        assert_approx_eq(brightness_rec709(blue), 0.0722);
        assert_approx_eq(brightness_rec601(blue), 0.114);
        assert_approx_eq(brightness_lightness(blue), 0.5);
    }

    #[test]
    fn export_to_text_file_maps_brightness_to_gradient() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = setup("export_to_text_file_maps_brightness_to_gradient", true)?;

        // a 2x2 checkerboard of black and white pixels
        let img = DynamicImage::ImageRgba8(RgbaImage::from_fn(2, 2, |x, y| {
            if (x + y) % 2 == 0 {
                Rgba([0, 0, 0, 255])
            } else {
                Rgba([255, 255, 255, 255])
            }
        }));

        let rows = parser::export_to_text_file(&img, None, brightness_lightness, "AB")?;

        // dark pixels map to the end of the gradient, light pixels to the start
        assert_eq!(rows, vec!["BA".to_string(), "AB".to_string()]);
        assert_eq!(rows.len(), img.height() as usize);
        assert!(rows.iter().all(|row| row.len() == img.width() as usize));

        let text_path = _guard.path("gradient.txt");
        let rows_with_file =
            parser::export_to_text_file(&img, Some(text_path.clone()), brightness_lightness, "AB")?;

        assert_eq!(rows_with_file, rows);
        assert!(Path::new(&text_path).exists());
        assert_eq!(fs::read_to_string(&text_path)?, "BA\nAB");

        Ok(())
    }

    #[test]
    fn adaptative_reduction_produces_expected_dimensions() {
        let img = DynamicImage::ImageRgba8(RgbaImage::from_pixel(400, 200, Rgba([0, 0, 0, 255])));

        let reduced = reductor::adaptative_reduction(&img, 100, 0.5);
        assert_eq!(reduced.dimensions(), (100, 25));

        // the height is clamped to at least one pixel
        let reduced = reductor::adaptative_reduction(&img, 100, 0.001);
        assert_eq!(reduced.dimensions(), (100, 1));
    }

    #[test]
    fn coloring_functions_return_expected_pixels() {
        let img = DynamicImage::ImageRgba8(RgbaImage::from_fn(2, 2, |x, y| {
            Rgba([x as u8 * 100, y as u8 * 100, 0, 255])
        }));

        assert_eq!(black_and_white(0, 0, &img), Rgba([255, 255, 255, 255]));
        assert_eq!(black_and_white(1, 1, &img), Rgba([255, 255, 255, 255]));
        assert_eq!(image_coloring(0, 0, &img), Rgba([0, 0, 0, 255]));
        assert_eq!(image_coloring(1, 1, &img), Rgba([100, 100, 0, 255]));
    }

    #[test]
    fn advance_height_ratio_is_consistent() -> Result<(), Box<dyn std::error::Error>> {
        let font = FontsMono::FiraCodeBold.font_ref()?;
        let scaled = font.as_scaled(PxScale::from(20.0));

        let (advance, line_height, ratio) = render::advance_height_ratio(scaled, 'H');

        assert!(advance > 0.0);
        assert!(line_height > 0.0);
        assert_approx_eq(ratio, advance / line_height);
        assert_approx_eq(ratio * line_height, advance);

        Ok(())
    }

    #[test]
    fn all_font_assets_load_successfully() {
        let fonts = [
            FontsMono::CascadiaCodeNfBold,
            FontsMono::CascadiaCodeNfRegular,
            FontsMono::FiraCodeBold,
            FontsMono::FiraCodeRegular,
            FontsMono::HackBold,
            FontsMono::HackRegular,
            FontsMono::JetBrainsMonoNlBold,
            FontsMono::JetBrainsMonoNlRegular,
        ];

        for font in fonts {
            assert!(!font.bytes().is_empty(), "font bytes should not be empty");
            assert!(!font.name().is_empty(), "font name should not be empty");
            assert!(
                font.font_ref().is_ok(),
                "{} should be a valid font",
                font.name()
            );
        }
    }
}
