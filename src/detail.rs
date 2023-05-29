use crate::graphs::PlantCharts;
use crate::requests::{get_all_plant_ids, get_graphs, get_plant_details, GraphData, PlantMetadata};
use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{Element, Length};
use iced_aw::tab_bar::TabLabel;
use iced_core::Alignment::Center;
use log::info;
use plotters::prelude::*;
use plotters_iced::ChartWidget;
use std::vec;

#[derive(Debug, Clone)]
pub struct DetailPlant {
    pub id: String,
    pub data: PlantMetadata,
    pub charts: PlantCharts<DetailMessage>,
}

impl DetailPlant {
    pub fn new(id: String, graph_data: Vec<GraphData>) -> Self {
        let charts =
            PlantCharts::create_charts(DetailMessage::Load, graph_data, Sensortypes::Feuchtigkeit);
        let plant_data = get_plant_details(id).unwrap();
        DetailPlant {
            id: String::new(),
            data: plant_data.0,
            charts,
        }
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
            id: String::new(),
            data: PlantMetadata::default(),
            charts: PlantCharts::new(Vec::new(), DetailMessage::Loaded),
        };
        DetailPage {
            plant,
            message: DetailMessage::Load,
        }
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
                self.plant = DetailPlant::new(id, graph_data.unwrap());
                self.message = DetailMessage::Loaded;
            }
            DetailMessage::SwitchGraph(sensor_types) => {
                let sensor_name = sensor_types.get_name();
                let graph_data = get_graphs(vec!["1".to_string()], sensor_name);
                self.plant.charts = PlantCharts::update_charts(
                    &self.plant.charts,
                    DetailMessage::Loaded,
                    graph_data.unwrap(),
                    sensor_types,
                );
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
            let mut care_tip_col: Column<DetailMessage> = Column::new();
            for caretip in &plant.data.additionalCareTips {
                care_tip_col = care_tip_col.push(Text::new(caretip.clone()));
            }
            let chart = ChartWidget::new(plant.charts.clone());
            let row: Row<DetailMessage> = Row::new()
                .push(Text::new(plant.data.name.clone()))
                .spacing(20)
                .push(Text::new(plant.data.description.clone()))
                .spacing(20)
                .push(Text::new(plant.data.location.clone()))
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
                .push(
                    Button::new(Text::new("Andere Pflanze anzeigen")).on_press(DetailMessage::Load),
                );
            let container = Container::new(chart)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y();
            let chart_col = Column::new().push(row).push(container);
            let row = Row::new()
                .push(care_tip_col)
                .push(chart_col)
                .spacing(20)
                .align_items(Center);
            row
        } else {
            let ids = get_all_plant_ids().unwrap();
            info!("Got all plant ids: {:?}", ids);
            let mut id_and_name = Vec::new();
            for id in ids {
                let plant_data = get_plant_details(id.clone()).unwrap();
                id_and_name.push((id, plant_data.0.name));
            }
            let mut id_name_column: Column<DetailMessage> = Column::new();
            for id in id_and_name {
                let id_name_row = Row::new()
                    .push(Text::new(id.0.clone()))
                    .push(Text::new(id.1.clone()))
                    .spacing(20);
                id_name_column = id_name_column.push(id_name_row);
            }
            let row = Row::new()
                .push(
                    TextInput::new("Keine Pflanze ausgewählt...", &self.plant.id)
                        .on_input(DetailMessage::Search),
                )
                .push(
                    Button::new(Text::new("Anzeigen"))
                        .on_press(DetailMessage::PlantData(self.plant.id.clone())),
                )
                .spacing(20)
                .align_items(Center);
            let column = Column::new().push(id_name_column).push(row);
            let row = Row::new().push(column).spacing(20).align_items(Center);
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
