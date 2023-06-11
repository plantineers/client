use crate::detail::Sensortypes;
use crate::requests::GraphData;
use crate::TEXT_SIZE;
use iced::{Element, Length};
use itertools::{enumerate, Itertools};
use plotters::chart::SeriesLabelPosition;
use plotters::element::PathElement;
use plotters::prelude::RGBColor;
use plotters::series::LineSeries;
use plotters::style::{Color, IntoFont, BLACK, BLUE, WHITE};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

#[derive(Debug, Clone, PartialEq)]
/// A chart that can be drawn
///
/// Fields:
/// - `name`: The name of the chart
/// - `x`: The x values of the chart
/// - `y`: The y values of the chart
/// - `color`: The color of the chart
pub struct PlantChart {
    pub name: String,
    pub x: Vec<i32>,
    pub y: Vec<i32>,
    color: RGBColor,
}
impl PlantChart {
    /// Create a new PlantChart
    pub fn new(name: String, x: Vec<i32>, y: Vec<i32>, color: RGBColor) -> PlantChart {
        PlantChart { name, x, y, color }
    }
    /// Create a test PlantChart
    pub fn test() -> PlantChart {
        PlantChart {
            name: String::from("Test"),
            x: vec![0, 0, 0, 0, 0, 0],
            y: vec![0, 1, 2, 3, 4, 5],
            color: BLUE,
        }
    }
    /// Get the color of the chart
    pub fn get_color(&self) -> RGBColor {
        self.color
    }
}
impl Default for PlantChart {
    /// Create a default PlantChart
    fn default() -> Self {
        Self {
            name: String::new(),
            x: Vec::new(),
            y: Vec::new(),
            color: BLUE,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
/// A collection of PlantCharts
///
/// Fields:
/// - `charts`: The charts
/// - `message`: The message that is passed to the charts, depending on the page it is used in
pub struct PlantCharts<M> {
    pub charts: Vec<PlantChart>,
    pub message: M,
}

impl<M: 'static> PlantCharts<M> {
    /// Create a new PlantCharts object
    pub fn new(charts: Vec<PlantChart>, message: M) -> PlantCharts<M> {
        PlantCharts { charts, message }
    }
    /// Create a test PlantCharts object
    pub fn test(message: M) -> PlantCharts<M> {
        PlantCharts {
            charts: vec![PlantChart::test()],
            message,
        }
    }
    /// Get the largest x and y values of the charts
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
    /// Create the charts from the data
    pub fn create_charts(
        message: M,
        graph_data: Vec<GraphData>,
        sensor: Sensortypes,
        name: Vec<String>,
    ) -> PlantCharts<M> {
        let mut charts = Vec::new();
        for (i, data) in enumerate(&graph_data) {
            let chart = PlantChart::new(
                format!("{}-{}", name[i], sensor),
                (0..data.timestamps.len() as i32).collect_vec(),
                data.values.clone(),
                sensor.get_color_with_random_offset(),
            );
            charts.push(chart);
        }
        PlantCharts::new(charts, message)
    }
    /// Update the charts with new data
    pub fn update_charts(
        &self,
        message: M,
        graph_data: Vec<GraphData>,
        sensor: Sensortypes,
        name: Vec<String>,
    ) -> PlantCharts<M> {
        PlantCharts::<M>::create_charts(message, graph_data, sensor, name)
    }
}

impl<M: 'static + Clone> Chart<M> for PlantCharts<M> {
    type State = ();
    /// Build the chart
    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        //Change background color
        let mut chart = builder
            .caption("Pflanzengraphen", ("sans-serif", TEXT_SIZE).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..self.largest_x_y().0, 0..self.largest_x_y().1)
            .unwrap();
        chart
            .configure_mesh()
            .bold_line_style(BLACK.mix(0.3))
            .light_line_style(BLACK.mix(0.3))
            .axis_style(BLACK.mix(0.5))
            .draw()
            .expect("failed to draw mesh");

        for plantchart in self.charts.iter() {
            let color = plantchart.get_color();
            chart
                .draw_series(
                    LineSeries::new(
                        plantchart
                            .x
                            .iter()
                            .zip(plantchart.y.iter())
                            .map(|(x, y)| (*x, *y)),
                        &color,
                    )
                    .point_size(2),
                )
                .unwrap()
                .label(plantchart.name.as_str())
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }
        chart
            .configure_series_labels()
            .legend_area_size(50)
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .position(SeriesLabelPosition::UpperLeft)
            .label_font(("sans-serif", TEXT_SIZE).into_font())
            .draw()
            .unwrap();
    }
}

impl<M: 'static + Clone> PlantCharts<M> {
    /// Shows the chart
    fn view(&self) -> Element<'_, M> {
        ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotters::style::RED;

    #[test]
    fn test_plant_chart_new() {
        let chart = PlantChart::new("Test".to_string(), vec![1, 2, 3], vec![4, 5, 6], RED);
        assert_eq!(chart.name, "Test");
        assert_eq!(chart.x, vec![1, 2, 3]);
        assert_eq!(chart.y, vec![4, 5, 6]);
        assert_eq!(chart.get_color(), RED);
    }

    #[test]
    fn test_plant_chart_test() {
        let chart = PlantChart::test();
        assert_eq!(chart.name, "Test");
        assert_eq!(chart.x, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(chart.y, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(chart.get_color(), BLUE);
    }

    #[test]
    fn test_plant_charts_new() {
        let message = "Message".to_string();
        let chart1 = PlantChart::new("Test1".to_string(), vec![1, 2, 3], vec![4, 5, 6], RED);
        let chart2 = PlantChart::new("Test2".to_string(), vec![1, 2, 3], vec![4, 5, 6], BLUE);
        let charts = PlantCharts::new(vec![chart1, chart2], message.clone());
        assert_eq!(charts.charts.len(), 2);
        assert_eq!(charts.message, message);
    }

    #[test]
    fn test_plant_charts_test() {
        let message = "Message".to_string();
        let charts = PlantCharts::test(message.clone());
        assert_eq!(charts.charts.len(), 1);
        assert_eq!(charts.message, message);
    }

    #[test]
    fn test_largest_x_y() {
        let chart1 = PlantChart::new("Test1".to_string(), vec![1, 2, 3], vec![4, 5, 6], RED);
        let chart2 = PlantChart::new("Test2".to_string(), vec![7, 8, 9], vec![10, 11, 12], BLUE);
        let charts = PlantCharts::new(vec![chart1, chart2], "Message".to_string());
        assert_eq!(charts.largest_x_y(), (9, 12));
    }
}
