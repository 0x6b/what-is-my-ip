#[derive(Debug, Clone, Default)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<(Option<f64>, Option<f64>)> for Coordinate {
    fn from((latitude, longitude): (Option<f64>, Option<f64>)) -> Self {
        Self {
            latitude: latitude.unwrap_or_default(),
            longitude: longitude.unwrap_or_default(),
        }
    }
}
