use crate::graphs::{PlantChart, PlantCharts};
use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, row, Button, Column, Container, Row, Text};
use iced::{Element, Length};
use iced_aw::tab_bar::TabLabel;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use std::vec;

#[derive(Debug, Clone)]
pub enum DetailMessage {
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
            DetailMessage::Plant => {
                println!("Plant")
            }
            DetailMessage::Graph => {
                println!("Graph")
            }
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
        let chartone = PlantChart::new(
            String::from("Wasser"),
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3],
            RED,
        );
        let charttwo = PlantChart::new(
            String::from("Licht"),
            vec![0, 1, 2, 3],
            vec![3, 2, 1, 0],
            BLUE,
        );
        let charts = PlantCharts::new(vec![chartone, charttwo], DetailMessage::Graph);
        let chart = ChartWidget::new(charts);
        let row = row!(text, chart);
        let content: Element<'_, DetailMessage> = Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();
        content.map(Message::Detail)
    }
}
