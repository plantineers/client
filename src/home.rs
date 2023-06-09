use crate::detail::Sensortypes;
use crate::graphs::PlantCharts;

use crate::requests::{GraphData, PlantGroupMetadata, PlantMetadata};

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
/// The message of the home page
pub enum HomeMessage {
    /// Open the modal to add a new plant
    OpenModalPlant,
    /// Open the modal to add a new group
    OpenModalGroup,
    /// Close the modal
    CloseModal,
    /// The cancel button was pressed
    CancelButtonPressed,
    /// The ok button was pressed, the data is sent to the server
    OkButtonPressed,
    /// An empty message to do nothing
    Plant,
    /// Deletes the selected group
    DeleteGroup,
    /// Refresh the page
    Refresh,
    /// Change the graphs to the selected sensor
    SwitchGraph(Sensortypes),
    /// Updates the variable to match the input
    FieldUpdated(u8, String),
}

/// The home page
///
/// Fields:
/// - `timerange`: The timerange of the graphs
/// - `selected_group`: The selected group
/// - `group_name_id`: The names and ids of the groups
/// - `show_modal`: If the modal is shown
/// - `modal_is_plant`: If the modal is for a plant
/// - `new_plant`: The data of the new plant
/// - `new_group`: The data of the new group
/// - `additionalCareTips`: The additional care tips of the new plant only for the plant
/// - `sensor_border`: The border of the sensor
/// - `careTips`: The care tips of the new plant for the plant and the group
/// - `group`: The group of the new plant
/// - `charts`: The charts of all groups for the selected sensor
/// - `active_sensor`: The active sensor
/// - `group_ids`: The ids of the groups
/// - `id_names`: The ids and names of the plants
///  - `group_names`: The names of the groups
/// - `sensor_data`: The graph data of the sensors if the sensor was already selected
pub(crate) struct HomePage {
    timerange: (String, String),
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
    group_ids: Vec<String>,
    id_names: Vec<(String, String)>,
    group_names: Vec<String>,
    sensor_data: HashMap<String, (Vec<GraphData>, Vec<String>)>,
}

impl HomePage {
    /// Creates a new home page
    pub fn new() -> Self {
        let vec_chart = Vec::new();
        let charts = PlantCharts::new(vec_chart, HomeMessage::Plant);
        HomePage {
            timerange: (
                "2019-01-01T00:00:00.000Z".to_string(),
                chrono::offset::Local::now()
                    .format("%Y-%m-%dT%H:%M:%S.000Z")
                    .to_string(),
            ),
            selected_group: String::new(),
            group_name_id: Vec::new(),
            show_modal: false,
            modal_is_plant: true,
            new_plant: PlantMetadata::default(),
            additionalCareTips: String::new(),
            careTips: String::new(),
            charts,
            group_names: Vec::new(),
            id_names: Vec::new(),
            active_sensor: Sensortypes::Luftfeuchtigkeit,
            group: String::new(),
            group_ids: Vec::new(),
            new_group: PlantGroupMetadata::default(),
            sensor_border: vec![
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            sensor_data: HashMap::new(),
        }
    }

    /// Handles the messages of the home page
    pub fn update(&mut self, message: HomeMessage) -> Command<HomeMessage> {
        match message {
            HomeMessage::DeleteGroup => {
                return Command::perform(
                    API_CLIENT
                        .get()
                        .unwrap()
                        .clone()
                        .delete_group(self.selected_group.clone())
                        .unwrap_or_else(|e| {
                            info!("Error: {}", e);
                        }),
                    |_| HomeMessage::Refresh,
                )
            }
            HomeMessage::Plant => (),
            HomeMessage::Refresh => {
                self.group_name_id = API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .get_all_group_ids_names()
                    .unwrap();
                self.id_names = API_CLIENT
                    .get()
                    .unwrap()
                    .clone()
                    .get_all_plant_ids_names()
                    .unwrap();
                self.group_ids = self.group_name_id.iter().map(|x| x.0.clone()).collect_vec();
            }
            HomeMessage::SwitchGraph(sensortypes) => {
                self.active_sensor = sensortypes;
                let mut graph_data = vec![];
                if !self
                    .sensor_data
                    .contains_key(sensortypes.get_name().as_str())
                {
                    let data = API_CLIENT
                        .get()
                        .unwrap()
                        .clone()
                        .get_graphs(
                            self.group_ids.clone(),
                            false,
                            sensortypes.get_name(),
                            self.timerange.clone(),
                        )
                        .unwrap();
                    // Collect names from id_names if id is in data
                    self.group_names = self
                        .group_name_id
                        .iter()
                        .filter(|(id, _)| data.iter().any(|(_, i)| i == id))
                        .map(|(_, name)| name.clone())
                        .collect_vec();
                    info!("Group names: {:?}", self.group_names);
                    // Collect graph_data from data and pair with names
                    graph_data = data.iter().map(|(g, _)| g.clone()).collect();
                    self.sensor_data.insert(
                        sensortypes.get_name(),
                        (graph_data.clone(), self.group_names.clone()),
                    );
                } else {
                    info!("Sensor data not in HashMap");
                    let data = self
                        .sensor_data
                        .get(sensortypes.get_name().as_str())
                        .unwrap()
                        .clone();
                    graph_data = data.0;
                    self.group_names = data.1;
                }
                self.charts = PlantCharts::update_charts(
                    &self.charts.clone(),
                    HomeMessage::Plant,
                    graph_data,
                    sensortypes,
                    self.group_names.clone(),
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
                13 => {
                    self.sensor_border[3] = value;
                }
                _ => (),
            },
            HomeMessage::CloseModal => self.show_modal = false,
            HomeMessage::CancelButtonPressed => self.show_modal = false,
            HomeMessage::OkButtonPressed => {
                return if self.modal_is_plant {
                    self.new_plant.additionalCareTips = self
                        .additionalCareTips
                        .split(';')
                        .map(String::from)
                        .collect();
                    self.show_modal = false;
                    Command::perform(
                        API_CLIENT.get().unwrap().clone().create_plant(
                            self.new_plant.clone(),
                            self.group.clone().parse().unwrap_or_default(),
                            None,
                        ),
                        |_| HomeMessage::Refresh,
                    )
                } else {
                    self.new_group.careTips = self.careTips.split(';').map(String::from).collect();
                    for (i, sensor) in enumerate(self.new_group.sensorRanges.iter_mut()) {
                        sensor.max = self.sensor_border.clone()[i]
                            .split(';')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap_or(0);
                        sensor.min = self.sensor_border.clone()[i]
                            .split(';')
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap_or(0);
                    }
                    self.show_modal = false;
                    Command::perform(
                        API_CLIENT
                            .get()
                            .unwrap()
                            .clone()
                            .create_group(self.new_group.clone(), None),
                        |_| HomeMessage::Refresh,
                    )
                };
            }
        }
        Command::none()
    }
}

impl Tab for HomePage {
    type Message = Message;

    /// Sets the title of the page
    fn title(&self) -> String {
        String::from("Dashboard")
    }

    /// Sets the label of the tab
    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Homescreen.into(), self.title())
    }
    /// Sets the content of the page
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
                                .push(Text::new("Die Pflanzengruppen ID kann auf der Startseite eingesehen werden").size(TEXT_SIZE))
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
                                )
                                .push(
                                    TextInput::new("Lichtgrenzwerte", &self.sensor_border[3])
                                        .size(TEXT_SIZE)
                                        .on_input(|input| HomeMessage::FieldUpdated(13, input)),
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
                    Button::new(Text::new("Feuchtigkeit").size(TEXT_SIZE))
                        .on_press(HomeMessage::SwitchGraph(Sensortypes::Feuchtigkeit)),
                )
                .spacing(20)
                .push(
                    Button::new(Text::new("Luftfeuchtigkeit").size(TEXT_SIZE))
                        .on_press(HomeMessage::SwitchGraph(Sensortypes::Luftfeuchtigkeit)),
                )
                .spacing(20)
                .push(
                    Button::new(Text::new("Temperatur").size(TEXT_SIZE))
                        .on_press(HomeMessage::SwitchGraph(Sensortypes::Temperatur)),
                )
                .spacing(20)
                .push(
                    Button::new(Text::new("Licht").size(TEXT_SIZE))
                        .on_press(HomeMessage::SwitchGraph(Sensortypes::Licht)),
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_page_creation() {
        let page = HomePage::new();

        assert_eq!(page.timerange.0, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(page.show_modal, false);
        assert_eq!(page.modal_is_plant, true);
        assert_eq!(page.new_plant, PlantMetadata::default());
        assert_eq!(page.new_group, PlantGroupMetadata::default());
        assert_eq!(page.active_sensor, Sensortypes::Luftfeuchtigkeit);
    }

    #[test]
    fn test_field_updated() {
        let mut page = HomePage::new();
        let index = 0;
        let value = String::from("My plant");

        page.update(HomeMessage::FieldUpdated(index, value.clone()));

        assert_eq!(page.new_plant.name, value);
    }

    #[test]
    fn test_open_modal_plant() {
        let mut page = HomePage::new();

        page.update(HomeMessage::OpenModalPlant);

        assert_eq!(page.show_modal, true);
        assert_eq!(page.modal_is_plant, true);
    }

    #[test]
    fn test_open_modal_group() {
        let mut page = HomePage::new();

        page.update(HomeMessage::OpenModalGroup);

        assert_eq!(page.show_modal, true);
        assert_eq!(page.modal_is_plant, false);
    }

    #[test]
    fn test_close_modal() {
        let mut page = HomePage::new();
        page.show_modal = true;

        page.update(HomeMessage::CloseModal);

        assert_eq!(page.show_modal, false);
    }
}
