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
        let text: Element<'_, ManagementMessage> = Text::new("This is the Management Page").into();

        let content: Element<'_, ManagementMessage> = Container::new(text)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();
        content.map(Message::Management)
    }
}
