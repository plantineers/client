use crate::{Icon, Message, Tab};
use iced::Alignment::Center;
use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{Button, Column, Container, Row, Text, TextInput},
    Alignment, Element, Length,
};
use iced_aw::tab_bar::TabLabel;
use iced_aw::{style, Card, Modal};
use log::info;

/// This enum represents the various states or actions related to a logout process.
///
/// - `OpenModal`: A message to indicate the opening of a logout modal.
/// - `CloseModal`: A message to indicate the closing of a logout modal.
/// - `CancelButtonPressed`: A message to indicate that the cancel button on the logout modal was pressed.
/// - `OkButtonPressed`: A message to indicate that the confirmation button on the logout modal was pressed.
#[derive(Debug, Clone, PartialEq)]
pub enum LogoutMessage {
    OpenModal,
    CloseModal,
    CancelButtonPressed,
    OkButtonPressed,
}

/// A representation of the logout tab, showing the logout modal and handling logout related actions.
///
/// The `show_modal` boolean indicates whether the logout modal is to be shown or not.
/// The `last_message` is an option that stores the last `LogoutMessage` that was received. It's `None` by default.
#[derive(Default)]
pub struct LogoutTab {
    show_modal: bool,
    last_message: Option<LogoutMessage>,
}

impl LogoutTab {
    /// Creates a new `LogoutTab` with default values.
    ///
    /// # Returns
    /// A `LogoutTab` instance with `show_modal` set to false and `last_message` set to None.
    pub fn new() -> Self {
        Self {
            show_modal: true,
            last_message: None,
        }
    }

    /// Updates the `LogoutTab` based on the provided `LogoutMessage`.
    ///
    /// # Arguments
    ///
    /// * `message` - The `LogoutMessage` to be processed.
    pub fn update(&mut self, message: LogoutMessage) {
        match message {
            LogoutMessage::OpenModal => self.show_modal = true,
            LogoutMessage::CloseModal => self.show_modal = false,
            LogoutMessage::CancelButtonPressed => self.show_modal = false,
            LogoutMessage::OkButtonPressed => {
                info!("Logout");
                self.show_modal = false;
            }
        }
        self.last_message = Some(message)
    }
}

impl Tab for LogoutTab {
    type Message = Message;

    /// Returns the title of the `LogoutTab`.
    ///
    /// # Returns
    /// A `String` with the value "Logout".
    fn title(&self) -> String {
        String::from("Abmelden")
    }

    /// Returns the tab label of the `LogoutTab`.
    ///
    /// # Returns
    /// A `TabLabel` consisting of the logout icon and the title of the tab.
    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Logout.into(), self.title())
    }

    /// Returns the view of the `LogoutTab`.
    ///
    /// This includes the content of the tab as well as the configuration of the overall layout.
    ///
    /// # Returns
    /// A container `Element` with the view contents of the `LogoutTab`.
    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(22)
            .push(Text::new(self.title()).size(70))
            .align_items(Center)
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(16)
            .into()
    }
    /// Constructs the content of the `LogoutTab`.
    ///
    /// This includes the modal with logout options and their corresponding actions.
    ///
    /// # Returns
    /// An `Element` containing the content of the `LogoutTab`.
    fn content(&self) -> Element<'_, Self::Message> {
        let modal_content = Container::new(
            Button::new(
                Text::new("Von Plantbuddy abmelden")
                    .size(50)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center),
            )
            .height(Length::from(200))
            .width(Length::from(300))
            .on_press(LogoutMessage::OpenModal),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        let content: Element<'_, LogoutMessage> =
            Modal::new(self.show_modal, modal_content, || {
                Card::new(
                    Text::new("Abmeldung")
                        .size(50)
                        .horizontal_alignment(Horizontal::Center),
                    Text::new("Wollen Sie sich wirklich von System abmelden?").size(45),
                )
                .width(Length::from(700))
                .height(Length::from(600))
                .foot(
                    Row::new()
                        .spacing(20)
                        .padding(10)
                        .width(Length::Fill)
                        .push(
                            Button::new(
                                Text::new("Abbrechen")
                                    .horizontal_alignment(Horizontal::Center)
                                    .size(45),
                            )
                            .width(Length::Fill)
                            .on_press(LogoutMessage::CancelButtonPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Ja")
                                    .horizontal_alignment(Horizontal::Center)
                                    .size(45),
                            )
                            .style(theme::Button::Destructive)
                            .width(Length::Fill)
                            .on_press(LogoutMessage::OkButtonPressed),
                        ),
                )
                .max_width(700.0)
                .max_height(600.0)
                .on_close(LogoutMessage::CloseModal)
                .into()
            })
            .backdrop(LogoutMessage::CloseModal)
            .on_esc(LogoutMessage::CloseModal)
            .into();

        content.map(Message::Logout)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use iced::executor::Executor;
    use iced::{Command, Element, Sandbox};

    #[test]
    fn test_logout_tab_title() {
        let tab = LogoutTab::new();
        assert_eq!(tab.title(), "Abmelden");
    }

    #[test]
    fn test_logout_tab_update() {
        let mut tab = LogoutTab::new();
        tab.update(LogoutMessage::OpenModal);
        assert_eq!(tab.show_modal, true);
        assert_eq!(tab.last_message, Some(LogoutMessage::OpenModal));

        tab.update(LogoutMessage::CloseModal);
        assert_eq!(tab.show_modal, false);
        assert_eq!(tab.last_message, Some(LogoutMessage::CloseModal));

        tab.update(LogoutMessage::CancelButtonPressed);
        assert_eq!(tab.show_modal, false);
        assert_eq!(tab.last_message, Some(LogoutMessage::CancelButtonPressed));

        tab.update(LogoutMessage::OkButtonPressed);
        assert_eq!(tab.show_modal, false);
        assert_eq!(tab.last_message, Some(LogoutMessage::OkButtonPressed));
    }
}
