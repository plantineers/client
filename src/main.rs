mod api;
mod graphs;
mod login;

use crate::graphs::PlantChart;
use iced::widget::vertical_slider::draw;
use iced::{Element, Length, Sandbox, Settings};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

pub fn main() -> iced::Result {
    Plantbuddy::run(Settings::default())
}

struct Plantbuddy;
impl<Message> Chart<Message> for Plantbuddy {
    type State = ();
    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        let first_chart: PlantChart = PlantChart::new(vec![1, 2, 3, 4], vec![8, 2, 3, 4]);
        let mut chart = builder
            .build_cartesian_2d(
                0..*first_chart.x.last().unwrap(),
                0..*first_chart.y.last().unwrap(),
            )
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                first_chart
                    .x
                    .iter()
                    .zip(first_chart.y.iter())
                    .map(|(x, y)| (*x, *y)),
                &RED,
            ))
            .unwrap()
            .label("First Chart")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
        chart.configure_mesh().draw().expect("failed to draw mesh");
        chart
            .configure_series_labels()
            .legend_area_size(50)
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .position(SeriesLabelPosition::UpperLeft)
            .label_font("Hectic")
            .draw()
            .unwrap();
    }
}
impl Sandbox for Plantbuddy {
    type Message = ();

    fn new() -> Plantbuddy {
        Plantbuddy
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&self) -> Element<Self::Message> {
        ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
