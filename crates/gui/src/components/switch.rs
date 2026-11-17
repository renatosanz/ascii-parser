use gpui::{prelude::FluentBuilder, *};

use crate::colorscheme::ColorScheme;

#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    label: SharedString,
    checked: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, checked: bool) -> Self {
        Switch {
            id: id.into(),
            label: label.into(),
            checked,
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let checked = self.checked;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .cursor_pointer()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(ColorScheme::TextPrimary.into()))
                    .child(self.label),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(36.))
                    .h(px(20.))
                    .p(px(2.))
                    .rounded_full()
                    .when(checked, |this| {
                        this.justify_end().bg(rgb(ColorScheme::BgBrand.into()))
                    })
                    .when(!checked, |this| {
                        this.justify_start()
                            .bg(rgb(ColorScheme::BorderStrong.into()))
                    })
                    .child(
                        div()
                            .size(px(16.))
                            .rounded_full()
                            .bg(rgb(ColorScheme::BgSurface.into())),
                    ),
            )
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
            })
    }
}
