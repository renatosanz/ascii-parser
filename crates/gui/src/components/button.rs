use gpui::{prelude::FluentBuilder, *};

use crate::colorscheme::ColorScheme;

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    disabled: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: SharedString) -> Self {
        Button {
            id: id.into(),
            label,
            on_click: None,
            disabled: false,
        }
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
        let border_color = match self.disabled {
            false => ColorScheme::BorderBrand.into(),
            true => ColorScheme::ColorGreen100.into(),
        };
        let background_color = match self.disabled {
            false => ColorScheme::ColorGreen100.into(),
            true => ColorScheme::ColorGray100.into(),
        };

        div()
            .id(self.id)
            .flex()
            .justify_center()
            .text_lg()
            .border_2()
            .p_1()
            .rounded_lg()
            .border_color(rgb(border_color))
            .text_color(rgb(ColorScheme::TextPrimary.into()))
            .bg(rgb(background_color))
            .hover(|style| {
                if !self.disabled {
                    style.bg(rgb(ColorScheme::ColorGreen300.into()))
                } else {
                    style
                }
            })
            .when_some(self.on_click, |this, on_click| {
                if !self.disabled {
                    this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
                } else {
                    this
                }
            })
            .child(self.label)
    }
}
