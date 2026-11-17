use base::{BrightnessFn, ColoringFn, FontsMono, ParsingGradient};
use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::{
    colorscheme::ColorScheme,
    components::{Button, Dropdown, Input, Label, Slider},
    state::{Screen, State},
};

pub fn render(state: &State, cx: &mut Context<State>) -> impl IntoElement {
    let Screen::Config(config) = &state.screen else {
        return div();
    };
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

    let max_size = config.max_size;
    let font_size = config.font_size;
    let padding_x = config.padding_x;
    let padding_y = config.padding_y;
    let font = config.font;
    let gradient = config.gradient;
    let coloring = config.coloring;
    let brightness = config.brightness;
    let status = config.status.clone();

    let controls =
        div()
            .flex()
            .flex_col()
            .gap_3()
            .max_w_1_2()
            .h_full()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .child("Settings"),
            )
            .child(
                div()
                    .id("controls")
                    .flex()
                    .flex_col()
                    .gap_3()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(field(
                        format!("Max side size: {max_size:.0}"),
                        Slider::new("max-size", max_size, 50., 400.).on_change({
                            let entity = entity.clone();
                            move |value, _, cx| {
                                entity.update(cx, |state, cx| {
                                    if let Screen::Config(config) = &mut state.screen {
                                        config.max_size = value;
                                    }
                                    cx.notify();
                                });
                            }
                        }),
                    ))
                    .child(field(
                        format!("Font size: {font_size:.0}"),
                        Slider::new("font-size", font_size, 5., 30.).on_change({
                            let entity = entity.clone();
                            move |value, _, cx| {
                                entity.update(cx, |state, cx| {
                                    if let Screen::Config(config) = &mut state.screen {
                                        config.font_size = value;
                                    }
                                    cx.notify();
                                });
                            }
                        }),
                    ))
                    .child(field(
                        format!("Padding X: {padding_x:.0}"),
                        Slider::new("padding-x", padding_x, 0., 30.).on_change({
                            let entity = entity.clone();
                            move |value, _, cx| {
                                entity.update(cx, |state, cx| {
                                    if let Screen::Config(config) = &mut state.screen {
                                        config.padding_x = value;
                                    }
                                    cx.notify();
                                });
                            }
                        }),
                    ))
                    .child(field(
                        format!("Padding Y: {padding_y:.0}"),
                        Slider::new("padding-y", padding_y, 0., 30.).on_change({
                            let entity = entity.clone();
                            move |value, _, cx| {
                                entity.update(cx, |state, cx| {
                                    if let Screen::Config(config) = &mut state.screen {
                                        config.padding_y = value;
                                    }
                                    cx.notify();
                                });
                            }
                        }),
                    ))
                    .child(field(
                        "Font",
                        Dropdown::new("font", font.name(), font_options).on_change({
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
                                    if let Screen::Config(config) = &mut state.screen {
                                        config.font = font;
                                    }
                                    cx.notify();
                                });
                            }
                        }),
                    ))
                    .child(field(
                        "Gradient",
                        Dropdown::new("gradient", format!("{gradient:?}"), gradient_options)
                            .on_change({
                                let entity = entity.clone();
                                move |picked, _, cx| {
                                    let Some(gradient) = ParsingGradient::ALL
                                        .iter()
                                        .copied()
                                        .find(|value| format!("{value:?}") == picked.as_str())
                                    else {
                                        return;
                                    };
                                    entity.update(cx, |state, cx| {
                                        if let Screen::Config(config) = &mut state.screen {
                                            config.gradient = gradient;
                                        }
                                        cx.notify();
                                    });
                                }
                            }),
                    ))
                    .child(field(
                        "Coloring",
                        Dropdown::new("coloring", format!("{coloring:?}"), coloring_options)
                            .on_change({
                                let entity = entity.clone();
                                move |picked, _, cx| {
                                    let Some(coloring) = ColoringFn::ALL
                                        .iter()
                                        .copied()
                                        .find(|value| format!("{value:?}") == picked.as_str())
                                    else {
                                        return;
                                    };
                                    entity.update(cx, |state, cx| {
                                        if let Screen::Config(config) = &mut state.screen {
                                            config.coloring = coloring;
                                        }
                                        cx.notify();
                                    });
                                }
                            }),
                    ))
                    .child(field(
                        "Brightness",
                        Dropdown::new("brightness", format!("{brightness:?}"), brightness_options)
                            .on_change({
                                let entity = entity.clone();
                                move |picked, _, cx| {
                                    let Some(brightness) = BrightnessFn::ALL
                                        .iter()
                                        .copied()
                                        .find(|value| format!("{value:?}") == picked.as_str())
                                    else {
                                        return;
                                    };
                                    entity.update(cx, |state, cx| {
                                        if let Screen::Config(config) = &mut state.screen {
                                            config.brightness = brightness;
                                        }
                                        cx.notify();
                                    });
                                }
                            }),
                    ))
                    .child(field(
                        "Output PNG",
                        Input::new(
                            "output-png",
                            config.output_png.clone(),
                            state.png_focus.clone(),
                        )
                        .on_change({
                            let entity = entity.clone();
                            move |value, _, cx| {
                                entity.update(cx, |state, cx| {
                                    if let Screen::Config(config) = &mut state.screen {
                                        config.output_png = value.to_string();
                                    }
                                    cx.notify();
                                });
                            }
                        }),
                    ))
                    .child(field(
                        "Output TXT (empty disables)",
                        Input::new(
                            "output-txt",
                            config.output_txt.clone(),
                            state.txt_focus.clone(),
                        )
                        .on_change({
                            let entity = entity.clone();
                            move |value, _, cx| {
                                entity.update(cx, |state, cx| {
                                    if let Screen::Config(config) = &mut state.screen {
                                        config.output_txt = value.to_string();
                                    }
                                    cx.notify();
                                });
                            }
                        }),
                    )),
            )
            .child(Button::new("start", "Start".into()).on_click(cx.listener(State::handle_start)))
            .when_some(status, |this, (message, is_error)| {
                let color = if is_error {
                    ColorScheme::ColorErrorText
                } else {
                    ColorScheme::ColorSuccessText
                };
                this.child(div().text_sm().text_color(rgb(color.into())).child(message))
            });

    div()
        .flex()
        .flex_row()
        .gap_4()
        .size_full()
        .p_6()
        .child(
            div()
                .flex_1()
                .min_w_0()
                .size_full()
                .max_w_1_2()
                .rounded_lg()
                .items_center()
                .content_center()
                .border_1()
                .border_color(rgb(ColorScheme::BorderSubtle.into()))
                .bg(rgb(ColorScheme::BgSurface.into()))
                .child(
                    img(config.image_path.clone())
                        .m_0()
                        .w_40()
                        .h_40()
                        .object_fit(ObjectFit::Contain),
                ),
        )
        .child(controls)
}

fn field(label: impl Into<SharedString>, control: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(Label::new(label))
        .child(control)
}
