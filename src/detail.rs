use crate::graphs::PlantChart;
use crate::ExampleMessage;
use iced::widget::{container, row, Button, Column, Container, Row, Text};
use iced::{Element, Length};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

pub(crate) struct DetailPage;

impl<Message> Chart<Message> for DetailPage {
    type State = ();
    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
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
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .position(SeriesLabelPosition::UpperLeft)
            .label_font("Hectic")
            .draw()
            .unwrap();
    }
}

impl DetailPage {
    pub(crate) fn view(&self) -> Element<ExampleMessage> {
        let chart: Element<ExampleMessage> = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        let text = Text::new("This is the Detail Page");
        let graphs = row![chart, text];
        container(graphs)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
