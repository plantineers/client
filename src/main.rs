mod graphs;
mod login;

use crate::graphs::PlantChart;
use iced::widget::vertical_slider::draw;
use iced::{Element, Length, Sandbox, Settings};
use iced::widget::{button, Button, Column, Container, Row, Text, container, row};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};


fn main() {
    ExampleApp::run(Settings::default());
}

#[derive(Default)]
struct ExampleApp {
    state: PageState,
}

#[derive(Debug, Clone)]
enum ExampleMessage {
    SwitchToPage1,
    SwitchToPage2,
}

#[derive(Debug, Clone)]
enum Page {
    Page1,
    Page2,
}

struct PageState {
    current: Page,
}

impl Default for PageState {
    fn default() -> Self {
        Self { current: Page::Page1 }
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
            ExampleMessage::SwitchToPage1 => {
                self.state.current = Page::Page1;
            }
            ExampleMessage::SwitchToPage2 => {
                self.state.current = Page::Page2;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let sidebar = Column::new()
            .width(Length::from(100))
            .spacing(10)
            .push(
                Button::new("Page 1")
                    .on_press(ExampleMessage::SwitchToPage1),
            )
            .push(
                Button::new("Page 2")
                    .on_press(ExampleMessage::SwitchToPage2),
            );

        let content = match self.state.current {
            Page::Page1 => Text::new("This is Page 1"),
            Page::Page2 => Text::new("This is Page 2"),
        };

        let main_view  =  row![sidebar, content];

        container(main_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}