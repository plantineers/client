use crate::detail::{DetailPlant, Sensortypes};
use crate::graphs::{PlantChart, PlantCharts};
use crate::requests::{get_all_plant_ids, get_graphs, get_plant_details, GraphData};
use crate::{Icon, Message, Tab};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, Button, Column, Container, Row, Text};
use iced::{Application, Command, Element, Length, Sandbox, Settings};
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
        let charts = PlantCharts::new(vec_chart, HomeMessage::Plant);
        HomePage { charts }
    }

    pub fn update(&mut self, message: HomeMessage) {
        match message {
            HomeMessage::Plant => (),
            HomeMessage::Refresh => (),
            HomeMessage::SwitchGraph(sensortypes) => {
                let ids = get_all_plant_ids().unwrap();
                let graph_data: Vec<GraphData> = get_graphs(ids, sensortypes.get_name()).unwrap();
                self.charts =
                    PlantCharts::update_charts(self.charts, PlantCharts::Plant, graph_data)
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
        let button: Button<HomeMessage> =
            Button::new("Refresh").on_press(HomeMessage::Refresh).into();
        let charts = self.charts.clone();
        let chart_widget = ChartWidget::new(charts);
        let row = row!(text, chart_widget, button);
        let content: Element<'_, HomeMessage> = Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();
        content.map(Message::Home)
    }
}
