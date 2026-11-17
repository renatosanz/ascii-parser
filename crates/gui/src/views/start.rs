use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::{colorscheme::ColorScheme, components::Button, state::State};

pub fn render(state: &State, cx: &mut Context<State>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .justify_between()
        .size_full()
        .p_6()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::BOLD)
                        .child("ascii-parser"),
                )
                .child(
                    div()
                        .text_color(rgb(ColorScheme::TextSecondary.into()))
                        .child(
                            "A Rust image parser that converts images into ASCII characters, \
                             producing both plain-text output and rendered PNG images.",
                        ),
                )
                .when_some(state.error.as_ref(), |this, error| {
                    this.child(
                        div()
                            .text_color(rgb(ColorScheme::ColorErrorText.into()))
                            .child(error.clone()),
                    )
                }),
        )
        .child(
            Button::new("select-file", "Select file".into())
                .on_click(cx.listener(State::handle_select_file)),
        )
}
