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

#[derive(Clone, Copy)]
pub struct ImageView {
    pub zoom: f32,
    pub pan: Point<Pixels>,
    /// Mouse position and pan at the start of a drag.
    drag: Option<(Point<Pixels>, Point<Pixels>)>,
}

impl Default for ImageView {
    fn default() -> Self {
        ImageView {
            zoom: 1.,
            pan: point(px(0.), px(0.)),
            drag: None,
        }
    }
}

impl ImageView {
    /// Clamps the pan so the zoomed image content stays inside the viewport.
    /// `fitted` is the image's displayed size at zoom 1.
    pub fn clamped_pan(&self, viewport: Size<Pixels>, fitted: Size<Pixels>) -> Point<Pixels> {
        let max_x =
            ((f32::from(fitted.width) * self.zoom - f32::from(viewport.width)) / 2.).max(0.);
        let max_y =
            ((f32::from(fitted.height) * self.zoom - f32::from(viewport.height)) / 2.).max(0.);
        point(
            px(f32::from(self.pan.x).clamp(-max_x, max_x)),
            px(f32::from(self.pan.y).clamp(-max_y, max_y)),
        )
    }
}

const MAX_ZOOM: f32 = 8.;

pub struct State {
    pub config: ProcessingConfig,
    pub error: Option<String>,
    pub live_render: bool,
    pub preview_png: Option<PathBuf>,
    pub original_view: ImageView,
    pub rendered_view: ImageView,
    preview_generation: u64,
}

impl State {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        State {
            config: ProcessingConfig::default(),
            error: None,
            live_render: false,
            preview_png: None,
            original_view: ImageView::default(),
            rendered_view: ImageView::default(),
            preview_generation: 0,
        }
    }

    fn image_view_mut(&mut self, original: bool) -> &mut ImageView {
        if original {
            &mut self.original_view
        } else {
            &mut self.rendered_view
        }
    }

    pub fn zoom_image(
        &mut self,
        original: bool,
        factor: f32,
        viewport: Size<Pixels>,
        fitted: Size<Pixels>,
    ) {
        let view = self.image_view_mut(original);
        view.zoom = (view.zoom * factor).clamp(1., MAX_ZOOM);
        if view.zoom == 1. {
            view.pan = point(px(0.), px(0.));
        }
        view.pan = view.clamped_pan(viewport, fitted);
    }

    pub fn pan_image(
        &mut self,
        original: bool,
        delta: Point<Pixels>,
        viewport: Size<Pixels>,
        fitted: Size<Pixels>,
    ) {
        let view = self.image_view_mut(original);
        view.pan = point(view.pan.x + delta.x, view.pan.y + delta.y);
        view.pan = view.clamped_pan(viewport, fitted);
    }

    pub fn begin_drag(&mut self, original: bool, mouse: Point<Pixels>) {
        let view = self.image_view_mut(original);
        if view.zoom > 1. {
            view.drag = Some((mouse, view.pan));
        }
    }

    pub fn drag_image(
        &mut self,
        original: bool,
        mouse: Point<Pixels>,
        viewport: Size<Pixels>,
        fitted: Size<Pixels>,
    ) {
        let view = self.image_view_mut(original);
        let Some((start_mouse, start_pan)) = view.drag else {
            return;
        };
        view.pan = point(
            start_pan.x + mouse.x - start_mouse.x,
            start_pan.y + mouse.y - start_mouse.y,
        );
        view.pan = view.clamped_pan(viewport, fitted);
    }

    pub fn end_drag(&mut self, original: bool) {
        self.image_view_mut(original).drag = None;
    }

    pub fn reset_image(&mut self, original: bool) {
        *self.image_view_mut(original) = ImageView::default();
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
                    state.reset_image(true);
                    state.reset_image(false);
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
            let probe_path = PathBuf::from(&path);
            let writable = cx
                .background_executor()
                .spawn(async move { output_is_writable(&probe_path) })
                .await;

            this.update_in(cx, |state, _window, cx| {
                match writable {
                    Ok(()) => {
                        if png {
                            state.config.output_png = Some(path);
                        } else {
                            state.config.output_txt = Some(path);
                        }
                        state.config.status = None;
                    }
                    Err(err) => {
                        state.config.status = Some((
                            format!(
                                "Cannot write {}: {err}",
                                views::processing::short_path(&path)
                            ),
                            true,
                        ));
                    }
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(views::processing::render(self, window, cx)),
            )
    }
}

fn output_is_writable(path: &Path) -> Result<(), std::io::Error> {
    if path.exists() {
        return std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map(|_| ());
    }

    let dir = path.parent().unwrap_or(Path::new("."));
    let probe = dir.join(format!(".ascii-parser-write-test-{}", std::process::id()));
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&probe)
    {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            Ok(())
        }
        // A leftover probe means we can create files here.
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::{ImageView, ProcessingConfig, State, output_is_writable};
    use gpui::{point, px, size};

    #[test]
    fn image_view_clamps_pan_to_content_not_viewport() {
        let viewport = size(px(100.), px(80.));
        let view = ImageView {
            zoom: 2.,
            pan: point(px(999.), px(-999.)),
            ..ImageView::default()
        };
        // Content sized like the viewport at zoom 1 overflows both axes at 2x.
        assert_eq!(
            view.clamped_pan(viewport, viewport),
            point(px(50.), px(-40.))
        );

        // A landscape image is letterboxed vertically, so it cannot pan vertically.
        let fitted = size(px(100.), px(40.));
        assert_eq!(view.clamped_pan(viewport, fitted), point(px(50.), px(0.)));

        let unzoomed = ImageView::default();
        assert_eq!(
            unzoomed.clamped_pan(viewport, viewport),
            point(px(0.), px(0.))
        );
    }

    #[test]
    fn drag_pans_by_mouse_delta_and_clamps() {
        let viewport = size(px(100.), px(80.));
        let mut state = State {
            config: ProcessingConfig::default(),
            error: None,
            live_render: false,
            preview_png: None,
            original_view: ImageView {
                zoom: 2.,
                ..ImageView::default()
            },
            rendered_view: ImageView::default(),
            preview_generation: 0,
        };

        state.begin_drag(true, point(px(10.), px(10.)));
        state.drag_image(true, point(px(30.), px(20.)), viewport, viewport);
        assert_eq!(state.original_view.pan, point(px(20.), px(10.)));

        state.drag_image(true, point(px(999.), px(-999.)), viewport, viewport);
        assert_eq!(state.original_view.pan, point(px(50.), px(-40.)));

        state.end_drag(true);
        state.drag_image(true, point(px(0.), px(0.)), viewport, viewport);
        assert_eq!(state.original_view.pan, point(px(50.), px(-40.)));
    }

    #[test]
    fn probe_reports_writable_and_missing_parent() {
        let target = std::env::temp_dir().join(format!(
            "ascii-parser-probe-test-{}.png",
            std::process::id()
        ));
        assert!(output_is_writable(&target).is_ok());
        let missing =
            std::env::temp_dir().join(format!("ascii-parser-missing-dir-{}", std::process::id()));
        assert!(output_is_writable(&missing.join("output.png")).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn probe_rejects_read_only_folder() {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let dir = std::env::temp_dir().join(format!("ascii-parser-ro-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o555)).unwrap();

        // Root ignores permission bits, so only assert when the mode is enforced.
        let can_bypass = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join(".root-check"))
            .is_ok();
        if !can_bypass {
            assert!(output_is_writable(&dir.join("out.png")).is_err());
        }

        fs::set_permissions(&dir, fs::Permissions::from_mode(0o755)).unwrap();
        let _ = fs::remove_dir_all(&dir);
    }
}
