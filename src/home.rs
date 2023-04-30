use crate::graphs::PlantChart;
use crate::ExampleMessage;
use crate::Page::Login;
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Row, Text};
use iced::{Application, Command, Element, Length, Sandbox, Settings};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

pub(crate) struct HomePage;

impl HomePage {
    pub(crate) fn view(&self) -> iced::Element<ExampleMessage> {
        // Replace this with your customized page layout
        Text::new("This is the Home Page")
            .width(Length::Fill)
            .height(Length::Fill)
            .size(50)
            .into()
    }
}
