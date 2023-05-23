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
use rand::random;

#[derive(Debug, Clone)]
pub enum ManagementMessage {
    DeleteUser,
    UsernameChanged(String),
    PasswordChanged(String),
    RoleChanged(PlantBuddyRole),
    CreateNewUser,
    EditUserButton(User),
    EditUser,
}
#[derive(Debug, Clone)]
pub struct User {
    id: u32,
    name: String,
    password: String,
    role: PlantBuddyRole,
}

pub(crate) struct ManagementTab {
    username_input: String,
    password_input: String,
    role_input: PlantBuddyRole,
    users: Vec<User>,
    error_message: String,
    editing_user: Option<User>,
}

impl ManagementTab {
    pub fn new() -> ManagementTab {
        ManagementTab {
            username_input: String::new(),
            password_input: String::new(),
            role_input: PlantBuddyRole::User,
            users: Vec::new(),
            error_message: String::new(),
            editing_user: None,
        }
    }

    pub fn update(&mut self, message: ManagementMessage) {
        match message {
            ManagementMessage::UsernameChanged(username) => {
                self.username_input = username;
                self.error_message = String::new();
            }
            ManagementMessage::PasswordChanged(password) => {
                self.password_input = password;
                self.error_message = String::new();
            }
            ManagementMessage::CreateNewUser => {
                //Check if username or password is empty
                if self.editing_user.is_none() {
                    if self.username_input.is_empty() || self.password_input.is_empty() {
                        self.error_message = String::from("Username or password is empty");
                        return;
                    }

                    // TODO: Await backend answer
                    self.users.push(User {
                        id: random(),
                        name: self.username_input.clone(),
                        password: self.password_input.clone(),
                        role: self.role_input.clone(),
                    });
                    self.username_input.clear();
                    self.password_input.clear();
                } else {
                    if self.username_input.is_empty() || self.password_input.is_empty() {
                        self.error_message = String::from("Username or password is empty");
                        return;
                    }
                    let mut users = Vec::new();
                    for mut user in self.users.clone() {
                        if user.id == self.editing_user.clone().unwrap().id {
                            user.name = self.username_input.clone();
                            user.password = self.password_input.clone();
                            user.role = self.role_input;
                        }
                        users.push(user);
                    }

                    self.username_input.clear();
                    self.password_input.clear();
                    self.users = users;
                    self.editing_user = None;
                }
            }
            ManagementMessage::DeleteUser => {
                self.error_message = String::new();
                // TODO: Await backend answer
                // Check If user to delete is the current user
            }
            ManagementMessage::RoleChanged(role) => {
                self.role_input = role;
            }
            ManagementMessage::EditUserButton(user) => {
                self.error_message = String::new();
                self.editing_user = Some(user.clone());

                self.role_input = user.role.clone();
                self.username_input = user.name.clone();
                self.password_input = user.password.clone();
            }
            ManagementMessage::EditUser => {
                self.error_message = String::new();
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
            .width(Length::Fill)
            .height(Length::Fill)
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
        let mut user_list = Column::new().width(Length::from(1100)).height(Length::Fill);
        user_list = user_list.push(
            Row::new()
                .spacing(20)
                .push(
                    Container::new(Text::new("#"))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::from(50)),
                )
                .push(
                    Container::new(Text::new("Username"))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::from(300)),
                )
                .push(
                    Container::new(Text::new("Password"))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::from(200)),
                )
                .push(
                    Container::new(Text::new("Role"))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::from(200)),
                )
                .push(Container::new(Text::new("")).width(Length::from(100)))
                .push(Container::new(Text::new("")).width(Length::from(100))),
        );
        for (i, user) in self.users.iter().enumerate() {
            let row = Row::new()
                .spacing(20)
                .push(
                    Container::new(Text::new(user.id.clone().to_string()))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::from(50)),
                )
                .push(
                    Container::new(Text::new(&user.name))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::from(300)),
                )
                .push(
                    Container::new(Text::new(&user.password))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::from(200)),
                )
                .push(
                    Container::new(Text::new(match &user.role {
                        PlantBuddyRole::User => "User",
                        PlantBuddyRole::Admin => "Admin",
                        _ => "This should not happen",
                    }))
                    .center_x()
                    .center_y()
                    .padding(10)
                    .width(Length::from(200)),
                )
                .push(Container::new(
                    Button::new(Text::new("Edit"))
                        .on_press(ManagementMessage::EditUserButton(user.clone()))
                        .width(Length::from(100)),
                ))
                .push(
                    Container::new(
                        Button::new(Text::new("Delete")).on_press(ManagementMessage::DeleteUser),
                    )
                    .width(Length::from(100)),
                );

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
            .spacing(20)
            .push(
                TextInput::new("Username", &self.username_input)
                    .on_input(ManagementMessage::UsernameChanged),
            )
            .push(
                TextInput::new("Password", &self.password_input)
                    .on_input(ManagementMessage::PasswordChanged),
            )
            .push(radio_column)
            .push(
                Button::new(match self.editing_user {
                    Some(_) => "Edit User",
                    None => "Create User",
                })
                .on_press(ManagementMessage::CreateNewUser),
            );

        let content: Element<'_, ManagementMessage> = Column::new()
            .push(scrollable)
            .push(if self.error_message != String::new() {
                Text::new(&self.error_message).style(Color::from_rgb(1.0, 0.0, 0.0))
            } else {
                Text::new("")
            })
            .push(input_row)
            .into();

        content.map(Message::Management)
    }
}
