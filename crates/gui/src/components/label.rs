use gpui::*;

use crate::colorscheme::ColorScheme;

#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    muted: bool,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Label {
            text: text.into(),
            muted: false,
        }
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = if self.muted {
            ColorScheme::TextMuted
        } else {
            ColorScheme::TextSecondary
        };

        div()
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(color.into()))
            .child(self.text)
    }
}
