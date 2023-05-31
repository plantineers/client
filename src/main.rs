//! The Plantbuddy struct is the main struct of the application.
//! Plantbuddy is a desktop application for managing plants. It allows users to view and edit plant data,
//! manage users, and customize settings. The application is built using the Rust programming language
//! and the Iced GUI library. The main.rs file contains the entry point for the application and defines
//! the Plantbuddy struct, which holds the application state and handles messages and updates. The struct
//! implements the Application trait from the Iced library, which defines the behavior of the application.
//! The file also includes several modules that define the different pages and components of the application,
//! such as the home page, detail page, login page, and management page. Each module defines a struct that
//! implements the Tab trait, which defines the behavior of a tab in the application. The file also includes
//!  everal utility functions and constants, such as the Icon enum, which defines the icons used in the
//! application, and the EXTERNAL_ICON_FONT constant, which defines the font used for the icons.

mod detail;
mod graphs;
use std::sync::OnceLock;

mod home;
mod login;
mod logout;
mod management;
mod requests;
mod settings;

use crate::graphs::PlantCharts;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::{Custom, Palette};
use iced::widget::container::{Appearance, StyleSheet};
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Image, Row, Text};
use iced::{
    executor, window, Application, Background, Color, Command, Element, Font, Length, Sandbox,
    Settings, Theme,
};
use iced_aw::style::TabBarStyles;
use iced_aw::{TabBar, TabLabel, Tabs};
use log::info;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use requests::ApiClient;
use serde::__private::de::IdentifierDeserializer;

use crate::detail::{DetailMessage, DetailPage, Sensortypes};
use crate::home::{HomeMessage, HomePage};
use crate::login::{LoginMessage, LoginTab, PlantBuddyRole};
use crate::logout::{LogoutMessage, LogoutTab};
use crate::management::{ManagementMessage, ManagementTab};
use crate::requests::{RequestResult, TempCreationUser};
use settings::{SettingsMessage, SettingsTab, TabBarPosition};

/// The font used for the icons.
const EXTERNAL_ICON_FONT: Font = iced::Font::External {
    name: "External Icons",
    bytes: include_bytes!("../fonts/MaterialIcons-Regular.ttf"),
};
const TEXT_SIZE: u16 = 30;
/// The Icons used in the application.
enum Icon {
    User,
    Homescreen,
    Detailpage,
    CogAlt,
    Logout,
    Management,
    X,
}
pub struct MyStylesheet;

static API_CLIENT: OnceLock<ApiClient> = OnceLock::new();

impl StyleSheet for MyStylesheet {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::WHITE)),
            border_radius: 0.2,
            border_width: 0.2,
            border_color: Color::BLACK,
        }
    }
}
/// Implementation of the from Icon to char conversion.
impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{ea77}',
            Icon::CogAlt => '\u{e8b8}',
            Icon::Homescreen => '\u{e88a}',
            Icon::Detailpage => '\u{e85c}',
            Icon::Logout => '\u{e9ba}',
            Icon::Management => '\u{f02e}',
            Icon::X => '\u{e5cd}',
        }
    }
}

/// The main function of the application.
fn main() {
    env_logger::init();
    Plantbuddy::run(Settings {
        antialiasing: false,
        window: window::Settings {
            size: (1920, 1080),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
    .unwrap();
}

/// The LoginState enum is used to keep track of the login state of the application.
#[derive(PartialEq, Debug)]
enum LoginState {
    NotLoggedIn,
    LoggedIn,
}

struct Plantbuddy {
    is_logged_in: LoginState,
    active_tab: usize,
    home_page: HomePage,
    detail_page: DetailPage,
    login_page: LoginTab,
    settings_tab: SettingsTab,
    logout_tab: LogoutTab,
    management_tab: ManagementTab,
    user: TempCreationUser,
}

/// The Message enum is used to handle messages from the different tabs.
#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(usize),
    Login(LoginMessage),
    Detail(DetailMessage),
    Home(HomeMessage),
    Settings(SettingsMessage),
    Logout(LogoutMessage),
    Management(ManagementMessage),
}

/// implementation of the Application trait for the Plantbuddy struct.
impl Application for Plantbuddy {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    /// Constructs a new instance of the `Plantbuddy` application.
    /// # Returns
    /// A tuple containing the newly created `Plantbuddy` application and an initial command of type `Message`.
    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        (
            Plantbuddy {
                is_logged_in: LoginState::NotLoggedIn,
                active_tab: 0,
                home_page: HomePage::new(),
                detail_page: DetailPage::new(),
                login_page: LoginTab::new(),
                settings_tab: SettingsTab::new(),
                logout_tab: LogoutTab::new(),
                management_tab: ManagementTab::new(),
                user: TempCreationUser::default(),
            },
            Command::none(),
        )
    }

    /// Returns the title of the application.
    fn title(&self) -> String {
        String::from("Plantbuddy")
    }

    /// Updates the state of the `Plantbuddy` application.
    /// # Arguments
    /// * `message` - The message to update the state with.
    /// # Returns
    /// A command of type `Message`.
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) => {
                // Check if login was successful and if so, update the user
                if let LoginMessage::Login(result) = &message {
                    if let RequestResult::Ok(user) = result {
                        self.is_logged_in = LoginState::LoggedIn;
                        self.user = user.clone();

                        // Clear the LoginTab

                        self.login_page = LoginTab::new();

                        // Update the logged in user in the management tab
                        self.management_tab.logged_in_user = user.clone();

                        self.home_page
                            .update(HomeMessage::SwitchGraph(Sensortypes::Feuchtigkeit));
                        self.detail_page.update(DetailMessage::Load);
                        // Get all users from the server and update the management tab
                        return self
                            .management_tab
                            .update(ManagementMessage::GetUsersPressed)
                            .map(Message::Management);
                    }
                }
                return self.login_page.update(message).map(Message::Login);
            }
            Message::Home(message) => self.home_page.update(message),
            Message::Detail(message) => self.detail_page.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
            Message::Logout(message) => {
                self.logout_tab.update(message.clone());
                // If the logout is approved, log out and return to the login screen
                if let LogoutMessage::OkButtonPressed = message {
                    self.is_logged_in = LoginState::NotLoggedIn;
                    self.user = TempCreationUser::default()
                }
            }
            Message::Management(message) => {
                return self.management_tab.update(message).map(Message::Management);
            }
        }
        Command::none()
    }

    /// Returns the view of the `Plantbuddy` application.
    fn view(&self) -> Element<Self::Message> {
        if self.is_logged_in == LoginState::LoggedIn {
            let position = self
                .settings_tab
                .settings()
                .tab_bar_position
                .unwrap_or_default();
            let theme = self
                .settings_tab
                .settings()
                .tab_bar_theme
                .unwrap_or_default();

            let mut tabs = Tabs::new(self.active_tab, Message::TabSelected)
                .push(self.home_page.tab_label(), self.home_page.view())
                .push(self.detail_page.tab_label(), self.detail_page.view())
                .push(self.settings_tab.tab_label(), self.settings_tab.view())
                .tab_bar_style(theme)
                .icon_font(EXTERNAL_ICON_FONT);

            if let PlantBuddyRole::Admin = PlantBuddyRole::try_from(self.user.role.clone()).unwrap()
            {
                tabs = tabs.push(self.management_tab.tab_label(), self.management_tab.view());
            }

            tabs = tabs.push(self.logout_tab.tab_label(), self.logout_tab.view());

            tabs.tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            })
            .into()
        } else {
            self.login_page.view()
        }
    }

    /// Returns the custom theme of the `Plantbuddy` application.
    fn theme(&self) -> Theme {
        let palette = Palette {
            background: Color::from_rgb(5.0 / 255.0, 59.0 / 255.0, 6.0 / 255.0),
            text: Color::from_rgb(252.0 / 255.0, 247.0 / 255.0, 1.0),
            primary: Color::from_rgb(0.11, 0.42, 0.87),
            success: Color::from_rgb(13.0 / 255.0, 171.0 / 255.0, 118.0 / 255.0),
            danger: Color::from_rgb(214.0 / 255.0, 73.0 / 255.0, 51.0 / 255.0),
        };
        let custom = Custom::new(palette);
        Theme::Custom(Box::new(custom))
    }
}

/// A trait representing a tab in the `Plantbuddy` application.
/// # Types
/// - `Message`: The type of message that this tab will use to communicate.
pub trait Tab {
    type Message;

    /// Returns the title of the tab.
    fn title(&self) -> String;

    /// Returns the label of the tab.
    fn tab_label(&self) -> TabLabel;

    /// Updates the state of the tab.
    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(32))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(16)
            .into()
    }

    /// Returns the content of the tab.
    fn content(&self) -> Element<'_, Self::Message>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_plantbuddy() {
        let (plantbuddy, cmd) = Plantbuddy::new(());
        assert_eq!(plantbuddy.is_logged_in, LoginState::NotLoggedIn);
        assert_eq!(plantbuddy.active_tab, 0);
        assert_eq!(plantbuddy.active_tab, 0);
    }

    #[test]
    fn test_plantbuddy_title() {
        let (plantbuddy, _) = Plantbuddy::new(());
        assert_eq!(plantbuddy.title(), "Plantbuddy");
    }

    #[test]
    fn test_login_state() {
        let (mut plantbuddy, _) = Plantbuddy::new(());
        let user = TempCreationUser {
            name: "test".to_string(),
            password: "test".to_string(),
            role: PlantBuddyRole::User.into(),
        };

        assert_eq!(plantbuddy.is_logged_in, LoginState::NotLoggedIn);
        let _ = plantbuddy.update(Message::Login(LoginMessage::Login(RequestResult::Ok(
            user.clone(),
        ))));
        assert_eq!(plantbuddy.is_logged_in, LoginState::LoggedIn);
    }

    #[test]
    fn test_active_tab() {
        let (mut plantbuddy, _) = Plantbuddy::new(());
        assert_eq!(plantbuddy.active_tab, 0);
        plantbuddy.update(Message::TabSelected(2));
        assert_eq!(plantbuddy.active_tab, 2);
    }

    #[test]
    fn test_icon_conversion() {
        assert_eq!(char::from(Icon::User), '\u{ea77}');
        assert_eq!(char::from(Icon::Homescreen), '\u{e88a}');
    }
}
