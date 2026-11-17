use base::{BrightnessFn, ColoringFn, FontsMono, ParsingGradient};
use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::{
    colorscheme::ColorScheme,
    components::{Button, Dropdown, Input, Label, Slider},
    state::State,
};

pub fn render(state: &State, cx: &mut Context<State>) -> impl IntoElement {
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
        .gap(px(12.))
        .child(
            div()
                .id("image_views_section")
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .min_h_0()
                .overflow_y_scroll()
                .child(
                    div()
                        .id("original")
                        .flex_grow(5.)
                        .border_2()
                        .border_color(rgb(ColorScheme::BorderSubtle.into()))
                        .rounded_xl(),
                )
                .child(
                    div()
                        .id("state_icon")
                        .m_auto()
                        .flex_grow(1.)
                        .items_center()
                        .content_center()
                        .child(
                            div()
                                .h_12()
                                .w_12()
                                .border_2()
                                .border_color(rgb(ColorScheme::BorderSubtle.into()))
                                .rounded_xl(),
                        ),
                )
                .child(
                    div()
                        .id("rendered")
                        .flex_grow(5.)
                        .border_2()
                        .border_color(rgb(ColorScheme::BorderSubtle.into()))
                        .rounded_xl(),
                ),
        )
        .child(
            div()
                .id("config_section")
                .flex()
                .flex_col()
                .w(px(240.))
                .flex_shrink_0()
                .gap(px(12.))
                .h_full()
                .min_h_0()
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
                                .p_4()
                                .border_2()
                                .flex()
                                .flex_col()
                                .flex_shrink_0()
                                .border_color(rgb(ColorScheme::BorderSubtle.into()))
                                .rounded_xl()
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
                                                cx.notify();
                                            });
                                        }
                                    }),
                                ))
                                .child(field(
                                    "Output PNG",
                                    Input::new("output-png", output_png, state.png_focus.clone())
                                        .disabled(ui_disabled)
                                        .on_change({
                                            let entity = entity.clone();
                                            move |value, _, cx| {
                                                entity.update(cx, |state, cx| {
                                                    state.config.output_png =
                                                        value.to_string().into();
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                ))
                                .child(field(
                                    "Output TXT (empty disables)",
                                    Input::new("output-txt", output_txt, state.txt_focus.clone())
                                        .disabled(ui_disabled)
                                        .on_change({
                                            let entity = entity.clone();
                                            move |value, _, cx| {
                                                entity.update(cx, |state, cx| {
                                                    state.config.output_txt =
                                                        value.to_string().into();
                                                    cx.notify();
                                                });
                                            }
                                        }),
                                )),
                        ),
                )
                .child(
                    div()
                        .p_4()
                        .border_2()
                        .flex()
                        .flex_col()
                        .flex_shrink_0()
                        .gap_2()
                        .border_color(rgb(ColorScheme::BorderSubtle.into()))
                        .rounded_xl()
                        .child(
                            Button::new("start", "Start".into())
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

fn field(label: impl Into<SharedString>, control: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(Label::new(label))
        .child(control)
}
