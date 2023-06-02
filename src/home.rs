use crate::detail::Sensortypes;
use crate::graphs::{PlantChart, PlantCharts};

use crate::requests::{GraphData, PlantGroupMetadata, PlantMetadata};
use crate::Message::Home;
use crate::{Icon, Message, MyStylesheet, Tab, API_CLIENT, TEXT_SIZE};
use iced::alignment::{Horizontal, Vertical};
use iced::futures::TryFutureExt;
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{theme, Command, Element, Length, Renderer};
use iced_aw::{Card, Modal, TabLabel};
use iced_core::Length::FillPortion;
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
    DeleteGroup,
    Refresh,
    SwitchGraph(Sensortypes),
    FieldUpdated(u8, String),
}

pub(crate) struct HomePage {
    selected_group: String,
    group_name_id: Vec<(String, String)>,
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
        let vec_chart = Vec::new();
        let charts = PlantCharts::new(vec_chart, HomeMessage::Plant);
        HomePage {
            selected_group: String::new(),
            group_name_id: Vec::new(),
            show_modal: false,
            modal_is_plant: true,
            new_plant: PlantMetadata::default(),
            additionalCareTips: String::new(),
            careTips: String::new(),
            charts,
            active_sensor: Sensortypes::Luftfeuchtigkeit,
            group: String::new(),
            ids: Vec::new(),
            new_group: PlantGroupMetadata::default(),
            sensor_border: vec!["".to_string(), "".to_string(), "".to_string()],
            sensor_data: HashMap::new(),
        }
    }

    pub fn update(&mut self, message: HomeMessage) -> Command<HomeMessage> {
        match message {
            HomeMessage::DeleteGroup => {
                API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .delete_group(self.selected_group.clone())
                    .unwrap_or_else(|e| {
                        info!("Error: {}", e);
                    });
            }
            HomeMessage::Plant => (),
            HomeMessage::Refresh => {
                let group_ids_name = API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .get_all_group_ids_names()
                    .unwrap();
                let ids_name = API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .get_all_plant_ids_names()
                    .unwrap();
                self.ids = ids_name.iter().map(|x| x.0.clone()).collect_vec();
                self.group_name_id = group_ids_name;
            }
            HomeMessage::SwitchGraph(sensortypes) => {
                self.active_sensor = sensortypes;
                let mut graph_data = vec![];
                if !self
                    .sensor_data
                    .contains_key(sensortypes.get_name().as_str())
                {
                    graph_data = API_CLIENT
                        .get()
                        .unwrap()
                        .clone()
                        .get_graphs(self.ids.clone(), sensortypes.get_name())
                        .unwrap();
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
                12 => {
                    self.selected_group = value;
                }
                _ => (),
            },
            HomeMessage::CloseModal => self.show_modal = false,
            HomeMessage::CancelButtonPressed => self.show_modal = false,
            HomeMessage::OkButtonPressed => {
                if self.modal_is_plant {
                    self.new_plant.additionalCareTips = self
                        .additionalCareTips
                        .split(';')
                        .map(String::from)
                        .collect();
                    self.show_modal = false;
                    return Command::perform(
                        // TODO: Don't unwrap the group but give feedback to the user
                        API_CLIENT.get().unwrap().clone().create_plant(
                            self.new_plant.clone(),
                            self.group.clone().parse().unwrap(),
                            None,
                        ),
                        |_| HomeMessage::Refresh,
                    );
                } else {
                    self.new_group.careTips = self.careTips.split(';').map(String::from).collect();
                    for (i, sensor) in enumerate(self.new_group.sensorRanges.iter_mut()) {
                        sensor.max = self.sensor_border.clone()[i]
                            .split(';')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap();
                        sensor.min = self.sensor_border.clone()[i]
                            .split(';')
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap();
                    }
                    self.show_modal = false;
                    return Command::perform(
                        API_CLIENT
                            .get()
                            .unwrap()
                            .clone()
                            .create_group(self.new_group.clone(), None),
                        |_| HomeMessage::Refresh,
                    );
                }
            }
        }
        Command::none()
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
                                .push(Text::new("Die Hinweise werden mit einem ';' getrennt").size(TEXT_SIZE))
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
                                .push(Text::new("Die Pflanzengruppen ID kann auf der Startseite eingsehen werden").size(TEXT_SIZE))
                                .spacing(20)
                                .push(
                                    TextInput::new("PflanzengruppenID", &self.group)
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
                                    Text::new("Die Grenzen werden so eingetragen: max;min")
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
                .height(Length::Fill)
                .width(Length::Fill)
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
            let mut group_column: Column<HomeMessage> = Column::new().push(
                Text::new("Gruppen")
                    .size(TEXT_SIZE)
                    .horizontal_alignment(Horizontal::Left),
            );
            for group in self.group_name_id.iter() {
                group_column = group_column.push(
                    Text::new(format!("{}: {}", group.0, group.1))
                        .size(TEXT_SIZE)
                        .horizontal_alignment(Horizontal::Center),
                );
            }
            let delete_row = Row::new()
                .push(
                    TextInput::new("GruppenId", &self.selected_group)
                        .size(TEXT_SIZE)
                        .on_input(|input| HomeMessage::FieldUpdated(12, input)),
                )
                .push(
                    Button::new(Text::new("Gruppe löschen").size(TEXT_SIZE))
                        .on_press(HomeMessage::DeleteGroup),
                );
            group_column = group_column.push(delete_row);
            let row = Row::new()
                .push(group_column.width(FillPortion(1)))
                .push(column.width(FillPortion(3)));
            let content: Element<'_, HomeMessage> = Container::new(row)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into();
            content.map(Message::Home)
        }
    }
}
