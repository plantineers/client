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
use iced::{application, color};
use iced_aw::tab_bar::TabLabel;
use log::{info, log};
use serde::{Deserialize, Serialize};
use std::{fmt, env};

use crate::requests::{login, RequestResult, TempCreationUser};
use crate::{Icon, Message, Tab};
/// Represents a message that can be sent to the `LoginTab` to update its state.
#[derive(Debug, Clone)]
pub enum LoginMessage {
    Login(RequestResult<TempCreationUser>),
    UsernameChanged(String),
    PasswordChanged(String),
    ClearPressed,
    LoginPressed,
}

/// Represents the role of a user in the PlantBuddy application.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Default)]
pub enum PlantBuddyRole {
    Admin,
    User,
    #[default]
    NotLoggedIn,
}

/// This impl provides a conversion from `PlantBuddyRole` to `u64`.
impl Into<u64> for PlantBuddyRole {
    fn into(self) -> u64 {
        match self {
            PlantBuddyRole::Admin => 0,
            PlantBuddyRole::User => 1,
            PlantBuddyRole::NotLoggedIn => 2,
        }
    }
}
/// This impl provides a conversion from `u64` to `PlantBuddyRole`.
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
/// This impl provides the functionality to display `PlantBuddyRole` as a string.
impl fmt::Display for PlantBuddyRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlantBuddyRole::Admin => write!(f, "Admin"),
            PlantBuddyRole::User => write!(f, "User"),
            PlantBuddyRole::NotLoggedIn => write!(f, "LoginFailed"),
        }
    }
}

/// Struct `LoginTab` encapsulates the information needed for the login tab.
pub struct LoginTab {
    username: String,
    password: String,
    login_failed: bool,
    last_error_massage: String,
}

/// This impl block provides methods associated with `LoginTab`.
impl LoginTab {
    /// Creates a new `LoginTab`.
    pub fn new() -> Self {
        info!("LoginTab created");
        LoginTab {
            username: String::new(),
            password: String::new(),
            login_failed: false,
            last_error_massage: String::new(),
        }
    }
    /// Updates the state of the `LoginTab` based on the given `LoginMessage`.
    /// Returns a `Command` that can be used to perform asynchronous tasks.
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
                self.last_error_massage = String::new();
                self.login_failed = false;
            }
            LoginMessage::LoginPressed => {
                // Check if username or password is empty and display error message if so
                if self.username.is_empty() || self.password.is_empty() {
                    info!("Username or password is empty");
                    self.login_failed = true;
                    self.last_error_massage = "Nutzername oder Passwort ist leer".to_string();
                    return Command::none();
                }
                return check_login(&self.username, &self.password);
            }
            LoginMessage::Login(result) => match result {
                Ok(user) => {
                    info!("Login successful");
                    info!("User: {:?}", user);
                    self.login_failed = false;
                }
                Err(error) => {
                    info!("Login failed");
                    info!("Error: {:?}", error);
                    self.login_failed = true;
                    self.last_error_massage = "Server-Fehler".to_string();
                }
            },
        }
        Command::none()
    }
}

/// This impl provides methods needed for the `LoginTab` to behave as a Tab.
impl Tab for LoginTab {
    type Message = Message;

    /// Returns the title of the `LoginTab`.
    fn title(&self) -> String {
        String::from("Login")
    }

    /// Returns the label of the `LoginTab`.
    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::User.into(), self.title())
    }

    /// Returns the view of the `LoginTab`.
    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
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

    /// Returns the content of the `LoginTab`.
    fn content(&self) -> Element<'_, Self::Message> {
        let image = Image::new("assets/plantbuddy.png")
            .width(Length::from(200))
            .height(Length::from(200));

        let content: Element<'_, LoginMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .height(Length::Fill)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(image)
                .push(
                    TextInput::new("Nutzername", &self.username)
                        .on_input(LoginMessage::UsernameChanged)
                        .padding(10)
                        .size(32),
                )
                .push(
                    TextInput::new("Passwort", &self.password)
                        .on_input(LoginMessage::PasswordChanged)
                        .on_submit(LoginMessage::LoginPressed)
                        .padding(10)
                        .size(32)
                        .password(),
                )
                .push(if self.login_failed {
                    Text::new(format!("Login failed: {}", self.last_error_massage))
                        .size(32)
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
                                Text::new("Clear")
                                    .horizontal_alignment(Horizontal::Center)
                                    .size(32),
                            )
                            .width(Length::Fill)
                            .height(Length::from(50))
                            .on_press(LoginMessage::ClearPressed),
                        )
                        .push(
                            Button::new(
                                Text::new("Login")
                                    .horizontal_alignment(Horizontal::Center)
                                    .size(32),
                            )
                            .height(Length::from(50))
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

/// This function performs the async login.
/// /// It expects a username and password as input parameters.
/// Returns a `Result` containing the `User` if the login was successful and an Error if not.
fn check_login(username: &str, password: &str) -> Command<LoginMessage> {
    info!("Checking login");
    Command::perform(
        login(username.to_string(), password.to_string()),
        LoginMessage::Login,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_tab_new() {
        let login_tab = LoginTab::new();
        assert_eq!(login_tab.username, "");
        assert_eq!(login_tab.password, "");
        assert_eq!(login_tab.login_failed, false);
        assert_eq!(login_tab.last_error_massage, "");
    }

    #[test]
    fn test_login_tab_update_username_changed() {
        let mut login_tab = LoginTab::new();
        let message = LoginMessage::UsernameChanged("test".to_string());
        login_tab.update(message);
        assert_eq!(login_tab.username, "test");
        assert_eq!(login_tab.login_failed, false);
    }

    #[test]
    fn test_login_tab_update_password_changed() {
        let mut login_tab = LoginTab::new();
        let message = LoginMessage::PasswordChanged("test".to_string());
        login_tab.update(message);
        assert_eq!(login_tab.password, "test");
        assert_eq!(login_tab.login_failed, false);
    }

    #[test]
    fn test_login_tab_update_clear_pressed() {
        let mut login_tab = LoginTab::new();
        login_tab.username = "test".to_string();
        login_tab.password = "test".to_string();
        login_tab.login_failed = true;
        login_tab.last_error_massage = "test".to_string();
        let message = LoginMessage::ClearPressed;
        login_tab.update(message);
        assert_eq!(login_tab.username, "");
        assert_eq!(login_tab.password, "");
        assert_eq!(login_tab.login_failed, false);
        assert_eq!(login_tab.last_error_massage, "");
    }

    #[test]
    fn test_login_tab_update_login_pressed_empty_username() {
        let mut login_tab = LoginTab::new();
        login_tab.password = "test".to_string();
        let message = LoginMessage::LoginPressed;
        let command = login_tab.update(message);
        assert_eq!(login_tab.login_failed, true);
        assert_eq!(
            login_tab.last_error_massage,
            "Nutzername oder Passwort ist leer"
        );
    }

    #[test]
    fn test_login_tab_update_login_pressed_empty_password() {
        let mut login_tab = LoginTab::new();
        login_tab.username = "test".to_string();
        let message = LoginMessage::LoginPressed;
        let command = login_tab.update(message);
        assert_eq!(login_tab.login_failed, true);
        assert_eq!(
            login_tab.last_error_massage,
            "Nutzername oder Passwort ist leer"
        );
    }

    #[test]
    fn test_login_tab_update_login_pressed_failed() {
        let mut login_tab = LoginTab::new();
        login_tab.username = "test".to_string();
        login_tab.password = "test".to_string();
        let message = LoginMessage::Login(RequestResult::Err("test".to_string()));
        let command = login_tab.update(message);
        assert_eq!(login_tab.login_failed, true);
        assert_eq!(login_tab.last_error_massage, "Server-Fehler");
    }

    #[test]
    fn test_plant_buddy_role_into() {
        assert_eq!(Into::<u64>::into(PlantBuddyRole::Admin), 0);
        assert_eq!(Into::<u64>::into(PlantBuddyRole::User), 1);
        assert_eq!(Into::<u64>::into(PlantBuddyRole::NotLoggedIn), 2);
    }

    #[test]
    fn test_plant_buddy_role_try_from() {
        assert_eq!(PlantBuddyRole::try_from(0), Ok(PlantBuddyRole::Admin));
        assert_eq!(PlantBuddyRole::try_from(1), Ok(PlantBuddyRole::User));
        assert_eq!(PlantBuddyRole::try_from(2), Ok(PlantBuddyRole::NotLoggedIn));
        assert_eq!(PlantBuddyRole::try_from(3), Err("Invalid role"));
    }

    #[test]
    fn test_plant_buddy_role_fmt_display() {
        assert_eq!(format!("{}", PlantBuddyRole::Admin), "Admin");
        assert_eq!(format!("{}", PlantBuddyRole::User), "User");
        assert_eq!(format!("{}", PlantBuddyRole::NotLoggedIn), "LoginFailed");
    }
}
