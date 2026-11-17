use std::{cell::Cell, rc::Rc};

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::colorscheme::ColorScheme;

#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    value: f32,
    min: f32,
    max: f32,
    on_change: Option<Box<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
    disabled: bool,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>, value: f32, min: f32, max: f32) -> Self {
        Slider {
            id: id.into(),
            value,
            min,
            max,
            on_change: None,
            disabled: false,
        }
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bounds: Rc<Cell<Option<Bounds<Pixels>>>> = Rc::new(Cell::new(None));
        let bounds_for_canvas = bounds.clone();
        let disabled = self.disabled;
        let on_change: Rc<Option<Box<dyn Fn(f32, &mut Window, &mut App)>>> =
            Rc::new(self.on_change);

        let min = self.min;
        let span = (self.max - self.min).max(f32::EPSILON);
        let fraction = ((self.value - min) / span).clamp(0.0, 1.0);

        let value_at = {
            let bounds = bounds.clone();
            let on_change = on_change.clone();
            move |position: Pixels, window: &mut Window, cx: &mut App| {
                let (Some(bounds), Some(on_change)) = (bounds.get(), on_change.as_ref()) else {
                    return;
                };
                let fraction = ((position - bounds.left()) / bounds.size.width).clamp(0.0, 1.0);
                on_change(min + fraction * span, window, cx);
            }
        };

        let on_down = {
            let value_at = value_at.clone();
            move |event: &MouseDownEvent, window: &mut Window, cx: &mut App| {
                if disabled {
                    return;
                }
                value_at(event.position.x, window, cx)
            }
        };

        let on_move = {
            let value_at = value_at.clone();
            move |event: &MouseMoveEvent, window: &mut Window, cx: &mut App| {
                if disabled {
                    return;
                }
                if event.pressed_button == Some(MouseButton::Left) {
                    value_at(event.position.x, window, cx)
                }
            }
        };

        div()
            .id(self.id)
            .relative()
            .h(px(16.))
            .w_full()
            .when(disabled, |this| this.opacity(0.5))
            .child(
                canvas(
                    move |bounds, _, _| bounds_for_canvas.set(Some(bounds)),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .child(
                div()
                    .absolute()
                    .top(px(6.))
                    .left_0()
                    .h(px(4.))
                    .w_full()
                    .rounded_full()
                    .bg(rgb(ColorScheme::BorderSubtle.into())),
            )
            .child(
                div()
                    .absolute()
                    .top(px(6.))
                    .left_0()
                    .h(px(4.))
                    .w(relative(fraction))
                    .rounded_full()
                    .bg(rgb(ColorScheme::BgBrand.into())),
            )
            .child(
                div()
                    .absolute()
                    .top(px(0.))
                    .left(relative(fraction))
                    .ml(px(-8.))
                    .size(px(16.))
                    .rounded_full()
                    .border_2()
                    .border_color(rgb(ColorScheme::BgSurface.into()))
                    .bg(rgb(ColorScheme::BgBrand.into())),
            )
            .on_mouse_down(MouseButton::Left, on_down)
            // shortcut: drag tracking stops when the pointer leaves the track
            .on_mouse_move(on_move)
    }
}
