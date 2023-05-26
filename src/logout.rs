use crate::{Icon, Message, Tab};
use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{Button, Column, Container, Row, Text, TextInput},
    Alignment, Color, Element, Length, Theme,
};
use iced_aw::tab_bar::TabLabel;
use iced_aw::{style, Card, Modal};
use log::info;

#[derive(Debug, Clone)]
pub enum LogoutMessage {
    OpenModal,
    CloseModal,
    CancelButtonPressed,
    OkButtonPressed,
}

#[derive(Default)]
pub struct LogoutTab {
    show_modal: bool,
    last_message: Option<LogoutMessage>,
}

impl LogoutTab {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
    pub fn update(&mut self, message: LogoutMessage) {
        match message {
            LogoutMessage::OpenModal => self.show_modal = true,
            LogoutMessage::CloseModal => self.show_modal = false,
            LogoutMessage::CancelButtonPressed => self.show_modal = false,
            LogoutMessage::OkButtonPressed => {
                info!("Logout");
                logout();
                self.show_modal = false;
            }
        }
        self.last_message = Some(message)
    }
}

impl Tab for LogoutTab {
    type Message = Message;
    fn title(&self) -> String {
        String::from("Logout")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Logout.into(), self.title())
    }
    fn content(&self) -> Element<'_, Self::Message> {
        let modal_content = Container::new(
            Button::new(
                Text::new("Von Plantbuddy abmelden")
                    .size(30)
                    .height(Length::Fill)
                    .width(Length::Fill),
            )
            .height(Length::from(100))
            .width(Length::from(200))
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
                        .size(40)
                        .horizontal_alignment(Horizontal::Center),
                    Text::new("Wollen Sie sich wirklich von System abmelden?").size(30),
                )
                .foot(
                    Row::new()
                        .spacing(20)
                        .padding(10)
                        .width(Length::Fill)
                        .push(
                            Button::new(
                                Text::new("Abbrechen")
                                    .style(Color::from_rgb(0.11, 0.42, 0.87))
                                    .horizontal_alignment(Horizontal::Center)
                                    .size(30),
                            )
                            .width(Length::Fill)
                            .style(theme::Button::Primary)
                            .on_press(LogoutMessage::CancelButtonPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Ja")
                                    .horizontal_alignment(Horizontal::Center)
                                    .size(30),
                            )
                            .style(theme::Button::Destructive)
                            .width(Length::Fill)
                            .on_press(LogoutMessage::OkButtonPressed),
                        ),
                )
                .max_width(500.0)
                .max_height(400.0)
                .on_close(LogoutMessage::CloseModal)
                .into()
            })
            .backdrop(LogoutMessage::CloseModal)
            .on_esc(LogoutMessage::CloseModal)
            .into();

        content.map(Message::Logout)
    }
}
fn logout() {
    info!("user logged out");
}
