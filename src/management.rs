use crate::{Icon, Message, Tab};

use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, scrollable, Rule};
use iced::Alignment::Center;

use crate::login::PlantBuddyRole;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{radio, Button, Column, Container, Row, Text, TextInput},
    Alignment, Color, Element, Length,
};
use iced::{Application, Command, Sandbox, Settings};
use iced_aw::TabLabel;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

#[derive(Debug, Clone)]
pub enum ManagementMessage {
    DeleteUser,
    UsernameChanged(String),
    PasswordChanged(String),
    RoleChanged(PlantBuddyRole),
    CreateNewUser,
}
pub struct User {
    username: String,
    password: String,
    role: PlantBuddyRole,
}

pub(crate) struct ManagementTab {
    username_input: String,
    password_input: String,
    role_input: PlantBuddyRole,
    users: Vec<User>,
}

impl ManagementTab {
    pub fn new() -> ManagementTab {
        ManagementTab {
            username_input: String::new(),
            password_input: String::new(),
            role_input: PlantBuddyRole::User,
            users: Vec::new(),
        }
    }

    pub fn update(&mut self, message: ManagementMessage) {
        match message {
            ManagementMessage::UsernameChanged(username) => {
                self.username_input = username;
            }
            ManagementMessage::PasswordChanged(password) => {
                self.password_input = password;
            }
            ManagementMessage::CreateNewUser => {
                self.users.push(User {
                    username: self.username_input.clone(),
                    password: self.password_input.clone(),
                    role: self.role_input.clone(),
                });
                self.username_input.clear();
                self.password_input.clear();
            }
            ManagementMessage::DeleteUser => {
                // Here you would need to determine which user to delete
                // This is currently unimplemented
            }
            ManagementMessage::RoleChanged(role) => {
                self.role_input = role;
            }
        }
    }
}

impl Tab for ManagementTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Management")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Management.into(), self.title())
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(55))
            .width(Length::from(600))
            .height(Length::from(500))
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

    fn content(&self) -> Element<'_, Self::Message> {
        let mut user_list = Column::new();
        user_list = user_list.push(
            Row::new()
                .push(
                    Container::new(Text::new("#"))
                        .padding(10)
                        .width(Length::from(50)),
                )
                .push(
                    Container::new(Text::new("Username"))
                        .padding(10)
                        .width(Length::from(150)),
                )
                .push(
                    Container::new(Text::new("Password"))
                        .padding(10)
                        .width(Length::from(150)),
                )
                .push(
                    Container::new(Text::new("Role"))
                        .padding(10)
                        .width(Length::from(150)),
                )
                .push(Container::new(Text::new("")))
                .push(Container::new(Text::new(""))),
        );
        for (i, user) in self.users.iter().enumerate() {
            let row = Row::new()
                .push(
                    Container::new(Text::new(i.clone().to_string()))
                        .padding(10)
                        .width(Length::from(50)),
                )
                .push(
                    Container::new(Text::new(&user.username))
                        .padding(10)
                        .width(Length::from(150)),
                )
                .push(
                    Container::new(Text::new(&user.password))
                        .padding(10)
                        .width(Length::from(150)),
                )
                .push(
                    Container::new(Text::new(match &user.role {
                        PlantBuddyRole::User => "User",
                        PlantBuddyRole::Admin => "Admin",
                        _ => "This should not happen",
                    }))
                    .padding(10)
                    .width(Length::from(50)),
                )
                .push(Button::new(Text::new("Edit")).on_press(ManagementMessage::CreateNewUser))
                .push(Button::new(Text::new("Delete")).on_press(ManagementMessage::DeleteUser));

            user_list = user_list.push(row).push(Rule::horizontal(10));
        }

        let scrollable = scrollable::Scrollable::new(user_list)
            .width(Length::Fill)
            .height(Length::Fill);

        let radio_column = Column::new()
            .padding(20)
            .spacing(10)
            .push(radio(
                "User",
                PlantBuddyRole::User,
                Some(self.role_input),
                ManagementMessage::RoleChanged,
            ))
            .push(radio(
                "Admin",
                PlantBuddyRole::Admin,
                Some(self.role_input),
                ManagementMessage::RoleChanged,
            ));

        let input_row = Row::new()
            .align_items(Center)
            .push(
                TextInput::new("Username", &self.username_input)
                    .on_input(ManagementMessage::UsernameChanged),
            )
            .push(
                TextInput::new("Password", &self.password_input)
                    .on_input(ManagementMessage::PasswordChanged),
            )
            .push(radio_column)
            .push(Button::new("Create User").on_press(ManagementMessage::CreateNewUser));

        let content: Element<'_, ManagementMessage> =
            Column::new().push(scrollable).push(input_row).into();

        content.map(Message::Management)
    }
}
