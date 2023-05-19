use crate::detail::{DetailMessage, DetailPage};
use crate::Message;
use color_eyre::owo_colors::OwoColorize;
use iced::widget::Container;
use iced::{Element, Length};
use itertools::enumerate;
use plotters::chart::SeriesLabelPosition;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::PathElement;
use plotters::prelude::RGBColor;
use plotters::series::LineSeries;
use plotters::style::{Color, FontTransform, IntoFont, ShapeStyle, BLACK, BLUE, GREEN, RED, WHITE};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

pub struct PlantChart {
    pub name: String,
    pub x: Vec<i32>,
    pub y: Vec<i32>,
    color: RGBColor,
}
impl PlantChart {
    pub fn new(name: String, x: Vec<i32>, y: Vec<i32>, color: RGBColor) -> PlantChart {
        PlantChart { name, x, y, color }
    }
    pub fn test() -> PlantChart {
        PlantChart {
            name: String::from("Test"),
            x: vec![0, 1, 2, 3, 4, 5],
            y: vec![0, -1, 10, 3, 4, 5],
            color: BLUE,
        }
    }
    pub fn get_color(&self) -> RGBColor {
        self.color.clone()
    }
}
pub struct PlantCharts<M> {
    pub charts: Vec<PlantChart>,
    pub message: M,
}

impl<M: 'static> PlantCharts<M> {
    pub fn new(charts: Vec<PlantChart>, message: M) -> PlantCharts<M> {
        PlantCharts { charts, message }
    }
    pub fn test(message: M) -> PlantCharts<M> {
        PlantCharts {
            charts: vec![PlantChart::test()],
            message,
        }
    }
    pub fn largest_x_y(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        for chart in self.charts.iter() {
            for (i, j) in chart.x.iter().zip(chart.y.iter()) {
                if *i > x {
                    x = *i;
                }
                if *j > y {
                    y = *j;
                }
            }
        }
        (x, y)
    }
}

impl<M: 'static + Clone> Chart<M> for PlantCharts<M> {
    type State = ();
    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .caption("Plant Charts", ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..self.largest_x_y().0, 0..self.largest_x_y().1)
            .unwrap();
        for plantchart in self.charts.iter() {
            let color = plantchart.get_color();
            chart
                .draw_series(LineSeries::new(
                    plantchart
                        .x
                        .iter()
                        .zip(plantchart.y.iter())
                        .map(|(x, y)| (*x, *y)),
                    &color,
                ))
                .unwrap()
                .label(plantchart.name.as_str())
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }
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

impl<M: 'static + Clone> PlantCharts<M> {
    fn view(&self) -> Element<'_, M> {
        ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
