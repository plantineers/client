use crate::graphs::PlantChart;
use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Row, Text};
use iced::{Application, Command, Element, Length, Sandbox, Settings};
use iced_aw::TabLabel;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    Plant,
    Graph,
}

pub(crate) struct HomePage;

impl HomePage {
    pub fn new() -> HomePage {
        HomePage
    }

    pub fn update(&mut self, message: HomeMessage) {
        match message {
            HomeMessage::Plant => (),
            HomeMessage::Graph => (),
        }
    }
}

impl Tab for HomePage {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Home")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Homescreen.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, HomeMessage> = Container::new(Text::new("This is the Home Page"))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();

        content.map(Message::Home)
    }
}
