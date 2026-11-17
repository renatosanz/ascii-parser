use std::borrow::Cow;

use gpui::*;

mod colorscheme;
mod components;
mod state;
mod views;

const START_WINDOW_SIZE: Size<Pixels> = Size {
    width: px(500.),
    height: px(500.),
};

fn main() {
    simple_logger::init_with_level(log::Level::Info).ok();

    gpui_platform::application().run(|cx: &mut App| {
        cx.text_system()
            .add_fonts(vec![
                Cow::Borrowed(include_bytes!("../assets/fonts/Inter-Regular.otf").as_slice()),
                Cow::Borrowed(include_bytes!("../assets/fonts/Inter-Medium.otf").as_slice()),
                Cow::Borrowed(include_bytes!("../assets/fonts/Inter-Bold.otf").as_slice()),
            ])
            .expect("failed to load bundled Inter fonts");

        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("ascii-parser".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                START_WINDOW_SIZE,
                cx,
            ))),
            is_resizable: false,
            app_id: Some("ascii-parser".into()),
            ..WindowOptions::default()
        };

        cx.open_window(window_options, |_window, cx| cx.new(state::State::new))
            .unwrap();
    });
}
