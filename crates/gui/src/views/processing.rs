use base::{BrightnessFn, ColoringFn, FontsMono, ParsingGradient};
use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::{
    colorscheme::ColorScheme,
    components::{Button, Dropdown, IconButton, Label, Slider},
    state::State,
    views::image_preview,
};

const OPEN_FOLDER: &[u8] = include_bytes!("../../assets/svg/open_folder.svg");

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
                    .child(image_preview::render(
                        "original",
                        "Original",
                        state.config.image_path.clone(),
                        state.original_view,
                        true,
                        entity.clone(),
                        window,
                        cx,
                    ))
                    .child(image_preview::render(
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

#[cfg(test)]
mod tests {
    use super::collapse_home;
    use std::path::Path;

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
