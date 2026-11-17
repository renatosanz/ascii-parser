use gpui::{prelude::FluentBuilder, *};

use crate::colorscheme::ColorScheme;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ButtonVariant {
    Primary,
    Secondary,
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    disabled: bool,
    variant: ButtonVariant,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Button {
            id: id.into(),
            label,
            on_click: None,
            disabled: false,
            variant: ButtonVariant::Primary,
        }
    }

    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled;
        let variant = self.variant;

        let background_color = match (variant, disabled) {
            (_, true) => ColorScheme::ColorGray100,
            (ButtonVariant::Primary, false) => ColorScheme::BgBrand,
            (ButtonVariant::Secondary, false) => ColorScheme::BgSurface,
        };
        let text_color = match disabled {
            true => ColorScheme::TextMuted,
            false => match variant {
                ButtonVariant::Primary => ColorScheme::TextOnBrand,
                ButtonVariant::Secondary => ColorScheme::TextPrimary,
            },
        };
        let border_color = match (variant, disabled) {
            (ButtonVariant::Secondary, false) => Some(ColorScheme::BorderSubtle),
            _ => None,
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .h(px(34.))
            .px_4()
            .rounded_md()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(text_color.into()))
            .bg(rgb(background_color.into()))
            .when_some(border_color, |this, color| {
                this.border_1().border_color(rgb(color.into()))
            })
            .hover(|style| {
                if disabled {
                    style
                } else {
                    match variant {
                        ButtonVariant::Primary => style.bg(rgb(ColorScheme::BgBrandHover.into())),
                        ButtonVariant::Secondary => {
                            style.bg(rgb(ColorScheme::BgSurfaceHover.into()))
                        }
                    }
                }
            })
            .when_some(self.on_click, |this, on_click| {
                if disabled {
                    this
                } else {
                    this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
                }
            })
            .child(self.label)
    }
}
