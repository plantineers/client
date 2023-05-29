use crate::detail::{DetailPlant, Sensortypes};
use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{get_all_plant_ids, get_graphs, get_plant_details, GraphData, PlantMetadata};
use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button::Primary;
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Row, Text, TextInput};
use iced::Theme::Dark;
use iced::{Application, Command, Element, Length, Renderer, Sandbox, Settings};
use iced_aw::TabLabel;
use itertools::Itertools;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    NewPlant,
    Plant,
    Refresh,
    SwitchGraph(Sensortypes),
    FieldUpdated(u8, String),
}

pub(crate) struct HomePage {
    new_plant_bool: bool,
    new_plant: PlantMetadata,
    charts: PlantCharts<HomeMessage>,
    active_sensor: Sensortypes,
    ids: Vec<String>,
}

impl HomePage {
    pub fn new() -> Self {
        let ids = get_all_plant_ids().unwrap();
        let graph_data: Vec<GraphData> =
            get_graphs(ids.clone(), Sensortypes::Luftfeuchtigkeit.get_name()).unwrap();
        let mut vec_chart = Vec::new();
        for data in graph_data {
            vec_chart.push(PlantChart::new(
                "".to_string(),
                (0..data.timestamps.len() as i32).collect_vec(),
                data.values,
                Default::default(),
            ));
        }
        let charts = PlantCharts::new(vec_chart, HomeMessage::Plant);
        HomePage {
            new_plant_bool: false,
            new_plant: PlantMetadata::default(),
            charts,
            active_sensor: Sensortypes::Luftfeuchtigkeit,
            ids,
        }
    }

    pub fn update(&mut self, message: HomeMessage) {
        match message {
            HomeMessage::Plant => (),
            HomeMessage::Refresh => {
                let ids = get_all_plant_ids().unwrap();
                let graph_data: Vec<GraphData> =
                    get_graphs(ids, self.active_sensor.get_name()).unwrap();
                self.charts = PlantCharts::update_charts(
                    &self.charts.clone(),
                    HomeMessage::Plant,
                    graph_data,
                    Sensortypes::Luftfeuchtigkeit,
                )
            }
            HomeMessage::SwitchGraph(sensortypes) => {
                self.active_sensor = sensortypes;
                let graph_data: Vec<GraphData> =
                    get_graphs(self.ids.clone(), sensortypes.get_name()).unwrap();
                self.charts = PlantCharts::update_charts(
                    &self.charts.clone(),
                    HomeMessage::Plant,
                    graph_data,
                    sensortypes,
                )
            }
            HomeMessage::NewPlant => {
                self.new_plant_bool = true;
            }
            HomeMessage::FieldUpdated(index, value) => match index {
                0 => {
                    self.new_plant.name = value;
                }
                1 => {
                    self.new_plant.description = value;
                }
                2 => {
                    self.new_plant.location = value;
                }
                3 => {
                    self.new_plant.additionalCareTips =
                        value.split("/n").map(String::from).collect();
                }
                _ => (),
            },
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
        if self.new_plant_bool {
            let caretip_string = "";
            let column: Column<HomeMessage> = Column::new()
                .push(
                    TextInput::new("Pflanzenname", &self.new_plant.name)
                        .on_input(|input| HomeMessage::FieldUpdated(0, input)),
                )
                .spacing(20)
                .push(
                    TextInput::new(
                        "Beschreibung der Pflanzengattung",
                        &self.new_plant.description,
                    )
                    .on_input(|input| HomeMessage::FieldUpdated(1, input)),
                )
                .spacing(20)
                .push(
                    TextInput::new("Position der Pflanze", &self.new_plant.location)
                        .on_input(|input| HomeMessage::FieldUpdated(2, input)),
                )
                .spacing(20)
                .push(
                    TextInput::new("Pflegehinweise", caretip_string)
                        .on_input(|input| HomeMessage::FieldUpdated(3, input)),
                )
                .spacing(20);
            let content: Element<'_, HomeMessage> = Container::new(column).into();
            content.map(Message::Home)
        } else {
            let chart_widget = ChartWidget::new(self.charts.clone());
            let row = Row::new().push(chart_widget).push(
                Column::new()
                    .push(Button::new("Refresh").on_press(HomeMessage::Refresh))
                    .spacing(20)
                    .push(
                        Button::new("Temperatur")
                            .on_press(HomeMessage::SwitchGraph(Sensortypes::Temperatur)),
                    )
                    .spacing(20)
                    .push(
                        Button::new("Luftfeuchtigkeit")
                            .on_press(HomeMessage::SwitchGraph(Sensortypes::Luftfeuchtigkeit)),
                    )
                    .spacing(20)
                    .push(
                        Button::new("Feuchtigkeit")
                            .on_press(HomeMessage::SwitchGraph(Sensortypes::Feuchtigkeit)),
                    ),
            );
            let lower_row: Row<HomeMessage, Renderer> =
                Row::new().push(Button::new("Add Plant").on_press(HomeMessage::NewPlant));
            let column = Column::new().push(lower_row).push(row);
            let content: Element<'_, HomeMessage> = Container::new(column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into();
            content.map(Message::Home)
        }
    }
}
