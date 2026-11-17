use std::sync::Arc;

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::colorscheme::ColorScheme;

#[derive(IntoElement)]
pub struct Dropdown {
    id: ElementId,
    selected: SharedString,
    options: Vec<SharedString>,
    on_change: Option<Box<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
    disabled: bool,
}

impl Dropdown {
    pub fn new(
        id: impl Into<ElementId>,
        selected: impl Into<SharedString>,
        options: Vec<SharedString>,
    ) -> Self {
        Dropdown {
            id: id.into(),
            selected: selected.into(),
            options,
            on_change: None,
            disabled: false,
        }
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Dropdown {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let open = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), "open".into()),
            cx,
            |_, _| false,
        );
        let is_open = *open.read(cx) && !self.disabled;

        let on_change: Arc<Option<Box<dyn Fn(&SharedString, &mut Window, &mut App)>>> =
            Arc::new(self.on_change);
        let selected = self.selected;

        let toggle = {
            let open = open.clone();
            let disabled = self.disabled;
            move |_: &MouseDownEvent, _: &mut Window, cx: &mut App| {
                if disabled {
                    return;
                }
                open.update(cx, |open, cx| {
                    *open = !*open;
                    cx.notify();
                });
            }
        };

        let close = {
            let open = open.clone();
            move |_: &MouseDownEvent, _: &mut Window, cx: &mut App| {
                open.update(cx, |open, cx| {
                    *open = false;
                    cx.notify();
                });
            }
        };

        div()
            .relative()
            .flex()
            .flex_col()
            .when(self.disabled, |this| this.opacity(0.5))
            .child(
                div()
                    .id(self.id.clone())
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(36.))
                    .px_3()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(ColorScheme::BorderSubtle.into()))
                    .bg(rgb(ColorScheme::BgSurface.into()))
                    .text_sm()
                    .on_mouse_down(MouseButton::Left, toggle)
                    .child(selected.clone())
                    .child(
                        div()
                            .text_color(rgb(ColorScheme::TextMuted.into()))
                            .child("v"),
                    ),
            )
            .when(is_open, |this| {
                this.child(deferred(
                    anchored().position(point(px(0.), px(0.))).child(
                        div()
                            .id(format!("dropdown-backdrop_{}", self.id))
                            .occlude()
                            .w(window.viewport_size().width)
                            .h(window.viewport_size().height)
                            .on_mouse_down(MouseButton::Left, close),
                    ),
                ))
                .child(deferred(
                    div()
                        .id(format!("list_options_{}", self.id))
                        .flex()
                        .flex_col()
                        .absolute()
                        .top(px(30.))
                        .left(px(0.))
                        .mt_1()
                        .w_full()
                        .max_h_32()
                        .overflow_y_scroll()
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(ColorScheme::BorderSubtle.into()))
                        .bg(rgb(ColorScheme::BgSurface.into()))
                        .children(self.options.into_iter().enumerate().map(|(ix, option)| {
                            let is_selected = option == selected;
                            let on_change = on_change.clone();
                            let open = open.clone();
                            let option_for_handler = option.clone();

                            div()
                                .id(format!("dropdown-option-{}-{ix}", self.id))
                                .px_3()
                                .py_2()
                                .text_sm()
                                .when(is_selected, |this| {
                                    this.bg(rgb(ColorScheme::BgBrandSubtle.into()))
                                })
                                .hover(|this| this.bg(rgb(ColorScheme::BgSurfaceHover.into())))
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    open.update(cx, |open, cx| {
                                        *open = false;
                                        cx.notify();
                                    });
                                    if let Some(on_change) = on_change.as_ref() {
                                        on_change(&option_for_handler, window, cx);
                                    }
                                })
                                .child(option)
                        })),
                ))
            })
    }
}
