use crate::detail::{DetailPlant, Sensortypes};
use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{get_all_plant_ids, get_graphs, get_plant_details, GraphData};
use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Row, Text};
use iced::{Application, Command, Element, Length, Sandbox, Settings};
use iced::theme::Button::Primary;
use iced::Theme::Dark;
use iced_aw::TabLabel;
use itertools::Itertools;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    Plant,
    Refresh,
    SwitchGraph(Sensortypes),
}

pub(crate) struct HomePage {
    charts: PlantCharts<HomeMessage>,
    active_sensor: Sensortypes,
    ids: Vec<String>,
}

impl HomePage {
    pub fn new() -> Self {
        let ids = get_all_plant_ids().unwrap();
        let graph_data: Vec<GraphData> =
            get_graphs(ids, Sensortypes::Luftfeuchtigkeit.get_name()).unwrap();
        let mut vec_chart = Vec::new();
        for data in graph_data {
            vec_chart.push(PlantChart::new(
                "".to_string(),
                (0..data.timestamps.len() as i32).collect_vec(),
                data.values,
                Default::default(),
            ));
        }
        let ids = get_all_plant_ids().unwrap();
        let charts = PlantCharts::new(vec_chart, HomeMessage::Plant);
        HomePage {
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
        let text: Element<'_, HomeMessage> = Text::new("This is the Home Page").into();
        let refresh_button: Button<HomeMessage> =
            Button::new("Refresh").on_press(HomeMessage::Refresh);
        let temp_button: Button<HomeMessage> =
            Button::new("Temperatur").on_press(HomeMessage::SwitchGraph(Sensortypes::Temperatur));
        let hum_button: Button<HomeMessage> = Button::new("Luftfeuchtigkeit")
            .on_press(HomeMessage::SwitchGraph(Sensortypes::Luftfeuchtigkeit));
        let moisture_button: Button<HomeMessage> = Button::new("Feuchtigkeit")
            .on_press(HomeMessage::SwitchGraph(Sensortypes::Feuchtigkeit));
        let charts = self.charts.clone();
        let chart_widget = ChartWidget::new(charts);
        let row = Row::new().push(chart_widget).push(
            Column::new()
                .push(refresh_button)
                .spacing(20)
                .push(temp_button)
                .spacing(20)
                .push(hum_button)
                .spacing(20)
                .push(moisture_button),
        );

        let content: Element<'_, HomeMessage> = Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();
        content.map(Message::Home)
    }
}
