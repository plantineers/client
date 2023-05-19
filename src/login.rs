use std::fmt;
use color_eyre::owo_colors::OwoColorize;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Row, Text, TextInput},
    Alignment, Element, Length,
};
use iced_aw::tab_bar::TabLabel;

use crate::{Icon, Message, Tab};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    ClearPressed,
    LoginPressed,
}
pub enum PlantBuddyRole {
    Admin,
    User,
    NotLoggedIn,
}
pub struct LoginPage {
    username: String,
    password: String,
    login_failed: bool
}

impl LoginPage {
    pub fn new() -> Self {
        LoginPage {
            username: String::new(),
            password: String::new(),
            login_failed: false,
        }
    }

    pub fn update(&mut self, message: LoginMessage) -> PlantBuddyRole {
        match message {
            LoginMessage::UsernameChanged(value) => {
                self.username = value;
                self.login_failed = false;
            },
            LoginMessage::PasswordChanged(value) => {
                self.password = value;
                self.login_failed = false;
            },
            LoginMessage::ClearPressed => {
                self.username = String::new();
                self.password = String::new();
                self.login_failed = false;
            }
            LoginMessage::LoginPressed => {

                let role = check_login(&self.username, &self.password);

                return match role {
                    PlantBuddyRole::Admin | PlantBuddyRole::User => {
                        println!("Login successful as {}", role);
                        self.username = String::new();
                        self.password = String::new();
                        self.login_failed = false;
                        role
                    },
                    _ => {
                        println!("Login failed");
                        self.login_failed = true;
                        role
                    }
                }


            }
        }
        PlantBuddyRole::NotLoggedIn
    }
}

impl Tab for LoginPage {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::User.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, LoginMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    TextInput::new("Username", &self.username)
                        .on_input(LoginMessage::UsernameChanged)
                        .padding(10)
                        .size(32),
                )
                .push(
                    TextInput::new("Password", &self.password)
                        .on_input(LoginMessage::PasswordChanged)
                        .padding(10)
                        .size(32)
                        .password(),
                )

                .push(
                    if self.login_failed {
                        Text::new("Login failed")
                            .horizontal_alignment(Horizontal::Center)
                    } else {
                        Text::new("")
                    }
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                Text::new("Clear").horizontal_alignment(Horizontal::Center),
                            )
                                .width(Length::Fill)
                                .on_press(LoginMessage::ClearPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Login").horizontal_alignment(Horizontal::Center),
                            )
                                .width(Length::Fill)
                                .on_press(LoginMessage::LoginPressed),
                        ),
                ),
        )
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();

        content.map(Message::Login)
    }
}

fn check_login(username: &str, password: &str) -> PlantBuddyRole {
    return if username == "admin" && password == "1234" {
        PlantBuddyRole::Admin
    }
        else if username == "user" && password == "1234" {
        PlantBuddyRole::User
    }
    else {
        PlantBuddyRole::NotLoggedIn
    };

}

impl fmt::Display for PlantBuddyRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlantBuddyRole::Admin => write!(f, "Admin"),
            PlantBuddyRole::User => write!(f, "User"),
            PlantBuddyRole::NotLoggedIn => write!(f, "LoginFailed"),
        }
    }
}