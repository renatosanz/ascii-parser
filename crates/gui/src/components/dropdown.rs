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
        let trigger_bounds = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), "trigger-bounds".into()),
            cx,
            |_, _| None::<Bounds<Pixels>>,
        );
        // Captured during the previous frame, so the menu can flip before it overflows.
        let known_bounds = *trigger_bounds.read(cx);
        let record_bounds = {
            let trigger_bounds = trigger_bounds.clone();
            move |children: Vec<Bounds<Pixels>>, _: &mut Window, cx: &mut App| {
                let Some(bounds) = children.first().copied() else {
                    return;
                };
                trigger_bounds.update(cx, |slot, cx| {
                    if *slot != Some(bounds) {
                        *slot = Some(bounds);
                        cx.notify();
                    }
                });
            }
        };
        let is_open = *open.read(cx) && !self.disabled;
        let id = self.id.clone();

        let on_change: Arc<Option<Box<dyn Fn(&SharedString, &mut Window, &mut App)>>> =
            Arc::new(self.on_change);
        let selected = self.selected;
        let options = self.options;
        let trigger_bg = if self.disabled {
            ColorScheme::BgSurfaceHover
        } else {
            ColorScheme::BgSurface
        };
        let trigger_text = if self.disabled {
            ColorScheme::TextMuted
        } else {
            ColorScheme::TextPrimary
        };

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
            .child(
                div().on_children_prepainted(record_bounds).child(
                    div()
                        .id(id.clone())
                        .flex()
                        .items_center()
                        .justify_between()
                        .h(px(34.))
                        .px_3()
                        .rounded_md()
                        .border_1()
                        .border_color(rgb(ColorScheme::BorderSubtle.into()))
                        .bg(rgb(trigger_bg.into()))
                        .text_sm()
                        .text_color(rgb(trigger_text.into()))
                        .on_mouse_down(MouseButton::Left, toggle)
                        .child(selected.clone())
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(ColorScheme::TextMuted.into()))
                                .child("▾"),
                        ),
                ),
            )
            .when(is_open, move |this| {
                let list = div()
                    .id(format!("list_options_{id}"))
                    .flex()
                    .flex_col()
                    .max_h_32()
                    .overflow_y_scroll()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(ColorScheme::BorderSubtle.into()))
                    .bg(rgb(ColorScheme::BgSurface.into()))
                    .shadow_sm()
                    .children(options.into_iter().enumerate().map(|(ix, option)| {
                        let is_selected = option == selected;
                        let on_change = on_change.clone();
                        let open = open.clone();
                        let option_for_handler = option.clone();

                        div()
                            .id(format!("dropdown-option-{id}-{ix}"))
                            .px_3()
                            .py_2()
                            .text_sm()
                            .text_color(rgb(ColorScheme::TextPrimary.into()))
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
                    }));

                // Attach to the trigger so gpui flips it above and snaps it to the window
                // edges. Falls back to below the trigger until the first frame is measured.
                let menu = if let Some(bounds) = known_bounds {
                    anchored()
                        .attach(bounds, Anchor::BottomLeft)
                        .offset(point(px(0.), px(4.)))
                        .child(list.w(bounds.size.width))
                        .into_any_element()
                } else {
                    list.absolute()
                        .top(px(38.))
                        .left(px(0.))
                        .w_full()
                        .into_any_element()
                };

                this.child(deferred(
                    anchored().position(point(px(0.), px(0.))).child(
                        div()
                            .id(format!("dropdown-backdrop_{id}"))
                            .occlude()
                            .w(window.viewport_size().width)
                            .h(window.viewport_size().height)
                            .on_mouse_down(MouseButton::Left, close),
                    ),
                ))
                .child(deferred(menu))
            })
    }
}
