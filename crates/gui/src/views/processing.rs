use base::{BrightnessFn, ColoringFn, FontsMono, ParsingGradient};
use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::{
    colorscheme::ColorScheme,
    components::{Button, Dropdown, IconButton, Label, Slider},
    state::{ImageView, State},
};

const OPEN_FOLDER: &[u8] = include_bytes!("../../assets/svg/open_folder.svg");
const ZOOM_OUT: &[u8] = include_bytes!("../../assets/svg/zoom_out.svg");

pub fn render(state: &State, window: &mut Window, cx: &mut Context<State>) -> impl IntoElement {
    let ui_disabled = state.config.image_path.is_none();

    let entity = cx.entity();

    let font_options: Vec<SharedString> = FontsMono::ALL
        .iter()
        .map(|font| font.name().into())
        .collect();
    let gradient_options: Vec<SharedString> = ParsingGradient::ALL
        .iter()
        .map(|value| format!("{value:?}").into())
        .collect();
    let coloring_options: Vec<SharedString> = ColoringFn::ALL
        .iter()
        .map(|value| format!("{value:?}").into())
        .collect();
    let brightness_options: Vec<SharedString> = BrightnessFn::ALL
        .iter()
        .map(|value| format!("{value:?}").into())
        .collect();

    let max_size = state.config.max_size;
    let font_size = state.config.font_size;
    let padding_x = state.config.padding_x;
    let padding_y = state.config.padding_y;
    let font = state.config.font;
    let gradient = state.config.gradient;
    let coloring = state.config.coloring;
    let brightness = state.config.brightness;
    let output_png = state.config.output_png.clone().unwrap_or_default();
    let output_txt = state.config.output_txt.clone().unwrap_or_default();
    let status = state.config.status.clone();

    div()
        .flex()
        .flex_row_reverse()
        .size_full()
        .min_h_0()
        .gap_4()
        .when(state.live_render, |this| {
            this.child(
                div()
                    .id("image_views_section")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .gap_3()
                    .overflow_y_scroll()
                    .child(image_box(
                        "original",
                        "Original",
                        state.config.image_path.clone(),
                        state.original_view,
                        true,
                        entity.clone(),
                        window,
                        cx,
                    ))
                    .child(image_box(
                        "rendered",
                        "Rendered",
                        state.preview_png.clone(),
                        state.rendered_view,
                        false,
                        entity.clone(),
                        window,
                        cx,
                    )),
            )
        })
        .child(
            div()
                .id("config_section")
                .flex()
                .flex_col()
                .flex_shrink_0()
                .when(state.live_render, |this| this.w(px(240.)))
                .when(!state.live_render, |this| this.flex_1())
                .gap_3()
                .h_full()
                .min_h_0()
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::MEDIUM)
                        .child("Settings"),
                )
                .child(
                    div()
                        .id("controls")
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_h_0()
                        .overflow_y_scroll()
                        .gap(px(12.))
                        .child(
                            div()
                                .p_3()
                                .border_1()
                                .flex()
                                .flex_col()
                                .flex_shrink_0()
                                .border_color(rgb(ColorScheme::BorderSubtle.into()))
                                .bg(rgb(ColorScheme::BgSurface.into()))
                                .rounded_lg()
                                .gap_3()
                                .child(field(
                                    format!("Max side size: {max_size:.0}"),
                                    Slider::new("max-size", max_size, 50., 400.)
                                        .disabled(ui_disabled)
                                        .on_change({
                                            let entity = entity.clone();
                                            move |value, _, cx| {
                                                entity.update(cx, |state, cx| {
                                                    state.config.max_size = value;
                                                    state.request_live_preview(cx);
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                ))
                                .child(field(
                                    format!("Font size: {font_size:.0}"),
                                    Slider::new("font-size", font_size, 5., 30.)
                                        .disabled(ui_disabled)
                                        .on_change({
                                            let entity = entity.clone();
                                            move |value, _, cx| {
                                                entity.update(cx, |state, cx| {
                                                    state.config.font_size = value;
                                                    state.request_live_preview(cx);
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                ))
                                .child(field(
                                    format!("Padding X: {padding_x:.0}"),
                                    Slider::new("padding-x", padding_x, 0., 30.)
                                        .disabled(ui_disabled)
                                        .on_change({
                                            let entity = entity.clone();
                                            move |value, _, cx| {
                                                entity.update(cx, |state, cx| {
                                                    state.config.padding_x = value;
                                                    state.request_live_preview(cx);
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                ))
                                .child(field(
                                    format!("Padding Y: {padding_y:.0}"),
                                    Slider::new("padding-y", padding_y, 0., 30.)
                                        .disabled(ui_disabled)
                                        .on_change({
                                            let entity = entity.clone();
                                            move |value, _, cx| {
                                                entity.update(cx, |state, cx| {
                                                    state.config.padding_y = value;
                                                    state.request_live_preview(cx);
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                ))
                                .child(field(
                                    "Font",
                                    Dropdown::new("font", font.name(), font_options)
                                        .disabled(ui_disabled)
                                        .on_change({
                                            let entity = entity.clone();
                                            move |picked, _, cx| {
                                                let Some(font) = FontsMono::ALL
                                                    .iter()
                                                    .copied()
                                                    .find(|font| font.name() == picked.as_str())
                                                else {
                                                    return;
                                                };
                                                entity.update(cx, |state, cx| {
                                                    state.config.font = font;
                                                    state.request_live_preview(cx);
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                ))
                                .child(field(
                                    "Gradient",
                                    Dropdown::new(
                                        "gradient",
                                        format!("{gradient:?}"),
                                        gradient_options,
                                    )
                                    .disabled(ui_disabled)
                                    .on_change({
                                        let entity = entity.clone();
                                        move |picked, _, cx| {
                                            let Some(gradient) =
                                                ParsingGradient::ALL.iter().copied().find(
                                                    |value| format!("{value:?}") == picked.as_str(),
                                                )
                                            else {
                                                return;
                                            };
                                            entity.update(cx, |state, cx| {
                                                state.config.gradient = gradient;
                                                state.request_live_preview(cx);
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ))
                                .child(field(
                                    "Coloring",
                                    Dropdown::new(
                                        "coloring",
                                        format!("{coloring:?}"),
                                        coloring_options,
                                    )
                                    .disabled(ui_disabled)
                                    .on_change({
                                        let entity = entity.clone();
                                        move |picked, _, cx| {
                                            let Some(coloring) =
                                                ColoringFn::ALL.iter().copied().find(|value| {
                                                    format!("{value:?}") == picked.as_str()
                                                })
                                            else {
                                                return;
                                            };
                                            entity.update(cx, |state, cx| {
                                                state.config.coloring = coloring;
                                                state.request_live_preview(cx);
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ))
                                .child(field(
                                    "Brightness",
                                    Dropdown::new(
                                        "brightness",
                                        format!("{brightness:?}"),
                                        brightness_options,
                                    )
                                    .disabled(ui_disabled)
                                    .on_change({
                                        let entity = entity.clone();
                                        move |picked, _, cx| {
                                            let Some(brightness) =
                                                BrightnessFn::ALL.iter().copied().find(|value| {
                                                    format!("{value:?}") == picked.as_str()
                                                })
                                            else {
                                                return;
                                            };
                                            entity.update(cx, |state, cx| {
                                                state.config.brightness = brightness;
                                                state.request_live_preview(cx);
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ))
                                .child(field(
                                    "Output PNG",
                                    div()
                                        .flex()
                                        .gap_2()
                                        .w_full()
                                        .child(display_path(&output_png, "Select a folder"))
                                        .child(
                                            IconButton::new("output-png-dir", OPEN_FOLDER)
                                                .disabled(ui_disabled)
                                                .on_click(
                                                    cx.listener(
                                                        State::handle_select_output_png_dir,
                                                    ),
                                                ),
                                        ),
                                ))
                                .child(field(
                                    "Output TXT",
                                    div()
                                        .flex()
                                        .gap_2()
                                        .w_full()
                                        .child(display_path(&output_txt, "Disabled"))
                                        .child(
                                            IconButton::new("output-txt-dir", OPEN_FOLDER)
                                                .disabled(ui_disabled)
                                                .on_click(
                                                    cx.listener(
                                                        State::handle_select_output_txt_dir,
                                                    ),
                                                ),
                                        ),
                                )),
                        ),
                )
                .child(
                    div()
                        .p_3()
                        .border_1()
                        .flex()
                        .flex_col()
                        .flex_shrink_0()
                        .gap_2()
                        .border_color(rgb(ColorScheme::BorderSubtle.into()))
                        .bg(rgb(ColorScheme::BgSurface.into()))
                        .rounded_lg()
                        .child(
                            Button::new(
                                "start",
                                if state.live_render { "Save" } else { "Start" }.into(),
                            )
                            .on_click(cx.listener(State::handle_start))
                            .disabled(ui_disabled),
                        )
                        .when_some(status, |this, (message, is_error)| {
                            let color = if is_error {
                                ColorScheme::ColorErrorText
                            } else {
                                ColorScheme::ColorSuccessText
                            };
                            this.child(div().text_sm().text_color(rgb(color.into())).child(message))
                        }),
                ),
        )
}

fn display_path(path: &str, empty_label: &'static str) -> Div {
    let empty = path.is_empty();
    div()
        .flex()
        .items_center()
        .h(px(34.))
        .flex_1()
        .min_w_0()
        .text_sm()
        .when(empty, |this| {
            this.text_color(rgb(ColorScheme::TextMuted.into()))
        })
        .when(!empty, |this| {
            this.text_color(rgb(ColorScheme::TextSecondary.into()))
        })
        .child(
            div()
                .w_full()
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis_middle()
                .child(if empty {
                    empty_label.to_string()
                } else {
                    short_path(path)
                }),
        )
}

pub(crate) fn short_path(path: &str) -> String {
    collapse_home(
        path,
        std::env::var_os("HOME")
            .map(std::path::PathBuf::from)
            .as_deref(),
    )
}

fn collapse_home(path: &str, home: Option<&std::path::Path>) -> String {
    let Some(home) = home.and_then(|home| home.to_str()) else {
        return path.to_string();
    };
    let home = home.trim_end_matches('/');
    if path == home {
        "~".to_string()
    } else if let Some(rest) = path.strip_prefix(home).filter(|rest| rest.starts_with('/')) {
        format!("~{rest}")
    } else {
        path.to_string()
    }
}

fn field(label: impl Into<SharedString>, control: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(Label::new(label))
        .child(control)
}

fn image_box(
    id: &'static str,
    caption: &'static str,
    source: Option<std::path::PathBuf>,
    view: ImageView,
    original: bool,
    entity: Entity<State>,
    window: &mut Window,
    cx: &mut Context<State>,
) -> impl IntoElement {
    let viewport_state = window.use_keyed_state(
        ElementId::Name(format!("{id}-viewport").into()),
        cx,
        |_, _| None::<Size<Pixels>>,
    );
    let viewport = *viewport_state.read(cx);
    let record_viewport = {
        let viewport_state = viewport_state.clone();
        move |children: Vec<Bounds<Pixels>>, _: &mut Window, cx: &mut App| {
            let Some(size) = children.first().map(|bounds| bounds.size) else {
                return;
            };
            viewport_state.update(cx, |slot, cx| {
                if *slot != Some(size) {
                    *slot = Some(size);
                    cx.notify();
                }
            });
        }
    };

    let measured = viewport.unwrap_or_default();
    let pan = view.clamped_pan(measured);
    let zoomed = view.zoom > 1.;
    let width = measured.width * view.zoom;
    let height = measured.height * view.zoom;
    let left = measured.width * ((1. - view.zoom) / 2.) + pan.x;
    let top = measured.height * ((1. - view.zoom) / 2.) + pan.y;
    let (vertical_offset, vertical_size) = scroll_thumb(measured.height, view.zoom, pan.y);
    let (horizontal_offset, horizontal_size) = scroll_thumb(measured.width, view.zoom, pan.x);
    let has_source = source.is_some();

    let on_scroll = {
        let entity = entity.clone();
        move |event: &ScrollEvent, _: &mut Window, cx: &mut App| {
            if !has_source {
                return;
            }
            let delta = scroll_delta(&event.delta);
            entity.update(cx, |state, cx| {
                if delta.y != px(0.) {
                    let factor = (1. + f32::from(delta.y) / 250.).clamp(0.5, 2.);
                    state.zoom_image(original, factor, measured);
                }
                if delta.x != px(0.) {
                    state.pan_image(original, point(-delta.x, px(0.)), measured);
                }
                cx.notify();
            });
            cx.stop_propagation();
        }
    };

    let reset = {
        let entity = entity.clone();
        move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            entity.update(cx, |state, cx| {
                state.reset_image(original);
                cx.notify();
            });
        }
    };

    div()
        .id(id)
        .flex_grow(5.)
        .flex_basis(px(0.))
        .min_h(px(120.))
        .flex()
        .flex_col()
        .overflow_hidden()
        .border_1()
        .border_color(rgb(ColorScheme::BorderSubtle.into()))
        .bg(rgb(ColorScheme::BgSurface.into()))
        .rounded_lg()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .px_3()
                .py_2()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ColorScheme::TextSecondary.into()))
                .child(caption)
                .child(
                    IconButton::new(format!("{id}-reset"), ZOOM_OUT)
                        .compact()
                        .on_click(reset),
                ),
        )
        .child(
            div()
                .on_children_prepainted(record_viewport)
                .flex_1()
                .min_h_0()
                .flex()
                .px_3()
                .pb_3()
                .child(
                    div()
                        .id(format!("{id}-viewport"))
                        .relative()
                        .flex_1()
                        .h_full()
                        .flex()
                        .items_center()
                        .content_center()
                        .overflow_hidden()
                        .on_scroll(on_scroll)
                        .when_some(source, |this, path| {
                            this.child(if zoomed {
                                div()
                                    .absolute()
                                    .left(left)
                                    .top(top)
                                    .w(width)
                                    .h(height)
                                    .child(img(path).size_full().object_fit(ObjectFit::Contain))
                                    .into_any_element()
                            } else {
                                img(path)
                                    .size_full()
                                    .object_fit(ObjectFit::Contain)
                                    .into_any_element()
                            })
                        })
                        .when(zoomed, |this| {
                            this.child(
                                div()
                                    .absolute()
                                    .left(px(2.))
                                    .top(px(4.))
                                    .bottom(px(10.))
                                    .w(px(4.))
                                    .rounded_full()
                                    .bg(rgb(ColorScheme::BorderSubtle.into()))
                                    .child(
                                        div()
                                            .absolute()
                                            .top(relative(vertical_offset))
                                            .left(px(0.))
                                            .w_full()
                                            .h(relative(vertical_size))
                                            .rounded_full()
                                            .bg(rgb(ColorScheme::BorderStrong.into())),
                                    ),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .bottom(px(2.))
                                    .left(px(4.))
                                    .right(px(10.))
                                    .h(px(4.))
                                    .rounded_full()
                                    .bg(rgb(ColorScheme::BorderSubtle.into()))
                                    .child(
                                        div()
                                            .absolute()
                                            .left(relative(horizontal_offset))
                                            .top(px(0.))
                                            .h_full()
                                            .w(relative(horizontal_size))
                                            .rounded_full()
                                            .bg(rgb(ColorScheme::BorderStrong.into())),
                                    ),
                            )
                        }),
                ),
        )
}

fn scroll_delta(delta: &ScrollDelta) -> Point<Pixels> {
    match delta {
        ScrollDelta::Pixels(delta) => *delta,
        ScrollDelta::Lines(delta) => point(px(delta.x * 20.), px(delta.y * 20.)),
    }
}

/// Returns the thumb offset and size as fractions of the scrollbar track.
fn scroll_thumb(viewport: Pixels, zoom: f32, pan: Pixels) -> (f32, f32) {
    let span = f32::from(viewport) * (zoom - 1.);
    let fraction = if span > 0. {
        (0.5 - f32::from(pan) / span).clamp(0., 1.)
    } else {
        0.
    };
    (fraction * (1. - 1. / zoom), 1. / zoom)
}

#[cfg(test)]
mod tests {
    use super::{collapse_home, scroll_thumb};
    use gpui::{Pixels, px};
    use std::path::Path;

    #[test]
    fn scroll_thumb_tracks_pan() {
        let (offset, size) = scroll_thumb(px(100.), 1., px(0.));
        assert_eq!((offset, size), (0., 1.));

        let (_, size) = scroll_thumb(px(100.), 2., px(0.));
        assert!((size - 0.5).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), 2., px(50.));
        assert!((offset - 0.).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), 2., px(0.));
        assert!((offset - 0.25).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), 2., px(-50.));
        assert!((offset - 0.5).abs() < 1e-6);
    }

    #[test]
    fn scroll_thumb_is_stable_without_overflow() {
        let (offset, size) = scroll_thumb(Pixels::ZERO, 1., px(0.));
        assert_eq!((offset, size), (0., 1.));
    }

    #[test]
    fn collapse_home_to_tilde() {
        let home = Path::new("/home/renato");
        assert_eq!(
            collapse_home("/home/renato/img/out.txt", Some(home)),
            "~/img/out.txt"
        );
        assert_eq!(collapse_home("/home/renato", Some(home)), "~");
        assert_eq!(collapse_home("/tmp/out.txt", Some(home)), "/tmp/out.txt");
        assert_eq!(
            collapse_home("/home/renato2/out.txt", Some(home)),
            "/home/renato2/out.txt"
        );
    }
}
