#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub id: usize,
    pub latitude: f64,
    pub longitude: f64,
    pub cycle_time: Option<f64>,
    pub is_customer: bool,
    pub is_depo: bool,
    pub is_security_point: bool,
}
impl Point {
    pub fn new_customer(id: usize, latitude: f64, longitude: f64) -> Self {
        Self {
            id,
            latitude,
            longitude,
            cycle_time: None,
            is_customer: true,
            is_depo: false,
            is_security_point: false,
        }
    }
    pub fn new_depo(id: usize, latitude: f64, longitude: f64) -> Self {
        Self {
            id,
            latitude,
            longitude,
            cycle_time: None,
            is_customer: false,
            is_depo: true,
            is_security_point: false,
        }
    }
    pub fn new_security_point(id: usize, latitude: f64, longitude: f64, cycle_time: f64) -> Self {
        Self {
            id,
            latitude,
            longitude,
            cycle_time: Some(cycle_time),
            is_customer: false,
            is_depo: false,
            is_security_point: true,
        }
    }
}