use crate::graphs::PlantChart;
use iced::widget::{container, row, Button, Column, Container, Row, Text};
use iced::{Element, Length};
use iced::alignment::{Horizontal, Vertical};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use crate::{Icon, Message, Tab};
use iced_aw::tab_bar::TabLabel;


#[derive(Debug, Clone)]
pub enum DetailMessage{
    Plant,
    Graph,
}

pub(crate) struct DetailPage;

impl DetailPage {
    pub fn new() -> DetailPage {
        DetailPage
    }

    pub fn update(&mut self, message: DetailMessage) {
        match message {
            DetailMessage::Plant => {println!("Plant")},
            DetailMessage::Graph => {println!("Graph")},
        }
    }
}




impl Tab for DetailPage {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Detail")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Detailpage.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {

        let text: Element<'_, DetailMessage> = Text::new("This is the Detail Page").into();
        let graphs = Row::new().push(text);

        let content: Element<'_, DetailMessage> = Container::new(Text::new("This is the Detail Page"))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();

        content.map(Message::Detail)
    }
}