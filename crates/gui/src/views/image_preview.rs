use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::{
    colorscheme::ColorScheme,
    components::IconButton,
    state::{ImageView, State},
};

const ZOOM_OUT: &[u8] = include_bytes!("../../assets/svg/zoom_out.svg");

/// One image preview panel with wheel zoom, drag/scroll pan, and a reset button.
pub fn render(
    id: &'static str,
    caption: &'static str,
    source: Option<std::path::PathBuf>,
    view: ImageView,
    original: bool,
    entity: Entity<State>,
    window: &mut Window,
    cx: &mut Context<State>,
) -> impl IntoElement {
    let viewport_state = window.use_keyed_state(
        ElementId::Name(format!("{id}-viewport").into()),
        cx,
        |_, _| None::<Size<Pixels>>,
    );
    let viewport = *viewport_state.read(cx);
    let record_viewport = {
        let viewport_state = viewport_state.clone();
        move |children: Vec<Bounds<Pixels>>, _: &mut Window, cx: &mut App| {
            let Some(size) = children.first().map(|bounds| bounds.size) else {
                return;
            };
            viewport_state.update(cx, |slot, cx| {
                if *slot != Some(size) {
                    *slot = Some(size);
                    cx.notify();
                }
            });
        }
    };

    let measured = viewport.unwrap_or_default();
    let pan = view.clamped_pan(measured);
    let zoomed = view.zoom > 1.;
    let width = measured.width * view.zoom;
    let height = measured.height * view.zoom;
    let left = measured.width * ((1. - view.zoom) / 2.) + pan.x;
    let top = measured.height * ((1. - view.zoom) / 2.) + pan.y;
    let (vertical_offset, vertical_size) = scroll_thumb(measured.height, view.zoom, pan.y);
    let (horizontal_offset, horizontal_size) = scroll_thumb(measured.width, view.zoom, pan.x);
    let has_source = source.is_some();

    let on_scroll = {
        let entity = entity.clone();
        move |event: &ScrollEvent, _: &mut Window, cx: &mut App| {
            if !has_source {
                return;
            }
            let delta = scroll_delta(&event.delta);
            entity.update(cx, |state, cx| {
                if delta.y != px(0.) {
                    let factor = (1. + f32::from(delta.y) / 250.).clamp(0.5, 2.);
                    state.zoom_image(original, factor, measured);
                }
                if delta.x != px(0.) {
                    state.pan_image(original, point(-delta.x, px(0.)), measured);
                }
                cx.notify();
            });
            cx.stop_propagation();
        }
    };

    let on_mouse_down = {
        let entity = entity.clone();
        move |event: &MouseDownEvent, _: &mut Window, cx: &mut App| {
            if !has_source {
                return;
            }
            entity.update(cx, |state, _| state.begin_drag(original, event.position));
            cx.stop_propagation();
        }
    };

    let on_mouse_move = {
        let entity = entity.clone();
        move |event: &MouseMoveEvent, _: &mut Window, cx: &mut App| {
            if !has_source || !event.dragging() {
                return;
            }
            entity.update(cx, |state, cx| {
                state.drag_image(original, event.position, measured);
                cx.notify();
            });
            cx.stop_propagation();
        }
    };

    let on_mouse_up = {
        let entity = entity.clone();
        move |_: &MouseUpEvent, _: &mut Window, cx: &mut App| {
            entity.update(cx, |state, _| state.end_drag(original));
        }
    };

    let reset = {
        let entity = entity.clone();
        move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            entity.update(cx, |state, cx| {
                state.reset_image(original);
                cx.notify();
            });
        }
    };

    div()
        .id(id)
        .flex_grow(5.)
        .flex_basis(px(0.))
        .min_h(px(120.))
        .flex()
        .flex_col()
        .overflow_hidden()
        .border_1()
        .border_color(rgb(ColorScheme::BorderSubtle.into()))
        .bg(rgb(ColorScheme::BgSurface.into()))
        .rounded_lg()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .px_3()
                .py_2()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(ColorScheme::TextSecondary.into()))
                .child(caption)
                .child(
                    IconButton::new(format!("{id}-reset"), ZOOM_OUT)
                        .compact()
                        .on_click(reset),
                ),
        )
        .child(
            div()
                .on_children_prepainted(record_viewport)
                .flex_1()
                .min_h_0()
                .flex()
                .px_3()
                .pb_3()
                .child(
                    div()
                        .id(format!("{id}-viewport"))
                        .relative()
                        .flex_1()
                        .h_full()
                        .flex()
                        .items_center()
                        .content_center()
                        .overflow_hidden()
                        .cursor(if zoomed {
                            CursorStyle::OpenHand
                        } else {
                            CursorStyle::Arrow
                        })
                        .on_scroll(on_scroll)
                        .on_mouse_down(MouseButton::Left, on_mouse_down)
                        .on_mouse_move(on_mouse_move)
                        .on_mouse_up(MouseButton::Left, on_mouse_up)
                        .when_some(source, |this, path| {
                            this.child(if zoomed {
                                div()
                                    .absolute()
                                    .left(left)
                                    .top(top)
                                    .w(width)
                                    .h(height)
                                    .child(img(path).size_full().object_fit(ObjectFit::Contain))
                                    .into_any_element()
                            } else {
                                img(path)
                                    .size_full()
                                    .object_fit(ObjectFit::Contain)
                                    .into_any_element()
                            })
                        })
                        .when(zoomed, |this| {
                            this.child(
                                div()
                                    .absolute()
                                    .left(px(2.))
                                    .top(px(4.))
                                    .bottom(px(10.))
                                    .w(px(4.))
                                    .rounded_full()
                                    .bg(rgb(ColorScheme::BorderSubtle.into()))
                                    .child(
                                        div()
                                            .absolute()
                                            .top(relative(vertical_offset))
                                            .left(px(0.))
                                            .w_full()
                                            .h(relative(vertical_size))
                                            .rounded_full()
                                            .bg(rgb(ColorScheme::BorderStrong.into())),
                                    ),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .bottom(px(2.))
                                    .left(px(4.))
                                    .right(px(10.))
                                    .h(px(4.))
                                    .rounded_full()
                                    .bg(rgb(ColorScheme::BorderSubtle.into()))
                                    .child(
                                        div()
                                            .absolute()
                                            .left(relative(horizontal_offset))
                                            .top(px(0.))
                                            .h_full()
                                            .w(relative(horizontal_size))
                                            .rounded_full()
                                            .bg(rgb(ColorScheme::BorderStrong.into())),
                                    ),
                            )
                        }),
                ),
        )
}

fn scroll_delta(delta: &ScrollDelta) -> Point<Pixels> {
    match delta {
        ScrollDelta::Pixels(delta) => *delta,
        ScrollDelta::Lines(delta) => point(px(delta.x * 20.), px(delta.y * 20.)),
    }
}

/// Returns the thumb offset and size as fractions of the scrollbar track.
fn scroll_thumb(viewport: Pixels, zoom: f32, pan: Pixels) -> (f32, f32) {
    let span = f32::from(viewport) * (zoom - 1.);
    let fraction = if span > 0. {
        (0.5 - f32::from(pan) / span).clamp(0., 1.)
    } else {
        0.
    };
    (fraction * (1. - 1. / zoom), 1. / zoom)
}

#[cfg(test)]
mod tests {
    use super::scroll_thumb;
    use gpui::{Pixels, px};

    #[test]
    fn scroll_thumb_tracks_pan() {
        let (offset, size) = scroll_thumb(px(100.), 1., px(0.));
        assert_eq!((offset, size), (0., 1.));

        let (_, size) = scroll_thumb(px(100.), 2., px(0.));
        assert!((size - 0.5).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), 2., px(50.));
        assert!((offset - 0.).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), 2., px(0.));
        assert!((offset - 0.25).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), 2., px(-50.));
        assert!((offset - 0.5).abs() < 1e-6);
    }

    #[test]
    fn scroll_thumb_is_stable_without_overflow() {
        let (offset, size) = scroll_thumb(Pixels::ZERO, 1., px(0.));
        assert_eq!((offset, size), (0., 1.));
    }
}
