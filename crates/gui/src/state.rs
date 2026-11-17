use std::path::{Path, PathBuf};

use base::{BrightnessFn, ColoringFn, FontsMono, ParsingGradient};
use gpui::*;

use crate::{colorscheme::ColorScheme, components::Button, views};

pub struct ProcessingConfig {
    pub image_path: Option<PathBuf>,
    pub output_png: Option<String>,
    pub output_txt: Option<String>,
    pub max_size: f32,
    pub font_size: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub font: FontsMono,
    pub gradient: ParsingGradient,
    pub coloring: ColoringFn,
    pub brightness: BrightnessFn,
    pub status: Option<(String, bool)>,
}

impl ProcessingConfig {
    fn new(image_path: PathBuf) -> Self {
        let stem = image_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("output");
        let dir = image_path.parent().unwrap_or(Path::new(""));

        ProcessingConfig {
            output_png: Some(
                dir.join(format!("{stem}_ascii.png"))
                    .to_string_lossy()
                    .into_owned(),
            ),
            output_txt: Some(
                dir.join(format!("{stem}.txt"))
                    .to_string_lossy()
                    .into_owned(),
            ),
            image_path: Some(image_path),
            max_size: 100.,
            font_size: 10.,
            padding_x: 3.,
            padding_y: 3.,
            font: FontsMono::FiraCodeBold,
            gradient: ParsingGradient::Standard,
            coloring: ColoringFn::ImageBased,
            brightness: BrightnessFn::Lightness,
            status: None,
        }
    }
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        ProcessingConfig {
            image_path: None,
            output_png: None,
            output_txt: None,
            max_size: 100.,
            font_size: 10.,
            padding_x: 3.,
            padding_y: 3.,
            font: FontsMono::FiraCodeBold,
            gradient: ParsingGradient::Standard,
            coloring: ColoringFn::ImageBased,
            brightness: BrightnessFn::Lightness,
            status: None,
        }
    }
}

pub struct State {
    pub config: ProcessingConfig,
    pub error: Option<String>,
    pub png_focus: FocusHandle,
    pub txt_focus: FocusHandle,
}

impl State {
    pub fn new(cx: &mut Context<Self>) -> Self {
        State {
            config: ProcessingConfig::default(),
            error: None,
            png_focus: cx.focus_handle(),
            txt_focus: cx.focus_handle(),
        }
    }

    // ------------- HANDLERS ------------------
    pub fn handle_select_file(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Select file".into()),
        });

        cx.spawn_in(window, async move |this, cx| {
            let Ok(Ok(Some(paths))) = receiver.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };

            let is_image = infer::get_from_path(&path)
                .ok()
                .flatten()
                .is_some_and(|kind| kind.matcher_type() == infer::MatcherType::Image);

            this.update_in(cx, |state, _window, cx| {
                if is_image {
                    state.error = None;
                    log::info!("Selected file: {}", path.display());
                    //TODO: enable all the widgets once the images is selected
                    state.config = ProcessingConfig::new(path);
                } else {
                    state.error =
                        Some(format!("{} is not a supported image file.", path.display()));
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub fn handle_start(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.status = Some(("Processing...".into(), false));

        let image_path = self.config.image_path.clone();
        let output_png = self.config.output_png.clone();
        let text_path = self.config.output_txt.clone().unwrap();
        let text_path = text_path.trim();
        let text_path = (!text_path.is_empty()).then(|| PathBuf::from(text_path));
        let max_size = self.config.max_size.round() as u32;
        let font = self.config.font;
        let font_size = self.config.font_size;
        let padding_x = self.config.padding_x.round() as u32;
        let padding_y = self.config.padding_y.round() as u32;
        let gradient = self.config.gradient;
        let coloring = self.config.coloring;
        let brightness = self.config.brightness;

        let task = cx.background_spawn(async move {
            base::process_image(
                &image_path.unwrap(),
                Path::new(&output_png.unwrap()),
                text_path,
                max_size,
                font,
                font_size,
                padding_x,
                padding_y,
                gradient,
                coloring,
                brightness,
            )
            .map_err(|err| err.to_string())
        });

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |state, cx| {
                state.config.status = Some(match result {
                    Ok(()) => ("Done. Output written.".into(), false),
                    Err(err) => (err, true),
                });
                cx.notify();
            })
            .ok();
        })
        .detach();

        cx.notify();
    }
}

impl Render for State {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .font_family("Inter")
            .bg(rgb(ColorScheme::BgApp.into()))
            .text_color(rgb(ColorScheme::TextPrimary.into()))
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .id("topbar_menu")
                    .bg(rgb(ColorScheme::BgSurfaceHover.into()))
                    .w_full()
                    .flex()
                    .p_2()
                    .child(
                        Button::new("select-file", "Select file".into())
                            .on_click(cx.listener(State::handle_select_file)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .on_mouse_down(MouseButton::Left, |ev, window, _cx| {
                                // Only start dragging when the click is on empty bar area,
                                // not on a button (see note below).
                                window.start_window_move();
                                let _ = ev;
                            }),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .p_4()
                    .child(views::processing::render(self, cx)),
            )
    }
}
