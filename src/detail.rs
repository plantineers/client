use crate::graphs::{PlantChart, PlantCharts};
use std::collections::HashMap;

use crate::requests::{GraphData, PlantGroupMetadata, PlantMetadata};
use crate::{Icon, Message, MyStylesheet, Tab, API_CLIENT, TEXT_SIZE};
use iced::alignment::{Horizontal, Vertical};
use iced::futures::TryFutureExt;

use iced::widget::{scrollable, Button, Column, Container, Row, Text, TextInput};
use iced::{theme, Command, Element, Length};
use iced_aw::tab_bar::TabLabel;
use iced_aw::{Card, Modal};
use iced_core::Alignment::Center;
use itertools::enumerate;
use log::info;
use plotters::prelude::*;
use plotters_iced::ChartWidget;
use rand::Rng;
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
        let plant_data: (PlantMetadata, PlantGroupMetadata) = API_CLIENT
            .get()
            .unwrap()
            .clone()
            .get_plant_details(id.clone())
            .unwrap_or_default();
        let charts = PlantCharts::create_charts(
            DetailMessage::Loaded,
            graph_data,
            Sensortypes::Feuchtigkeit,
            vec![plant_data.0.name.clone()],
        );
        DetailPlant {
            id,
            data: plant_data.0,
            charts,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DetailMessage {
    OkButtonPressed,
    SwitchTime(chrono::Duration),
    OpenModalPlant,
    OpenModalGroup,
    CloseModal,
    Delete,
    Pending,
    Load,
    PlantData(String),
    Loaded,
    SwitchGraph(Sensortypes),
    Search(String),
    FieldUpdated(u8, String),
    DeleteSuccess,
}

pub(crate) struct DetailPage {
    pub active_sensor: Sensortypes,
    pub timerange: (String, String),
    pub modal: bool,
    pub modal_is_plant: bool,
    pub additionalCareTips: String,
    pub careTips: String,
    pub sensor_border: HashMap<String, String>,
    pub id_names: Vec<(String, String)>,
    pub plant: DetailPlant,
    pub message: DetailMessage,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sensortypes {
    Feuchtigkeit,
    Luftfeuchtigkeit,
    Temperatur,
    Licht,
}
impl Display for Sensortypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sensortypes::Feuchtigkeit => write!(f, "Feuchtigkeit"),
            Sensortypes::Luftfeuchtigkeit => write!(f, "Luftfeuchtigkeit"),
            Sensortypes::Temperatur => write!(f, "Temperatur"),
            Sensortypes::Licht => write!(f, "Licht"),
        }
    }
}
impl Sensortypes {
    pub fn get_name(&self) -> String {
        match self {
            Sensortypes::Feuchtigkeit => String::from("soil-moisture"),
            Sensortypes::Luftfeuchtigkeit => String::from("humidity"),
            Sensortypes::Temperatur => String::from("temperature"),
            Sensortypes::Licht => String::from("light"),
        }
    }
    pub fn get_color(&self) -> RGBColor {
        match self {
            Sensortypes::Feuchtigkeit => RGBColor(0, 0, 255),
            Sensortypes::Luftfeuchtigkeit => RGBColor(0, 255, 0),
            Sensortypes::Temperatur => RGBColor(255, 0, 0),
            Sensortypes::Licht => RGBColor(255, 255, 0),
        }
    }
    pub fn get_color_with_random_offset(&self) -> RGBColor {
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(0..=255);
        let offset2 = rng.gen_range(0..=255);
        let offset3 = rng.gen_range(0..=50);
        match self {
            Sensortypes::Feuchtigkeit => RGBColor(offset, offset2, 255 - offset3),
            Sensortypes::Luftfeuchtigkeit => RGBColor(offset, 255 - offset3, offset2),
            Sensortypes::Temperatur => RGBColor(255 - offset3, offset, offset2),
            Sensortypes::Licht => RGBColor(255 - offset3, 255 - offset3.clone(), offset),
        }
    }
    pub fn iter() -> impl Iterator<Item = Sensortypes> {
        [
            Sensortypes::Feuchtigkeit,
            Sensortypes::Luftfeuchtigkeit,
            Sensortypes::Temperatur,
            Sensortypes::Licht,
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
            active_sensor: Sensortypes::Feuchtigkeit,
            id_names: vec![],
            timerange: (
                "2019-01-01T00:00:00.000Z".to_string(),
                chrono::offset::Local::now()
                    .format("%Y-%m-%dT%H:%M:%S.000Z")
                    .to_string(),
            ),
            modal: false,
            modal_is_plant: true,
            careTips: String::new(),
            sensor_border: HashMap::new(),
            additionalCareTips: String::new(),
            plant,
            message: DetailMessage::Pending,
        }
    }
    pub fn min_max_graphs(&self, sensor_types: Sensortypes) -> Vec<PlantChart> {
        let mut charts = vec![];
        self.plant
            .data
            .plantGroup
            .sensorRanges
            .iter()
            .filter(|sensor| sensor.sensorType.name == sensor_types.get_name())
            .for_each(|sensor| {
                let current_chart = self
                    .plant
                    .charts
                    .charts
                    .get(0)
                    .map(|chart| chart.clone())
                    .unwrap_or_default();
                charts.push(PlantChart::new(
                    format!("{:?}_Max_Grenze", self.plant.data.name.clone()),
                    current_chart.x.clone(),
                    vec![sensor.max; current_chart.x.len()],
                    BLACK,
                ));
                charts.push(PlantChart::new(
                    format!("{:?}_Min_Grenze", self.plant.data.name.clone()),
                    current_chart.x.clone(),
                    vec![sensor.min; current_chart.x.len()],
                    BLACK,
                ))
            });
        charts
    }
    pub fn update(&mut self, message: DetailMessage) -> Command<DetailMessage> {
        match message {
            DetailMessage::SwitchTime(value) => {
                info!("Switching time to {:?}", value);
                let now = chrono::offset::Local::now();
                let start = now - value;
                self.timerange = (
                    start.format("%Y-%m-%dT%H:%M:%S.000Z").to_string(),
                    now.format("%Y-%m-%dT%H:%M:%S.000Z").to_string(),
                );
                return self.update(DetailMessage::SwitchGraph(self.active_sensor));
            }
            DetailMessage::Pending => {
                self.message = DetailMessage::Pending;
            }
            DetailMessage::Delete => {
                let plant_id = self.plant.id.clone();
                return Command::perform(
                    // TODO: Error handling here by not unwrapping
                    API_CLIENT
                        .get()
                        .unwrap()
                        .clone()
                        .delete_plant(plant_id)
                        .unwrap_or_else(|_| ()),
                    |_| DetailMessage::DeleteSuccess,
                );
            }
            DetailMessage::Load => {
                info!("Refresh Id List");
                //if empty self.id_names should be an empty vec
                self.id_names = API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .get_all_plant_ids_names()
                    .unwrap_or_default();
                self.message = DetailMessage::Pending;
            }
            DetailMessage::PlantData(id) => {
                let data = API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .get_graphs(
                        vec![id.clone()],
                        true,
                        Sensortypes::Feuchtigkeit.get_name(),
                        self.timerange.clone(),
                    )
                    .unwrap_or_default();
                let graph_data: Vec<GraphData> = data.iter().map(|(g, _)| g.clone()).collect();
                self.plant = DetailPlant::new(id, graph_data);
                self.plant.data.additionalCareTips.iter().for_each(|x| {
                    self.additionalCareTips.push_str(x);
                    self.additionalCareTips.push(';');
                });
                self.plant.data.plantGroup.careTips.iter().for_each(|x| {
                    self.careTips.push_str(x);
                    self.careTips.push(';');
                });
                info!("SensorType: {:?}", self.plant.data.plantGroup.sensorRanges);
                self.plant
                    .data
                    .plantGroup
                    .sensorRanges
                    .iter()
                    .for_each(|x| match x.sensorType.name.as_str() {
                        //TODO: Maybe get sensors from Api and match them here to a hashmap
                        "soil-moisture" => {
                            self.sensor_border.insert(
                                Sensortypes::Feuchtigkeit.get_name(),
                                format!("{};{}", x.max, x.min),
                            );
                        }
                        "humidity" => {
                            self.sensor_border.insert(
                                Sensortypes::Luftfeuchtigkeit.get_name(),
                                format!("{};{}", x.max, x.min),
                            );
                        }
                        "temperature" => {
                            self.sensor_border.insert(
                                Sensortypes::Temperatur.get_name(),
                                format!("{};{}", x.max, x.min),
                            );
                        }
                        "light" => {
                            self.sensor_border.insert(
                                Sensortypes::Licht.get_name(),
                                format!("{};{}", x.max, x.min),
                            );
                        }
                        _ => {}
                    });
                Sensortypes::iter().for_each(|sensor| {
                    if !self.sensor_border.contains_key(sensor.get_name().as_str()) {
                        self.sensor_border
                            .insert(sensor.get_name(), String::from("0;0"));
                    }
                });
                self.plant
                    .charts
                    .charts
                    .append(&mut self.min_max_graphs(Sensortypes::Feuchtigkeit));
                self.message = DetailMessage::Loaded;
            }
            DetailMessage::SwitchGraph(sensor_types) => {
                info!("Switching Graph to {:?}", sensor_types);
                self.active_sensor = sensor_types;
                let sensor_name = sensor_types.get_name();
                let data = API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .get_graphs(
                        vec![self.plant.id.clone()],
                        true,
                        sensor_name,
                        self.timerange.clone(),
                    )
                    .unwrap_or_default();
                let graph_data: Vec<GraphData> = data.iter().map(|(g, _)| g.clone()).collect();
                self.plant.charts = PlantCharts::update_charts(
                    &self.plant.charts,
                    DetailMessage::Loaded,
                    graph_data,
                    sensor_types,
                    vec![self.plant.data.name.clone()],
                );
                self.plant
                    .charts
                    .charts
                    .append(&mut self.min_max_graphs(sensor_types));
                self.message = DetailMessage::Loaded;
            }
            DetailMessage::Loaded => {}
            DetailMessage::Search(value) => {
                self.plant.id = value;
            }
            DetailMessage::OpenModalPlant => {
                self.modal_is_plant = true;
                self.modal = true;
            }
            DetailMessage::OpenModalGroup => {
                self.modal_is_plant = false;
                self.modal = true;
            }
            DetailMessage::CloseModal => {
                self.modal = false;
            }
            DetailMessage::OkButtonPressed => {
                return if self.modal_is_plant {
                    self.plant.data.additionalCareTips = self
                        .additionalCareTips
                        .split(';')
                        .map(|x| x.to_string())
                        .collect();
                    self.modal = false;
                    Command::perform(
                        API_CLIENT.get().unwrap().clone().create_plant(
                            self.plant.data.clone(),
                            self.plant.data.plantGroup.id.clone(),
                            Some(self.plant.id.clone()),
                        ),
                        |_| DetailMessage::Loaded,
                    )
                } else {
                    self.plant.data.plantGroup.careTips =
                        self.careTips.split(';').map(|x| x.to_string()).collect();
                    for sensor in self.plant.data.plantGroup.sensorRanges.iter_mut() {
                        for i in Sensortypes::iter() {
                            if i.get_name() == sensor.sensorType.name {
                                    .collect::<Vec<&str>>();
                                sensor.max = self
                                    .sensor_border
                                    .clone()
                                    .get(i.get_name().as_str())
                                    .unwrap()
                                    .split(';')
                                    .next()
                                    .unwrap()
                                    .parse()
                                    .unwrap_or_default();
                                sensor.min = self
                                    .sensor_border
                                    .clone()
                                    .get(i.get_name().as_str())
                                    .unwrap()
                                    .split(';')
                                    .last()
                                    .unwrap()
                                    .parse()
                                    .unwrap_or_default();
                            }
                        }
                    }
                    self.modal = false;
                    Command::perform(
                        API_CLIENT.get().unwrap().clone().create_group(
                            self.plant.data.plantGroup.clone(),
                            Some(self.plant.data.plantGroup.id.to_string()),
                        ),
                        |_| DetailMessage::Loaded,
                    )
                }
            }
            DetailMessage::FieldUpdated(index, value) => match index {
                0 => self.plant.data.name = value,
                1 => self.plant.data.description = value,
                2 => self.plant.data.location = value,
                3 => self.plant.data.species = value,
                4 => self.plant.data.plantGroup.id = value.parse().unwrap_or(1),
                5 => self.additionalCareTips = value,
                6 => {
                    self.plant.data.plantGroup.name = value;
                }
                7 => {
                    self.plant.data.description = value;
                }
                8 => {
                    self.careTips = value;
                }
                9 => {
                    self.sensor_border
                        .insert(Sensortypes::Feuchtigkeit.get_name(), value);
                }
                10 => {
                    self.sensor_border
                        .insert(Sensortypes::Luftfeuchtigkeit.get_name(), value);
                }
                11 => {
                    self.sensor_border
                        .insert(Sensortypes::Temperatur.get_name(), value);
                }
                12 => {
                    self.sensor_border
                        .insert(Sensortypes::Licht.get_name(), value);
                }
                _ => {}
            },
            DetailMessage::DeleteSuccess => {
                self.modal = false;
                self.message = DetailMessage::Pending;
            }
        }
        Command::none()
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
            if self.modal_is_plant {
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
                                TextInput::new("Pflanzenspezies", &self.plant.data.species)
                                    .size(TEXT_SIZE)
                                    .on_input(|input| DetailMessage::FieldUpdated(3, input)),
                            )
                            .spacing(20)
                            .push(
                                TextInput::new(
                                    "Pflanzengruppe",
                                    &self.plant.data.plantGroup.id.to_string(),
                                )
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(4, input)),
                            )
                            .spacing(20)
                            .push(
                                TextInput::new("Pflegehinweise", &self.additionalCareTips)
                                    .size(TEXT_SIZE)
                                    .on_input(|input| DetailMessage::FieldUpdated(5, input)),
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
                                    Text::new("Löschen")
                                        .size(TEXT_SIZE)
                                        .horizontal_alignment(Horizontal::Center),
                                )
                                .style(theme::Button::Destructive)
                                .width(Length::Fill)
                                .on_press(DetailMessage::Delete),
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
                let container: Container<DetailMessage> =
                    Container::new(Text::new("Neue Gruppe").size(TEXT_SIZE))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center);
                let content: Element<'_, DetailMessage> = Modal::new(self.modal, container, || {
                    // Todo: Is there a possibility to make this more generic?
                    Card::new(
                        Text::new("Gruppe bearbeiten")
                            .size(TEXT_SIZE)
                            .horizontal_alignment(Horizontal::Center),
                        Column::new()
                            .push(
                                TextInput::new("Gruppennamen", &self.plant.data.plantGroup.name)
                                    .size(TEXT_SIZE)
                                    .on_input(|input| DetailMessage::FieldUpdated(6, input)),
                            )
                            .spacing(20)
                            .push(
                                TextInput::new(
                                    "Beschreibung der Gruppe",
                                    &self.plant.data.plantGroup.description,
                                )
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(7, input)),
                            )
                            .spacing(20)
                            .push(
                                TextInput::new("Pflegehinweise", &self.careTips)
                                    .size(TEXT_SIZE)
                                    .on_input(|input| DetailMessage::FieldUpdated(8, input)),
                            )
                            .spacing(20)
                            .push(
                                Text::new("Die Grenzen werden so eingetragen: max;min")
                                    .size(TEXT_SIZE),
                            )
                            .push(
                                TextInput::new(
                                    "Feuchtigkeitsgrenzwerte",
                                    &self
                                        .sensor_border
                                        .get(Sensortypes::Feuchtigkeit.get_name().as_str())
                                        .unwrap(),
                                )
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(9, input)),
                            )
                            .push(
                                TextInput::new(
                                    "Luftfeuchtigkeitsgrenzwerte",
                                    &self
                                        .sensor_border
                                        .get(Sensortypes::Luftfeuchtigkeit.get_name().as_str())
                                        .unwrap(),
                                )
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(10, input)),
                            )
                            .push(
                                TextInput::new(
                                    "Temperaturgrenzwerte",
                                    &self
                                        .sensor_border
                                        .get(Sensortypes::Temperatur.get_name().as_str())
                                        .unwrap(),
                                )
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(11, input)),
                            )
                            .push(
                                TextInput::new(
                                    "Lichtgrenzwerte",
                                    &self
                                        .sensor_border
                                        .get(Sensortypes::Licht.get_name().as_str())
                                        .unwrap(),
                                )
                                .size(TEXT_SIZE)
                                .on_input(|input| DetailMessage::FieldUpdated(12, input)),
                            ),
                    )
                    .foot(
                        Row::new()
                            .spacing(10)
                            .padding(5)
                            .width(Length::Fill)
                            .push(
                                Button::new(
                                    Text::new("Zurück")
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
            }
        } else {
            let row = if self.message != DetailMessage::Pending {
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
                        Button::new(Text::new("Licht").size(TEXT_SIZE))
                            .on_press(DetailMessage::SwitchGraph(Sensortypes::Licht)),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Andere Pflanze anzeigen").size(TEXT_SIZE))
                            .on_press(DetailMessage::Load),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Pflanze bearbeiten").size(TEXT_SIZE))
                            .on_press(DetailMessage::OpenModalPlant),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Gruppe bearbeiten").size(TEXT_SIZE))
                            .on_press(DetailMessage::OpenModalGroup),
                    )
                    .spacing(20);
                let time_row = Row::new()
                    .push(
                        Button::new(Text::new("Letzte 6 Stunden").size(TEXT_SIZE))
                            .on_press(DetailMessage::SwitchTime(chrono::Duration::hours(6))),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Letzte 12 Stunden").size(TEXT_SIZE))
                            .on_press(DetailMessage::SwitchTime(chrono::Duration::hours(12))),
                    )
                    .spacing(20)
                    .push(
                        Button::new(Text::new("Gesamt").size(TEXT_SIZE))
                            .on_press(DetailMessage::SwitchTime(chrono::Duration::weeks(100))),
                    )
                    .spacing(20);
                let chart_col = Column::new().push(row).push(container).push(time_row);
                let row = Row::new()
                    .push(detail_column)
                    .push(chart_col)
                    .spacing(20)
                    .align_items(Center);
                row
            } else {
                let mut id_name_column: Column<DetailMessage> = Column::new().push(
                    Row::new()
                        .push(Text::new("ID").size(TEXT_SIZE))
                        .push(Text::new("Name").size(TEXT_SIZE))
                        .spacing(20),
                );
                for id in self.id_names.clone() {
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
                    .push(
                        Button::new(Text::new("Refresh").size(TEXT_SIZE))
                            .on_press(DetailMessage::Load),
                    )
                    .align_items(Center);
                let id_name_scrollable = scrollable::Scrollable::new(id_name_column);
                let column = Column::new()
                    .push(id_name_scrollable)
                    .align_items(Center)
                    .push(row);
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
