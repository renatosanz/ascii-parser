fn main() {}

#[cfg(test)]
mod test {
    use ab_glyph::{Font, PxScale};
    use core::{
        fonts::FontsMono,
        image::write_to_image,
        parser::{self, JP2A_GRADIENT, brightness_lightness, brightness_rec709},
        reductor,
        render::{self, black_and_white, image_coloring},
    };
    use image::GenericImageView;
    use std::{fs, path::Path};
    struct TestGuard {
        cleanup: bool,
        pub test_img_path: &'static str,
    }

    impl Drop for TestGuard {
        fn drop(&mut self) {
            // clean all the test generated files
            if self.cleanup {
                println!("Cleaning up after the test.");
                let path = Path::new(self.test_img_path);
                fs::remove_dir_all(path).unwrap();
            }
        }
    }

    fn setup(cleanup: bool) -> TestGuard {
        // create necessary stuff for tests
        println!("Setup: Preparing for the test.");

        let test_dir = "/tmp/ascii-parser-test";
        let path = Path::new(test_dir);
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }

        TestGuard {
            cleanup,
            test_img_path: test_dir,
        }
    }

    #[test]
    pub fn image_reduction_test_square_ratio() {
        let _guard = setup(true);

        let img = image::open("./test/bic.jpg").unwrap(); // open image

        let img_reducted = reductor::adaptative_reduction(&img, 200, 1 as f32); //reduce it

        // save it in the /tmp/ascii-parser dir
        img_reducted
            .save(format!("{}/reduction_square.jpg", _guard.test_img_path))
            .unwrap();

        // validate the reducted image
        let (w, h) = img_reducted.dimensions();
        println!("reducted dimesions: {w} x {h}");
        // both sides should be the same but max 200px
        assert!((w <= 200) && (h <= 200));
    }

    #[test]
    pub fn image_reduction_test_rectangle_ratio() {
        let _guard = setup(true);

        let img = image::open("./test/bg1.jpg").unwrap(); // open image

        let img_reducted = reductor::adaptative_reduction(&img, 200, 1 as f32); //reduce it

        // save it in the /tmp/ascii-parser dir
        img_reducted
            .save(format!("{}/reduction_rectagle.jpg", _guard.test_img_path))
            .unwrap();

        // validate the reducted image
        let (w, h) = img_reducted.dimensions();
        println!("reducted dimesions: {w} x {h}");
        // at least one side should be max 200px
        assert!((w <= 200) || (h <= 200));
    }

    #[test]
    pub fn export_reducted_rectangle_image_test() {
        let _guard = setup(true);

        let img = image::open("./test/bg1.jpg").unwrap(); // open image

        let img_reducted = reductor::adaptative_reduction(&img, 200, 1 as f32); //reduce it
        // save it in the /tmp/ascii-parser dir
        img_reducted
            .save(format!("{}/reduction_rectagle.jpg", _guard.test_img_path))
            .unwrap();
        parser::export_to_text_file(
            &img_reducted,
            Some(format!("{}/reduction_rectagle.txt", _guard.test_img_path).to_string()),
            brightness_rec709,
            JP2A_GRADIENT,
        );
    }
    #[test]
    pub fn export_reducted_square_image_test() {
        let _guard = setup(true);

        let img = image::open("./test/bic.jpg").unwrap(); // open image

        let img_reducted = reductor::adaptative_reduction(&img, 200, 1 as f32); //reduce it
        // save it in the /tmp/ascii-parser dir
        img_reducted
            .save(format!("{}/square_rectagle.jpg", _guard.test_img_path))
            .unwrap();
        parser::export_to_text_file(
            &img_reducted,
            Some(format!("{}/square_rectagle.txt", _guard.test_img_path).to_string()),
            brightness_rec709,
            JP2A_GRADIENT,
        );
    }

    #[test]
    pub fn render_reducted_ascii_image_test() {
        let _guard = setup(true);

        // load font file (ensure you have a .ttf file in your project directory)
        // include_bytes! embeds the font file directly into your compiled binary
        let font = FontsMono::FiraCodeBold.font_ref().unwrap();
        let font_size = 10 as f32;
        let padding_x = font_size as u32;
        let padding_y = font_size as u32;

        // scale the font to get the relation between the height and width on each char of the font
        let scaled = font.as_scaled(PxScale::from(font_size));
        let (advance, line_height, ratio) = render::advance_height_ratio(scaled, 'H');

        let img = image::open("./test/angel.jpg").unwrap(); // open image
        let img_reducted = reductor::adaptative_reduction(&img, 400, ratio); //reduce it

        // save it in the /tmp/ascii-parser dir
        img_reducted
            .save(format!("{}/square_rectagle.jpg", _guard.test_img_path))
            .unwrap();

        // export to txt file
        let text_rows =
            parser::export_to_text_file(&img_reducted, None, brightness_lightness, JP2A_GRADIENT);

        write_to_image(
            &img_reducted,
            format!("{}/text_output_black_and_white.png", _guard.test_img_path),
            &text_rows,
            &font,
            padding_x,
            padding_y,
            advance as u32,
            line_height as u32,
            scaled,
            font_size,
            black_and_white,
        );

        write_to_image(
            &img_reducted,
            format!("{}/text_output_colored.png", _guard.test_img_path),
            &text_rows,
            &font,
            padding_x,
            padding_y,
            advance as u32,
            line_height as u32,
            scaled,
            font_size,
            image_coloring,
        );
    }
}
