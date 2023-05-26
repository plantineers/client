use color_eyre::owo_colors::OwoColorize;
use iced::futures::executor::block_on;
use iced::futures::TryStreamExt;
use iced::theme::{self, Theme};
use iced::widget::{container, Image};
use iced::Alignment::Center;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Row, Text, TextInput},
    Alignment, Application, Color, Command, Element, Length,
};
use iced_aw::tab_bar::TabLabel;
use log::{info, log};
use serde::{Deserialize, Serialize};
use std::{fmt, env};

use crate::requests::{login, RequestResult};
use crate::{Icon, Message, Tab};

#[derive(Debug, Clone)]
pub enum LoginMessage {
    Login(RequestResult<PlantBuddyRole>),
    UsernameChanged(String),
    PasswordChanged(String),
    ClearPressed,
    LoginPressed,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum PlantBuddyRole {
    Admin,
    User,
    NotLoggedIn,
}

impl TryFrom<u64> for PlantBuddyRole {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlantBuddyRole::Admin),
            1 => Ok(PlantBuddyRole::User),
            2 => Ok(PlantBuddyRole::NotLoggedIn),
            _ => Err("Invalid role"),
        }
    }
}

impl Into<u64> for PlantBuddyRole {
    fn into(self) -> u64 {
        match self {
            PlantBuddyRole::Admin => 0,
            PlantBuddyRole::User => 1,
            PlantBuddyRole::NotLoggedIn => 2,
        }
    }
}
pub struct LoginTab {
    username: String,
    password: String,
    login_failed: bool,
}

impl LoginTab {
    pub fn new() -> Self {
        LoginTab {
            username: String::new(),
            password: String::new(),
            login_failed: false,
        }
    }

    pub fn update(&mut self, message: LoginMessage) -> Command<LoginMessage> {
        #[cfg(debug_assertions)]
        if env::var("USERNAME").is_ok() && env::var("PASSWORD").is_ok() {
            if self.username.is_empty() {
                self.username = env::var("USERNAME").unwrap();
            }
            if self.password.is_empty() {
                self.password = env::var("PASSWORD").unwrap();
            }
        }
        match message {
            LoginMessage::UsernameChanged(value) => {
                self.username = value;
                self.login_failed = false;
            }
            LoginMessage::PasswordChanged(value) => {
                self.password = value;
                self.login_failed = false;
            }
            LoginMessage::ClearPressed => {
                self.username = String::new();
                self.password = String::new();
                self.login_failed = false;
            }
            LoginMessage::LoginPressed => {
                return check_login(&self.username, &self.password);
            }
            LoginMessage::Login(l) => {
                println!("Login result: {:?}", l);
            }
        }
        Command::none()
    }
}

impl Tab for LoginTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Login")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::User.into(), self.title())
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(55))
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
        let image = Image::new("assets/plantbuddy.png")
            .width(Length::from(100))
            .height(Length::from(100));

        let content: Element<'_, LoginMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(image)
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
                .push(if self.login_failed {
                    Text::new("Login failed")
                        .horizontal_alignment(Horizontal::Center)
                        .style(theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0)))
                } else {
                    Text::new("")
                })
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

fn check_login(username: &str, password: &str) -> Command<LoginMessage> {
    info!("Checking login");
    Command::perform(
        login(username.to_string(), password.to_string()),
        LoginMessage::Login,
    )
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
