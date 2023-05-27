mod graphs;
use crate::graphs::PlantCharts;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::{Custom, Palette};
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Image, Row, Text};
use iced::{
    executor, window, Application, Background, Color, Command, Element, Font, Length, Sandbox,
    Settings, Theme,
};
use iced_aw::style::TabBarStyles;
use iced_aw::{TabBar, TabLabel, Tabs};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use serde::__private::de::IdentifierDeserializer;

mod home;
use crate::home::{HomeMessage, HomePage};
mod detail;
use crate::detail::{DetailMessage, DetailPage};
mod login;
use crate::login::{LoginMessage, LoginTab, PlantBuddyRole};
mod logout;
mod management;
mod requests;
mod settings;

use crate::management::{ManagementMessage, ManagementTab};

use crate::logout::{LogoutMessage, LogoutTab};

use crate::requests::{RequestResult, TempCreationUser};
use settings::{SettingsMessage, SettingsTab, TabBarPosition};

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

const EXTERNAL_ICON_FONT: Font = iced::Font::External {
    name: "External Icons",
    bytes: include_bytes!("../fonts/MaterialIcons-Regular.ttf"),
};

enum Icon {
    User,
    Homescreen,
    Detailpage,
    CogAlt,
    Logout,
    Management,
    X,
}

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

#[derive(PartialEq)]
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

impl Application for Plantbuddy {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

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
    fn title(&self) -> String {
        String::from("Plantbuddy")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) => {
                // Check if login was successful and if so, update the user
                if let LoginMessage::Login(result) = &message {
                    if let RequestResult::Ok(role) = result {
                        self.is_logged_in = LoginState::LoggedIn;
                        self.user = role.clone();

                        // Clear the LoginTab
                        self.login_page = LoginTab::new();

                        // Update the logged in user in the management tab
                        self.management_tab.logged_in_user = role.clone();

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

pub trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

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

    fn content(&self) -> Element<'_, Self::Message>;
}
