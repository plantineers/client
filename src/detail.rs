use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{get_all_plant_ids, get_graphs, GraphData};
use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, row, Button, Column, Container, Row, Text};
use iced::{Element, Length};
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
    pub id: i32,
    pub name: String,
    pub description: String,
    pub sensors: Vec<Sensortypes>,
    pub charts: PlantCharts<DetailMessage>,
}

impl DetailPlant {
    pub fn new() -> Self {
        let plant = DetailPlant {
            id: 0,
            name: String::from("Kaktus"),
            description: String::from("The big one"),
            sensors: vec![
                Sensortypes::Feuchtigkeit,
                Sensortypes::Temperatur,
                Sensortypes::Luftfeuchtigkeit,
            ],
            charts: PlantCharts::test(DetailMessage::PlantData),
        };
        plant
    }
    pub fn update_charts(
        &mut self,
        message: DetailMessage,
        graph_data: Vec<GraphData>,
        sensor: Sensortypes,
    ) {
        let mut charts = vec![];
        for data in graph_data {
            let chart = PlantChart::new(
                format!("{:?}", sensor),
                data.values,
                (0..data.timestamps.len() as i32).collect_vec(),
                sensor.get_color(),
            );
            charts.push(chart);
        }
        let mut charts = PlantCharts::new(charts, message);
        self.charts = charts;
    }
}
#[derive(Debug, Clone)]
pub enum DetailMessage {
    Load,
    PlantData,
    SwitchGraph(Sensortypes),
}

pub(crate) struct DetailPage {
    pub plant: DetailPlant,
    pub message: DetailMessage,
}
#[derive(Debug, Clone, Copy)]
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
        DetailPage {
            plant: DetailPlant::new(),
            message: DetailMessage::Load,
        }
    }

    pub fn update(&mut self, message: DetailMessage) {
        match message {
            DetailMessage::Load => {
                let graph_data = get_graphs(vec!["1".to_string()], "soil-moisture".to_string());
                info!("Graph data: {:?}", graph_data.unwrap());
            }
            DetailMessage::PlantData => {
                println!("Loading plant data");
            }
            DetailMessage::SwitchGraph(Sensortypes) => {
                println!("Switching graph to {}", Sensortypes.get_name());
                let sensor_name = Sensortypes.get_name();
                let graph_data = get_graphs(vec!["1".to_string()], sensor_name);
                self.plant.update_charts(
                    DetailMessage::PlantData,
                    graph_data.unwrap(),
                    Sensortypes,
                );
                info!("New charts: {:?}", self.plant.charts);
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
        let plant = &self.plant;
        let left_column: Column<DetailMessage> = Column::new()
            .push(Text::new(plant.name.clone()))
            .spacing(20)
            .push(Text::new(plant.description.clone()))
            .spacing(20)
            .align_items(Center);

        let chart = ChartWidget::new(plant.charts.clone());
        let right_column: Column<DetailMessage> = Column::new()
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
            .push(chart);
        let row = row!(left_column, right_column);
        let content: Element<'_, DetailMessage> = Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();
        content.map(Message::Detail)
    }
}
