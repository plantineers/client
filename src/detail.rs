use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{get_graphs, get_plant_details, GraphData};
use crate::{Icon, Message, Tab};
use color_eyre::owo_colors::OwoColorize;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, row, Button, Column, Container, Row, Text, TextInput};
use iced::{Application, Element, Length};
use iced_aw::tab_bar::TabLabel;
use iced_core::keyboard::KeyCode::C;
use iced_core::Alignment::Center;
use itertools::Itertools;
use log::info;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use std::vec;

#[derive(Debug, Clone)]
pub struct DetailPlant {
    pub id: String,
    pub name: String,
    pub description: String,
    pub charts: PlantCharts<DetailMessage>,
}

impl DetailPlant {
    pub fn new(&mut self, id: String, graph_data: Vec<GraphData>) -> Self {
        let charts = self.create_charts(DetailMessage::Load, graph_data, Sensortypes::Feuchtigkeit);
        let plant_data = get_plant_details(id).unwrap();
        let plant = DetailPlant {
            id: plant_data.id.to_string(),
            name: plant_data.id.to_string(),
            description: plant_data.description,
            charts,
        };
        plant
    }
    pub fn create_charts(
        &self,
        message: DetailMessage,
        graph_data: Vec<GraphData>,
        sensor: Sensortypes,
    ) -> PlantCharts<DetailMessage> {
        let mut charts = Vec::new();
        for data in &graph_data {
            let chart = PlantChart::new(
                format!("{:?}", sensor),
                (0..data.timestamps.len() as i32).collect_vec(),
                data.values.clone(),
                sensor.get_color(),
            );
            charts.push(chart);
        }
        let mut plant_charts = PlantCharts::new(charts, message);
        plant_charts
    }
    pub fn update_charts(
        &mut self,
        message: DetailMessage,
        graph_data: Vec<GraphData>,
        sensor: Sensortypes,
    ) {
        self.charts = self.create_charts(message, graph_data, sensor);
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DetailMessage {
    Load,
    PlantData(String),
    Loaded,
    SwitchGraph(Sensortypes),
    Search(String),
}

pub(crate) struct DetailPage {
    pub plant: DetailPlant,
    pub message: DetailMessage,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sensortypes {
    Feuchtigkeit,
    Luftfeuchtigkeit,
    Temperatur,
}
impl Sensortypes {
    pub fn get_name(&self) -> String {
        match self {
            Sensortypes::Feuchtigkeit => String::from("soil-moisture"),
            Sensortypes::Luftfeuchtigkeit => String::from("humidity"),
            Sensortypes::Temperatur => String::from("temperature"),
        }
    }
    pub fn get_color(&self) -> RGBColor {
        match self {
            Sensortypes::Feuchtigkeit => RGBColor(0, 0, 255),
            Sensortypes::Luftfeuchtigkeit => RGBColor(0, 255, 0),
            Sensortypes::Temperatur => RGBColor(255, 0, 0),
        }
    }
}
impl DetailPage {
    pub fn new() -> DetailPage {
        let plant = DetailPlant {
            id: "1".to_string(),
            name: String::from("no name"),
            description: String::from("no description"),
            charts: PlantCharts::new(Vec::new(), DetailMessage::Loaded),
        };
        let detail_page = DetailPage {
            plant: plant,
            message: DetailMessage::Load,
        };
        detail_page
    }
    pub fn update(&mut self, message: DetailMessage) {
        info!("Updating detail page");
        match message {
            DetailMessage::Load => {
                self.message = DetailMessage::Load;
            }
            DetailMessage::PlantData(id) => {
                let plant_data = get_plant_details(id.clone()).unwrap();
                let graph_data = get_graphs(vec![id.clone()], "soil-moisture".to_string());
                self.plant = self.plant.new(id, graph_data.unwrap());
                self.message = DetailMessage::Loaded;
            }
            DetailMessage::SwitchGraph(sensor_types) => {
                let sensor_name = sensor_types.get_name();
                let graph_data = get_graphs(vec!["1".to_string()], sensor_name);
                self.plant
                    .update_charts(DetailMessage::Loaded, graph_data.unwrap(), sensor_types);
                self.message = DetailMessage::Loaded;
            }
            DetailMessage::Loaded => {}
            DetailMessage::Search(value) => {
                self.plant.id = value;
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
        info!("{:?}", self.message);
        let row = if self.message != DetailMessage::Load {
            let plant = &self.plant;

            let chart = ChartWidget::new(plant.charts.clone());
            let row: Row<DetailMessage> = Row::new()
                .push(Text::new(plant.name.clone()))
                .spacing(20)
                .push(Text::new(plant.description.clone()))
                .spacing(20)
                .push(
                    Button::new(Text::new("Feuchtigkeit"))
                        .on_press(DetailMessage::SwitchGraph(Sensortypes::Feuchtigkeit)),
                )
                .push(
                    Button::new(Text::new("Luftfeuchtigkeit"))
                        .on_press(DetailMessage::SwitchGraph(Sensortypes::Luftfeuchtigkeit)),
                )
                .push(
                    Button::new(Text::new("Temperatur"))
                        .on_press(DetailMessage::SwitchGraph(Sensortypes::Temperatur)),
                )
                .push(Button::new(Text::new("Neue Pflanze")).on_press(DetailMessage::Load))
                .push(chart);
            row
        } else {
            let row = Row::new()
                .push(TextInput::new("Loading...", &self.plant.id).on_input(DetailMessage::Search))
                .push(
                    Button::new(Text::new("Load"))
                        .on_press(DetailMessage::PlantData(self.plant.id.clone())),
                )
                .spacing(20)
                .align_items(Center);
            row
        };
        let content: Element<'_, DetailMessage> = Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();
        content.map(Message::Detail)
    }
}
