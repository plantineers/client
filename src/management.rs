use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, scrollable, Button, Column, Container, Row, Text};
use iced::Alignment::Center;
use iced::{Application, Command, Element, Length, Sandbox, Settings};
use iced_aw::TabLabel;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

#[derive(Debug, Clone)]
pub enum ManagementMessage {
    AddUser,
    DeleteUser,
}

pub(crate) struct ManagementTab;

impl ManagementTab {
    pub fn new() -> ManagementTab {
        ManagementTab
    }

    pub fn update(&mut self, message: ManagementMessage) {
        match message {
            ManagementMessage::AddUser => (),
            ManagementMessage::DeleteUser => (),
        }
    }
}

impl Tab for ManagementTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Management")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Management.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let mut column = Column::new();

        for i in 0..30 {
            let text = Text::new(format!("This is the Management Page {}", i));
            column = column.push(text);
        }

        let scrollable: Element<'_, ManagementMessage> = scrollable::Scrollable::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        scrollable.map(Message::Management)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(55))
            .width(Length::from(600))
            .height(Length::from(500))
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
}
