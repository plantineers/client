use crate::detail::Sensortypes;
use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{
    create_group, create_plant, get_all_plant_ids_names, get_graphs, GraphData, PlantGroupMetadata,
    PlantMetadata,
};
use crate::{Icon, Message, MyStylesheet, Tab, TEXT_SIZE};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{theme, Command, Element, Length, Renderer};
use iced_aw::{Card, Modal, TabLabel};
use itertools::{enumerate, Itertools};
use log::info;
use plotters_iced::ChartWidget;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    OpenModalPlant,
    OpenModalGroup,
    CloseModal,
    CancelButtonPressed,
    OkButtonPressed,
    Plant,
    Refresh,
    SwitchGraph(Sensortypes),
    FieldUpdated(u8, String),
}

pub(crate) struct HomePage {
    show_modal: bool,
    modal_is_plant: bool,
    new_plant: PlantMetadata,
    new_group: PlantGroupMetadata,
    additionalCareTips: String,
    sensor_border: Vec<String>,
    careTips: String,
    group: String,
    charts: PlantCharts<HomeMessage>,
    active_sensor: Sensortypes,
    ids: Vec<String>,
    sensor_data: HashMap<String, Vec<GraphData>>,
}

impl HomePage {
    pub fn new() -> Self {
        let ids_name = get_all_plant_ids_names().unwrap();
        let ids = ids_name.iter().map(|x| x.0.clone()).collect_vec();
        let vec_chart = Vec::new();
        let charts = PlantCharts::new(vec_chart, HomeMessage::Plant);
        HomePage {
            show_modal: false,
            modal_is_plant: true,
            new_plant: PlantMetadata::default(),
            additionalCareTips: String::new(),
            careTips: String::new(),
            charts,
            active_sensor: Sensortypes::Luftfeuchtigkeit,
            group: String::new(),
            ids,
            new_group: PlantGroupMetadata::default(),
            sensor_border: vec!["".to_string(), "".to_string(), "".to_string()],
            sensor_data: HashMap::new(),
        }
    }

    pub fn update(&mut self, message: HomeMessage) {
        match message {
            HomeMessage::Plant => (),
            HomeMessage::Refresh => {
                HomePage::new();
            }
            HomeMessage::SwitchGraph(sensortypes) => {
                self.active_sensor = sensortypes;
                let mut graph_data = vec![];
                if !self
                    .sensor_data
                    .contains_key(sensortypes.get_name().as_str())
                {
                    graph_data = get_graphs(self.ids.clone(), sensortypes.get_name()).unwrap();
                    self.sensor_data
                        .insert(sensortypes.get_name(), graph_data.clone());
                } else {
                    graph_data = self
                        .sensor_data
                        .get(sensortypes.get_name().as_str())
                        .unwrap()
                        .clone();
                }

                self.charts = PlantCharts::update_charts(
                    &self.charts.clone(),
                    HomeMessage::Plant,
                    graph_data.clone(),
                    sensortypes,
                    format!("{}%", self.active_sensor),
                );
            }
            HomeMessage::OpenModalPlant => {
                self.modal_is_plant = true;
                self.show_modal = true;
            }
            HomeMessage::OpenModalGroup => {
                self.modal_is_plant = false;
                self.show_modal = true;
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
                    self.additionalCareTips = value;
                }
                4 => {
                    self.new_plant.species = value;
                }
                5 => {
                    self.group = value;
                }
                6 => {
                    self.new_group.name = value;
                }
                7 => {
                    self.new_group.description = value;
                }
                8 => {
                    self.careTips = value;
                }
                9 => {
                    self.sensor_border[0] = value;
                }
                10 => {
                    self.sensor_border[1] = value;
                }
                11 => {
                    self.sensor_border[2] = value;
                }
                _ => (),
            },
            HomeMessage::CloseModal => self.show_modal = false,
            HomeMessage::CancelButtonPressed => self.show_modal = false,
            HomeMessage::OkButtonPressed => {
                if self.modal_is_plant {
                    self.new_plant.additionalCareTips = self
                        .additionalCareTips
                        .split(',')
                        .map(String::from)
                        .collect();
                    self.show_modal = false;
                    let _ = create_plant(
                        self.new_plant.clone(),
                        self.group.clone().parse().unwrap(),
                        None,
                    );
                } else {
                    self.new_group.careTips = self.careTips.split(',').map(String::from).collect();
                    for (i, sensor) in enumerate(self.new_group.sensorRanges.iter_mut()) {
                        sensor.max = self.sensor_border.clone()[i]
                            .split(',')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap();
                        sensor.min = self.sensor_border.clone()[i]
                            .split(',')
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap();
                    }
                    self.show_modal = false;
                    let _ = create_group(self.new_group.clone(), None);
                }
            }
        }
    }
}

impl Tab for HomePage {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Dashboard")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Homescreen.into(), self.title())
    }
    fn content(&self) -> Element<'_, Self::Message> {
        if self.show_modal {
            if self.modal_is_plant {
                let container: Container<HomeMessage> =
                    Container::new(Text::new("Neue Pflanze").size(TEXT_SIZE))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center);
                let content: Element<'_, HomeMessage> =
                    Modal::new(self.show_modal, container, || {
                        Card::new(
                            Text::new("Neue Pflanze")
                                .size(TEXT_SIZE)
                                .horizontal_alignment(Horizontal::Center),
                            Column::new()
                                .push(
                                    TextInput::new("Pflanzenname", &self.new_plant.name)
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(0, input)),
                                )
                                .spacing(20)
                                .push(
                                    TextInput::new(
                                        "Beschreibung der Pflanze",
                                        &self.new_plant.description,
                                    )
                                    .size(TEXT_SIZE)
                                    .on_input(|input| HomeMessage::FieldUpdated(1, input)),
                                )
                                .spacing(20)
                                .push(
                                    TextInput::new(
                                        "Position der Pflanze",
                                        &self.new_plant.location,
                                    )
                                    .size(TEXT_SIZE)
                                    .on_input(|input| HomeMessage::FieldUpdated(2, input)),
                                )
                                .spacing(20)
                                .push(
                                    TextInput::new("Pflegehinweise", &self.additionalCareTips)
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(3, input)),
                                )
                                .spacing(20)
                                .push(
                                    TextInput::new("Pflanzenspecies", &self.new_plant.species)
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(4, input)),
                                )
                                .spacing(20)
                                .push(
                                    TextInput::new("Pflanzengruppe", &self.group)
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(5, input)),
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
                                    .on_press(HomeMessage::CancelButtonPressed),
                                )
                                .push(
                                    Button::new(
                                        Text::new("Ok")
                                            .size(TEXT_SIZE)
                                            .horizontal_alignment(Horizontal::Center),
                                    )
                                    .width(Length::Fill)
                                    .on_press(HomeMessage::OkButtonPressed),
                                ),
                        )
                        .max_width(300.0)
                        .on_close(HomeMessage::CloseModal)
                        .into()
                    })
                    .backdrop(HomeMessage::CloseModal)
                    .on_esc(HomeMessage::CloseModal)
                    .into();
                content.map(Message::Home)
            } else {
                let container: Container<HomeMessage> =
                    Container::new(Text::new("Neue Gruppe").size(TEXT_SIZE))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center);
                let content: Element<'_, HomeMessage> =
                    Modal::new(self.show_modal, container, || {
                        Card::new(
                            Text::new("Neue Gruppe")
                                .size(TEXT_SIZE)
                                .horizontal_alignment(Horizontal::Center),
                            Column::new()
                                .push(
                                    TextInput::new("Gruppennamen", &self.new_group.name)
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(6, input)),
                                )
                                .spacing(20)
                                .push(
                                    TextInput::new(
                                        "Beschreibung der Gruppe",
                                        &self.new_group.description,
                                    )
                                    .size(TEXT_SIZE)
                                    .on_input(|input| HomeMessage::FieldUpdated(7, input)),
                                )
                                .spacing(20)
                                .push(
                                    TextInput::new("Pflegehinweise", &self.careTips)
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(8, input)),
                                )
                                .spacing(20)
                                .push(
                                    Text::new("Die Grenzen werden so eingetragen: max,min")
                                        .size(TEXT_SIZE),
                                )
                                .push(
                                    TextInput::new(
                                        "Feuchtigkeitsgrenzwerte",
                                        &self.sensor_border[0],
                                    )
                                    .size(TEXT_SIZE)
                                    .on_input(|input| HomeMessage::FieldUpdated(9, input)),
                                )
                                .push(
                                    TextInput::new(
                                        "Luftfeuchtigkeitsgrenzwerte",
                                        &self.sensor_border[1],
                                    )
                                    .size(TEXT_SIZE)
                                    .on_input(|input| HomeMessage::FieldUpdated(10, input)),
                                )
                                .push(
                                    TextInput::new("Temperaturgrenzwerte", &self.sensor_border[2])
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(11, input)),
                                ),
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
                                    .on_press(HomeMessage::CancelButtonPressed),
                                )
                                .push(
                                    Button::new(
                                        Text::new("Ok")
                                            .size(TEXT_SIZE)
                                            .horizontal_alignment(Horizontal::Center),
                                    )
                                    .width(Length::Fill)
                                    .on_press(HomeMessage::OkButtonPressed),
                                ),
                        )
                        .max_width(300.0)
                        .on_close(HomeMessage::CloseModal)
                        .into()
                    })
                    .backdrop(HomeMessage::CloseModal)
                    .on_esc(HomeMessage::CloseModal)
                    .into();
                content.map(Message::Home)
            }
        } else {
            let chart_widget = ChartWidget::new(self.charts.clone());
            let container: Container<HomeMessage> = Container::new(chart_widget)
                .style(theme::Container::Custom(Box::new(MyStylesheet)))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y();
            let row = Row::new()
                .push(
                    Button::new(Text::new("Refresh").size(TEXT_SIZE))
                        .on_press(HomeMessage::Refresh),
                )
                .spacing(20)
                .push(
                    Button::new(Text::new("Temperatur").size(TEXT_SIZE))
                        .on_press(HomeMessage::SwitchGraph(Sensortypes::Temperatur)),
                )
                .spacing(20)
                .push(
                    Button::new(Text::new("Luftfeuchtigkeit").size(TEXT_SIZE))
                        .on_press(HomeMessage::SwitchGraph(Sensortypes::Luftfeuchtigkeit)),
                )
                .spacing(20)
                .push(
                    Button::new(Text::new("Feuchtigkeit").size(TEXT_SIZE))
                        .on_press(HomeMessage::SwitchGraph(Sensortypes::Feuchtigkeit)),
                );
            let lower_row: Row<HomeMessage, Renderer> = Row::new()
                .push(
                    Button::new(Text::new("Neue Pflanze erstellen").size(TEXT_SIZE))
                        .on_press(HomeMessage::OpenModalPlant),
                )
                .spacing(20)
                .push(
                    Button::new(Text::new("Gruppe hinzufügen").size(TEXT_SIZE))
                        .on_press(HomeMessage::OpenModalGroup),
                );
            let column = Column::new().push(row).push(container).push(lower_row);
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
