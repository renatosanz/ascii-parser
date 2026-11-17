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
    let fitted = fitted_size(source.as_deref(), measured);
    let pan = view.clamped_pan(measured, fitted);
    let zoomed = view.zoom > 1.;
    let content_width = fitted.width * view.zoom;
    let content_height = fitted.height * view.zoom;
    let left = (measured.width - content_width) / 2. + pan.x;
    let top = (measured.height - content_height) / 2. + pan.y;
    let vertical_bar = scroll_thumb(measured.height, content_height, pan.y);
    let horizontal_bar = scroll_thumb(measured.width, content_width, pan.x);
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
                    state.zoom_image(original, factor, measured, fitted);
                }
                if delta.x != px(0.) {
                    state.pan_image(original, point(-delta.x, px(0.)), measured, fitted);
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
                state.drag_image(original, event.position, measured, fitted);
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
                                    .w(content_width)
                                    .h(content_height)
                                    .child(img(path).size_full().object_fit(ObjectFit::Contain))
                                    .into_any_element()
                            } else {
                                img(path)
                                    .size_full()
                                    .object_fit(ObjectFit::Contain)
                                    .into_any_element()
                            })
                        })
                        .when_some(vertical_bar, |this, (offset, size)| {
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
                                            .top(relative(offset))
                                            .left(px(0.))
                                            .w_full()
                                            .h(relative(size))
                                            .rounded_full()
                                            .bg(rgb(ColorScheme::BorderStrong.into())),
                                    ),
                            )
                        })
                        .when_some(horizontal_bar, |this, (offset, size)| {
                            this.child(
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
                                            .left(relative(offset))
                                            .top(px(0.))
                                            .h_full()
                                            .w(relative(size))
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

/// Size the image is drawn at when fitted into `viewport` at zoom 1.
fn fitted_size(source: Option<&std::path::Path>, viewport: Size<Pixels>) -> Size<Pixels> {
    let Some((w, h)) = source.and_then(|path| image::image_dimensions(path).ok()) else {
        return viewport;
    };
    if w == 0 || h == 0 {
        return viewport;
    }
    let scale = (f32::from(viewport.width) / w as f32).min(f32::from(viewport.height) / h as f32);
    size(px(w as f32 * scale), px(h as f32 * scale))
}

/// Returns the thumb offset and size as fractions of the scrollbar track, or
/// `None` when the content fits and no scrolling is possible.
fn scroll_thumb(viewport: Pixels, content: Pixels, pan: Pixels) -> Option<(f32, f32)> {
    let overflow = f32::from(content) - f32::from(viewport);
    if overflow <= 0. {
        return None;
    }
    let size = (f32::from(viewport) / f32::from(content)).clamp(0., 1.);
    let offset = (0.5 - f32::from(pan) / overflow) * (1. - size);
    Some((offset, size))
}

#[cfg(test)]
mod tests {
    use super::scroll_thumb;
    use gpui::px;

    #[test]
    fn scroll_thumb_tracks_pan() {
        assert_eq!(scroll_thumb(px(100.), px(100.), px(0.)), None);

        let (_, size) = scroll_thumb(px(100.), px(200.), px(0.)).unwrap();
        assert!((size - 0.5).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), px(200.), px(-50.)).unwrap();
        assert!((offset - 0.5).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), px(200.), px(0.)).unwrap();
        assert!((offset - 0.25).abs() < 1e-6);

        let (offset, _) = scroll_thumb(px(100.), px(200.), px(50.)).unwrap();
        assert!((offset - 0.).abs() < 1e-6);
    }

    #[test]
    fn scroll_thumb_is_stable_without_overflow() {
        assert_eq!(scroll_thumb(px(0.), px(0.), px(0.)), None);
    }
}
