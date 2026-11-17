use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use base::{BrightnessFn, ColoringFn, FontsMono, ParsingGradient};
use gpui::*;

use crate::{
    colorscheme::ColorScheme,
    components::{Button, Switch},
    views,
};

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
    pub live_render: bool,
    pub preview_png: Option<PathBuf>,
    preview_generation: u64,
}

impl State {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        State {
            config: ProcessingConfig::default(),
            error: None,
            live_render: false,
            preview_png: None,
            preview_generation: 0,
        }
    }

    // ------------- HANDLERS ------------------
    pub fn handle_toggle_live_render(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.live_render = !self.live_render;
        if self.live_render {
            self.request_live_preview(cx);
        } else {
            self.preview_generation += 1;
        }
        cx.notify();
    }

    pub fn request_live_preview(&mut self, cx: &mut Context<Self>) {
        if !self.live_render {
            return;
        }
        let Some(image_path) = self.config.image_path.clone() else {
            return;
        };

        self.preview_generation += 1;
        let generation = self.preview_generation;

        let max_size = self.config.max_size.round() as u32;
        let font = self.config.font;
        let font_size = self.config.font_size;
        let padding_x = self.config.padding_x.round() as u32;
        let padding_y = self.config.padding_y.round() as u32;
        let gradient = self.config.gradient;
        let coloring = self.config.coloring;
        let brightness = self.config.brightness;

        // Debounce so dragging a slider only renders once it settles.
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(150))
                .await;

            let Ok(still_current) =
                this.update(cx, |state, _| state.preview_generation == generation)
            else {
                return;
            };
            if !still_current {
                return;
            }

            let preview_path = std::env::temp_dir().join(format!(
                "ascii-parser-live-{}-{generation}.png",
                std::process::id()
            ));
            let output_path = preview_path.clone();
            let result = cx
                .background_executor()
                .spawn(async move {
                    base::process_image(
                        &image_path,
                        &output_path,
                        None,
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
                })
                .await;

            this.update(cx, |state, cx| {
                if state.preview_generation != generation {
                    let _ = std::fs::remove_file(&preview_path);
                    return;
                }

                match result {
                    Ok(()) => {
                        if let Some(previous) = state.preview_png.replace(preview_path.clone()) {
                            if previous != preview_path {
                                let _ = std::fs::remove_file(previous);
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Live preview failed: {err}");
                        let _ = std::fs::remove_file(&preview_path);
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

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
                    if let Some(previous) = state.preview_png.take() {
                        let _ = std::fs::remove_file(previous);
                    }
                    state.preview_generation += 1;
                    state.request_live_preview(cx);
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

    pub fn handle_select_output_png_dir(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_output_dir(true, window, cx);
    }

    pub fn handle_select_output_txt_dir(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_output_dir(false, window, cx);
    }

    fn select_output_dir(&mut self, png: bool, window: &mut Window, cx: &mut Context<Self>) {
        let current = if png {
            &self.config.output_png
        } else {
            &self.config.output_txt
        };
        let file_name = current
            .as_deref()
            .and_then(|path| Path::new(path).file_name())
            .map(|name| name.to_os_string())
            .unwrap_or_else(|| {
                let stem = self
                    .config
                    .image_path
                    .as_ref()
                    .and_then(|path| path.file_stem())
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("output");
                if png {
                    format!("{stem}_ascii.png")
                } else {
                    format!("{stem}.txt")
                }
                .into()
            });

        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Select output folder".into()),
        });

        cx.spawn_in(window, async move |this, cx| {
            let Ok(Ok(Some(paths))) = receiver.await else {
                return;
            };
            let Some(dir) = paths.into_iter().next() else {
                return;
            };
            let path = dir.join(file_name).to_string_lossy().into_owned();

            this.update_in(cx, |state, _window, cx| {
                if png {
                    state.config.output_png = Some(path);
                } else {
                    state.config.output_txt = Some(path);
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

impl Drop for State {
    fn drop(&mut self) {
        if let Some(preview) = self.preview_png.take() {
            let _ = std::fs::remove_file(preview);
        }
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
                    .bg(rgb(ColorScheme::BgSurface.into()))
                    .border_b_1()
                    .border_color(rgb(ColorScheme::BorderSubtle.into()))
                    .w_full()
                    .flex()
                    .items_center()
                    .gap_3()
                    .p_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .h(px(34.))
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .child("ascii-parser"),
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
                    )
                    .child(
                        Switch::new("live-render", "Live", self.live_render)
                            .on_click(cx.listener(State::handle_toggle_live_render)),
                    )
                    .child(
                        Button::new("select-file", "Select file".into())
                            .on_click(cx.listener(State::handle_select_file)),
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

