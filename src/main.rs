mod graphs;
use color_eyre::owo_colors::OwoColorize;
use crate::graphs::PlantChart;
use iced::widget::vertical_slider::draw;
use iced::{Element, Length, Sandbox, Settings, Font};
use iced::Background::Color;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Button, Column, Container, Row, Text, container, row};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use iced_aw::{TabBar, TabLabel, Tabs};
use iced_aw::style::TabBarStyles;
mod home;
use crate::home::{HomePage, HomeMessage};
mod detail;
use crate::detail::{DetailPage, DetailMessage};
mod login;
use crate::login::{LoginMessage, LoginPage};
mod settings;
use settings::{SettingsMessage, SettingsTab, TabBarPosition};


const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

const ICON_FONT: Font = iced::Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/MaterialIcons-Regular.ttf"),
};

enum Icon {
    User,
    Homescreen,
    Detailpage,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{ea77}',
            Icon::CogAlt => '\u{e8b8}',
            Icon::Homescreen => '\u{e88a}',
            Icon::Detailpage => '\u{e85c}',
        }
    }
}
fn main() {
    Plantbuddy::run(Settings::default()).unwrap();
}


struct Plantbuddy {
    active_tab: usize,
    home_page: HomePage,
    detail_page: DetailPage,
    login_page: LoginPage,
    settings_tab: SettingsTab,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(usize),
    Login(LoginMessage),
    Detail(DetailMessage),
    Home(HomeMessage),
   Settings(SettingsMessage),
}

impl Sandbox for Plantbuddy {
    type Message = Message;

    fn new() -> Self {
        Plantbuddy {
            active_tab: 0,
            home_page: HomePage::new(),
            detail_page: DetailPage::new(),
            login_page: LoginPage::new(),
            settings_tab: SettingsTab::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Plantbuddy")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::Login(message) => self.login_page.update(message),
            Message::Home(message) => self.home_page.update(message),
            Message::Detail(message) => self.detail_page.update(message),
            Message::Settings(message) => self.settings_tab.update(message),
        }

    }

    fn view(&self) -> Element<Self::Message> {
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

        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.home_page.tab_label(), self.home_page.view())
            .push(self.detail_page.tab_label(), self.detail_page.view())
            .push(self.login_page.tab_label(), self.login_page.view())
            .push(self.settings_tab.tab_label(), self.settings_tab.view())
            .tab_bar_style(theme)
            .icon_font(ICON_FONT)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            })
            .into()
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