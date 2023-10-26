#[derive(Clone, Copy)]
pub struct Vehicle {
    pub capacity: f64,
    pub speed: f64,
}
impl Vehicle {
    pub fn new(capacity: f64, speed: f64) -> Self {
        Self { capacity, speed }
    }
}
