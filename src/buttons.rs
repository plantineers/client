use iced::theme::palette::Danger;
use iced::widget::button;
use iced::widget::button::{Appearance, StyleSheet};
use iced::widget::canvas::Style;
use iced::Theme;
use iced_core::{Background, Color, Vector};
use std::vec;

#[derive(Debug, Clone, Default)]
pub(crate) struct CustomButtonStyle {
    pub(crate) background: Color,
    pub(crate) text_color: Color,
    pub(crate) border_color: Color,
}
#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
    Secondary,
}
impl StyleSheet for CustomButtonStyle {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(Color::from_rgb(
                    5.0 / 255.0,
                    59.0 / 255.0,
                    6.0 / 255.0,
                ))),
                border_radius: 12.0,
                border_width: 0.0,
                border_color: self.border_color,
                text_color: self.text_color,
            },
            _ => Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(Color::from_rgb(
                    5.0 / 255.0,
                    59.0 / 255.0,
                    6.0 / 255.0,
                ))),
                border_radius: 12.0,
                border_width: 0.0,
                border_color: self.border_color,
                text_color: self.text_color,
            },
        }
    }
}
