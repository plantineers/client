mod detail;
mod graphs;
mod home;
mod login;

use crate::graphs::PlantChart;
use iced::widget::vertical_slider::draw;
use iced::{Element, Length, Sandbox, Settings};
use iced::widget::{button, Button, Column, Container, Row, Text, container, row};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use iced_aw::{TabBar, TabLabel};

fn main() {
    Plantbuddy::run(Settings::default()).unwrap();
}


#[derive(Default)]
struct Plantbuddy {
    active_tab: usize,
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(usize),
}

impl Sandbox for Plantbuddy {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Iced Example App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TabSelected(index) => {
                self.active_tab = index;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let tab_bar = TabBar::new(self.active_tab, Message::TabSelected)
            .push(TabLabel::Text(String::from("Home")))
            .push(TabLabel::Text(String::from("Einzelansicht")))
            .push(TabLabel::Text(String::from("Login")));

        let content = match self.active_tab {
            0 => home::HomePage.view(),
            1 => detail::DetailPage.view(),
            2 => login::LoginPage.view(),
            _ => Element::new(Text::new("This tab doesn't exist"))
        };

        let main_view = Row::new()
            .push(tab_bar)
            .push(content);

        Container::new(main_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}