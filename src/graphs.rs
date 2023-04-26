pub struct PlantChart {
    pub x: Vec<i32>,
    pub y: Vec<i32>,
}

impl PlantChart {
    pub fn new(x: Vec<i32>, y: Vec<i32>) -> PlantChart {
        PlantChart { x, y }
    }
}
