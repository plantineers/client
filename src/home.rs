use crate::detail::Sensortypes;
use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{create_plant, get_all_plant_ids, get_graphs, GraphData, PlantMetadata};
use crate::{Icon, Message, MyStylesheet, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{theme, Element, Length, Renderer};
use iced_aw::{Card, Modal, TabLabel};
use iced_core::Alignment::Center;
use itertools::Itertools;
use plotters_iced::ChartWidget;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    OpenModal,
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
    new_plant: PlantMetadata,
    additionalCareTips: String,
    group: String,
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
            show_modal: false,
            new_plant: PlantMetadata::default(),
            additionalCareTips: String::new(),
            charts,
            active_sensor: Sensortypes::Luftfeuchtigkeit,
            group: String::new(),
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
            HomeMessage::OpenModal => {
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
                _ => (),
            },
            HomeMessage::CloseModal => self.show_modal = false,
            HomeMessage::CancelButtonPressed => self.show_modal = false,
            HomeMessage::OkButtonPressed => {
                self.new_plant.additionalCareTips = self
                    .additionalCareTips
                    .split(',')
                    .map(String::from)
                    .collect();
                self.show_modal = false;
                let _ = create_plant(self.new_plant.clone(), self.group.clone().parse().unwrap());
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
            let container: Container<HomeMessage> = Container::new(Text::new("Neue Pflanze"))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center);
            let content: Element<'_, HomeMessage> = Modal::new(self.show_modal, container, || {
                Card::new(
                    Text::new("Neue Pflanze").horizontal_alignment(Horizontal::Center),
                    Column::new()
                        .push(
                            TextInput::new("Pflanzenname", &self.new_plant.name)
                                .on_input(|input| HomeMessage::FieldUpdated(0, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new("Beschreibung der Pflanze", &self.new_plant.description)
                                .on_input(|input| HomeMessage::FieldUpdated(1, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new("Position der Pflanze", &self.new_plant.location)
                                .on_input(|input| HomeMessage::FieldUpdated(2, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new("Pflegehinweise", &self.additionalCareTips)
                                .on_input(|input| HomeMessage::FieldUpdated(3, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new("Pflanzenspecies", &self.new_plant.species)
                                .on_input(|input| HomeMessage::FieldUpdated(4, input)),
                        )
                        .spacing(20)
                        .push(
                            TextInput::new("Pflanzengruppe", &self.group)
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
                                Text::new("Cancel").horizontal_alignment(Horizontal::Center),
                            )
                            .width(Length::Fill)
                            .on_press(HomeMessage::CancelButtonPressed),
                        )
                        .push(
                            Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
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
            let chart_widget = ChartWidget::new(self.charts.clone());
            let container: Container<HomeMessage> = Container::new(chart_widget)
                .style(theme::Container::Custom(Box::new(MyStylesheet)))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y();
            let row = Row::new().push(container).push(
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
                Row::new().push(Button::new("Add Plant").on_press(HomeMessage::OpenModal));
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
