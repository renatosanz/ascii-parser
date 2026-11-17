use gpui::{prelude::FluentBuilder, *};

use crate::colorscheme::ColorScheme;

#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: &'static [u8],
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    disabled: bool,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: &'static [u8]) -> Self {
        IconButton {
            id: id.into(),
            icon,
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

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled;
        let background_color = if disabled {
            ColorScheme::BgSurfaceHover
        } else {
            ColorScheme::BgSurface
        };
        let icon_color = if disabled {
            ColorScheme::TextMuted
        } else {
            ColorScheme::TextSecondary
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .size(px(34.))
            .rounded_md()
            .border_1()
            .border_color(rgb(ColorScheme::BorderSubtle.into()))
            .bg(rgb(background_color.into()))
            .when(!disabled, |this| {
                this.hover(|style| style.bg(rgb(ColorScheme::BgSurfaceHover.into())))
            })
            .when_some(self.on_click, |this, on_click| {
                if disabled {
                    this
                } else {
                    this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
                }
            })
            .child(
                svg()
                    .size(px(16.))
                    .data(self.icon)
                    .text_color(rgb(icon_color.into())),
            )
    }
}
