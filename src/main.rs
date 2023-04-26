use iced::{Element, Length, Sandbox, Settings};
use iced::widget::vertical_slider::draw;
use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;
use plotters_iced::{Chart, ChartWidget, DrawingBackend, ChartBuilder};
pub fn main() -> iced::Result {
    Plantbuddy::run(Settings::default())
}

struct Plantbuddy;
impl<Message> Chart<Message> for Plantbuddy {
    type State = ();
    fn build_chart<DB:DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder.build_cartesian_2d(0.0..10.0, 0.0..10.0).unwrap();
        chart.draw_series(LineSeries::new(
            (0..10).map(|x| (x as f64, x as f64)),
            &RED,
        )).unwrap();
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

    fn view(&self)->Element<Self::Message> {
        ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    }