use crate::detail::{DetailMessage, DetailPage};
use crate::Message;
use iced::widget::Container;
use iced::{Element, Length};
use plotters::chart::SeriesLabelPosition;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{Color, BLACK, RED, WHITE};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

pub static X: [i32; 4] = [1, 2, 3, 4];
pub static Y: [i32; 4] = [2, 4, 6, 8];
pub struct PlantChart<M> {
    pub x: Vec<i32>,
    pub y: Vec<i32>,
    pub message: M,
}

impl<M: 'static> PlantChart<M> {
    pub fn new(x: Vec<i32>, y: Vec<i32>, message: M) -> PlantChart<M> {
        PlantChart { x, y, message }
    }
    pub fn test(message: M) -> PlantChart<M> {
        PlantChart {
            x: X.to_vec(),
            y: Y.to_vec(),
            message,
        }
    }
}

impl<M: 'static + Clone> Chart<M> for PlantChart<M> {
    type State = ();
    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        let first_chart: PlantChart<M> =
            PlantChart::new(vec![1, 2, 3, 4], vec![8, 2, 3, 4], self.message.clone());
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
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .position(SeriesLabelPosition::UpperLeft)
            .label_font("Hectic")
            .draw()
            .unwrap();
    }
}

impl<M: 'static + Clone> PlantChart<M> {
    fn view(&self) -> Element<'_, M> {
        ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
