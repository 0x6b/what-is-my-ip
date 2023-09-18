#[derive(Debug, Clone, Default)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<(f64, f64)> for Coordinate {
    fn from((latitude, longitude): (f64, f64)) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}
