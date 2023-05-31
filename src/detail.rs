use crate::detail::DetailMessage::OpenModal;
use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{
    get_all_plant_ids, get_graphs, get_plant_details, GraphData, PlantGroupMetadata, PlantMetadata,
};
use crate::{Icon, Message, MyStylesheet, Tab, TEXT_SIZE};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{theme, Element, Length};
use iced_aw::tab_bar::TabLabel;
use iced_aw::{Card, Modal};
use iced_core::Alignment::Center;
use log::info;
use plotters::prelude::*;
use plotters_iced::ChartWidget;
use std::fmt::{Display, Formatter};
use std::vec;

#[derive(Debug, Clone)]
pub struct DetailPlant {
    pub id: String,
    pub data: PlantMetadata,
    pub charts: PlantCharts<DetailMessage>,
}

impl DetailPlant {
    pub fn new(id: String, graph_data: Vec<GraphData>) -> Self {
        let plant_data: (PlantMetadata, PlantGroupMetadata) = get_plant_details(id).unwrap();
        let charts = PlantCharts::create_charts(
            DetailMessage::Load,
            graph_data,
            Sensortypes::Feuchtigkeit,
            plant_data.0.name.clone(),
        );
        DetailPlant {
            id: String::new(),
            data: plant_data.0,
            charts,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DetailMessage {
    OkButtonPressed,
    OpenModal,
    CloseModal,
    Load,
    PlantData(String),
    Loaded,
    SwitchGraph(Sensortypes),
    Search(String),
    FieldUpdated(u8, String),
}

pub(crate) struct DetailPage {
    pub modal: bool,
    pub plant: DetailPlant,
    pub message: DetailMessage,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sensortypes {
    Feuchtigkeit,
    Luftfeuchtigkeit,
    Temperatur,
}
impl Display for Sensortypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sensortypes::Feuchtigkeit => write!(f, "Feuchtigkeit"),
            Sensortypes::Luftfeuchtigkeit => write!(f, "Luftfeuchtigkeit"),
            Sensortypes::Temperatur => write!(f, "Temperatur"),
        }
    }
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
    pub fn iter() -> impl Iterator<Item = Sensortypes> {
        [
            Sensortypes::Feuchtigkeit,
            Sensortypes::Luftfeuchtigkeit,
            Sensortypes::Temperatur,
        ]
        .iter()
        .copied()
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
            modal: false,
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
                    self.plant.data.name.clone(),
                );
                info!("Sensor: {:?}", self.plant.data.plantGroup.sensorRanges);
                self.plant
                    .data
                    .plantGroup
                    .sensorRanges
                    .iter()
                    .filter(|sensor| sensor.sensorType.name == sensor_types.get_name())
                    .for_each(|sensor| {
                        self.plant.charts.charts.push(PlantChart::new(
                            format!("{:?}_Max_Grenze", self.plant.data.name.clone()),
                            self.plant.charts.charts[0].x.clone(),
                            vec![sensor.max; self.plant.charts.charts[0].x.len()],
                            BLACK,
                        ));
                        self.plant.charts.charts.push(PlantChart::new(
                            format!("{:?}_Min_Grenze", self.plant.data.name.clone()),
                            self.plant.charts.charts[0].x.clone(),
                            vec![sensor.min; self.plant.charts.charts[0].x.len()],
                            BLACK,
                        ));
                    });
                info!("Charts: {:?}", self.plant.charts.charts);
                self.message = DetailMessage::Loaded;
            }
            DetailMessage::Loaded => {}
            DetailMessage::Search(value) => {
                self.plant.id = value;
            }
            DetailMessage::OpenModal => {
                self.modal = true;
            }
            DetailMessage::CloseModal => {
                self.modal = false;
            }
            DetailMessage::OkButtonPressed => {
                self.modal = false;
            }
            DetailMessage::FieldUpdated(index, value) => match index {
                0 => self.plant.data.name = value,
                1 => self.plant.data.description = value,
                2 => self.plant.data.location = value,
                3 => self.plant.data.species = value,
                4 => self.plant.data.plantGroup.name = value,
                _ => {}
            },
        }
    }
}

impl Tab for DetailPage {
    type Message = Message;

    fn title(&self) -> String {
        if self.message == DetailMessage::Load {
            return String::from("Verfügbare Pflanzen");
        }
        String::from("Detailübersicht")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Detailpage.into(), self.title())
    }
    fn content(&self) -> Element<'_, Self::Message> {
        if self.modal {
            let container: Container<DetailMessage> =
                Container::new(Text::new("Pflanze editieren").size(TEXT_SIZE))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center);
            let content: Element<'_, DetailMessage> = Modal::new(self.modal, container, || {
                Card::new(
                    Text::new("Pflanze editieren")
                        .size(TEXT_SIZE)
                        .horizontal_alignment(Horizontal::Center),
                    Column::new()
                        .push(
                            TextInput::new("Pflanzenname", &self.plant.data.name)
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(0, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new(
                                "Beschreibung der Pflanze",
                                &self.plant.data.description,
                            )
                            .size(TEXT_SIZE)
                            .on_input(|input| DetailMessage::FieldUpdated(1, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new("Position der Pflanze", &self.plant.data.location)
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(2, input)),
                        )
                        .spacing(20)
                        .spacing(20)
                        .push(
                            TextInput::new("Pflanzenspecies", &self.plant.data.species)
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(3, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new("Pflanzengruppe", &self.plant.data.plantGroup.name)
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(4, input)),
                        )
                        .spacing(20),
                )
                .foot(
                    Row::new()
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill)
                        .push(
                            Button::new(
                                Text::new("Cancel")
                                    .size(TEXT_SIZE)
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(DetailMessage::CloseModal),
                        )
                        .push(
                            Button::new(
                                Text::new("Ok")
                                    .size(TEXT_SIZE)
                                    .horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(DetailMessage::OkButtonPressed),
                        ),
                )
                .max_width(300.0)
                .on_close(DetailMessage::CloseModal)
                .into()
            })
            .backdrop(DetailMessage::CloseModal)
            .on_esc(DetailMessage::CloseModal)
            .into();
            content.map(Message::Detail)
        } else {
            let row = if self.message != DetailMessage::Load {
                let plant = &self.plant;
                let chart = ChartWidget::new(plant.charts.clone());
                let container: Container<DetailMessage> = Container::new(chart)
                    .style(theme::Container::Custom(Box::new(MyStylesheet)))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y();
                let mut detail_column: Column<DetailMessage> = Column::new()
                    .push(
                        Text::new(format!("Pflanzenname: {}", plant.data.name.clone()))
                            .size(TEXT_SIZE),
                    )
                    .spacing(20)
                    .push(
                        Text::new(format!(
                            "Pflanzenbeschreibung: {}",
                            plant.data.description.clone()
                        ))
                        .size(TEXT_SIZE),
                    )
                    .spacing(20)
                    .push(
                        Text::new(format!("Pflanzenstandort: {}", plant.data.location.clone()))
                            .size(TEXT_SIZE),
                    )
                    .spacing(20)
                    .push(
                        Text::new(format!("Pflanzenart: {}", plant.data.species.clone()))
                            .size(TEXT_SIZE),
                    )
                    .spacing(20)
                    .push(
                        Text::new(format!(
                            "Pflanzengruppe: {}",
                            plant.data.plantGroup.name.clone()
                        ))
                        .size(TEXT_SIZE),
                    )
                    .spacing(20)
                    .push(Text::new("Pflegetipps: ").size(TEXT_SIZE));
                for caretip in &plant.data.additionalCareTips {
                    detail_column = detail_column.push(Text::new(caretip.clone()).size(TEXT_SIZE));
                }
                detail_column =
                    detail_column.push(Text::new("Gruppen Pflegetipps: ").size(TEXT_SIZE));
                for group_caretip in &plant.data.plantGroup.careTips {
                    detail_column =
                        detail_column.push(Text::new(group_caretip.clone()).size(TEXT_SIZE));
                }
                let row: Row<DetailMessage> = Row::new()
                    .push(
                        Button::new(Text::new("Feuchtigkeit").size(TEXT_SIZE))
                            .on_press(DetailMessage::SwitchGraph(Sensortypes::Feuchtigkeit)),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Luftfeuchtigkeit").size(TEXT_SIZE))
                            .on_press(DetailMessage::SwitchGraph(Sensortypes::Luftfeuchtigkeit)),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Temperatur").size(TEXT_SIZE))
                            .on_press(DetailMessage::SwitchGraph(Sensortypes::Temperatur)),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Andere Pflanze anzeigen").size(TEXT_SIZE))
                            .on_press(DetailMessage::Load),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Pflanze bearbeiten").size(TEXT_SIZE))
                            .on_press(DetailMessage::OpenModal),
                    );
                let chart_col = Column::new().push(row).push(container);
                let row = Row::new()
                    .push(detail_column)
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
                let mut id_name_column: Column<DetailMessage> = Column::new().push(
                    Row::new()
                        .push(Text::new("ID").size(TEXT_SIZE))
                        .push(Text::new("Name").size(TEXT_SIZE))
                        .spacing(20),
                );
                for id in id_and_name {
                    let id_name_row = Row::new()
                        .push(Text::new(id.0.clone()).size(TEXT_SIZE))
                        .push(Text::new(id.1.clone()).size(TEXT_SIZE))
                        .spacing(20);
                    id_name_column = id_name_column.push(id_name_row);
                }
                let row = Row::new()
                    .push(
                        TextInput::new(
                            "Trage die ID der Pflanze ein, die du betrachten möchtest",
                            &self.plant.id,
                        )
                        .size(TEXT_SIZE)
                        .on_input(DetailMessage::Search),
                    )
                    .push(
                        Button::new(Text::new("Anzeigen").size(TEXT_SIZE))
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
}
