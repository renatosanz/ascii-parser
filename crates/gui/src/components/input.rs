use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::colorscheme::ColorScheme;

#[derive(IntoElement)]
pub struct Input {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    disabled: bool,
}

impl Input {
    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
        focus_handle: FocusHandle,
    ) -> Self {
        Input {
            id: id.into(),
            value: value.into(),
            placeholder: SharedString::default(),
            focus_handle,
            on_change: None,
            disabled: false,
        }
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Input {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let value_for_keys = self.value.clone();
        let on_change = self.on_change;
        let disabled = self.disabled;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .h(px(36.))
            .px_3()
            .overflow_hidden()
            .whitespace_nowrap()
            .rounded_md()
            .border_1()
            .border_color(rgb(ColorScheme::BorderSubtle.into()))
            .bg(rgb(ColorScheme::BgSurface.into()))
            .text_sm()
            .when(disabled, |this| this.opacity(0.5))
            .track_focus(&focus_handle)
            .focus(|style| style.border_color(rgb(ColorScheme::BorderBrand.into())))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                if disabled {
                    return;
                }
                window.focus(&focus_handle, cx);
            })
            // shortcut: single line, no selection/clipboard/IME until a real text field is needed
            .on_key_down(move |event, window, cx| {
                if disabled {
                    return;
                }
                let Some(on_change) = on_change.as_ref() else {
                    return;
                };

                let mut text = value_for_keys.to_string();
                if event.keystroke.key == "backspace" {
                    text.pop();
                } else {
                    match event.keystroke.key_char.as_deref() {
                        Some(ch)
                            if ch.chars().count() == 1
                                && !ch.chars().next().is_some_and(char::is_control) =>
                        {
                            text.push_str(ch)
                        }
                        _ => return,
                    }
                }

                on_change(&text, window, cx);
            })
            .when(self.value.is_empty(), |this| {
                this.child(
                    div()
                        .text_color(rgb(ColorScheme::TextMuted.into()))
                        .child(self.placeholder),
                )
            })
            .when(!self.value.is_empty(), |this| this.child(self.value))
    }
}
