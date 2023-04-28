mod detail;
mod graphs;
mod home;
mod login;

use crate::graphs::PlantChart;
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Row, Text};
use iced::{Element, Length, Sandbox, Settings};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

fn main() {
    ExampleApp::run(Settings::default()).unwrap();
}

#[derive(Default)]
struct ExampleApp {
    state: PageState,
}

#[derive(Debug, Clone)]
enum ExampleMessage {
    SwitchToHomePage,
    SwitchToLoginPage,
    SwitchToDetailPage,
}

#[derive(Debug, Clone)]
enum Page {
    Home,
    Login,
    Detail,
}

struct PageState {
    current: Page,
}

impl Default for PageState {
    fn default() -> Self {
        Self {
            current: Page::Home,
        }
    }
}

impl Sandbox for ExampleApp {
    type Message = ExampleMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Iced Example App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ExampleMessage::SwitchToHomePage => {
                self.state.current = Page::Home;
            }
            ExampleMessage::SwitchToDetailPage => {
                self.state.current = Page::Detail;
            }
            ExampleMessage::SwitchToLoginPage => {
                self.state.current = Page::Login;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let sidebar = Column::new()
            .width(Length::from(100))
            .spacing(10)
            .push(Button::new("Home").on_press(ExampleMessage::SwitchToHomePage))
            .push(Button::new("Einzelansicht").on_press(ExampleMessage::SwitchToDetailPage))
            .push(Button::new("Login").on_press(ExampleMessage::SwitchToLoginPage));

        let content = match self.state.current {
            Page::Home => home::HomePage.view(),
            Page::Detail => detail::DetailPage.view(),
            Page::Login => login::LoginPage.view(),
        };

        let main_view = row![sidebar, content];

        container(main_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
