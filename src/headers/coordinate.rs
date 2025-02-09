use std::{fmt, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<(f64, f64)> for Coordinate {
    fn from((latitude, longitude): (f64, f64)) -> Self {
        Self { latitude, longitude }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{latitude}, {longitude} (https://maps.google.com/maps?q={latitude},{longitude})",
            latitude = self.latitude,
            longitude = self.longitude
        )?;
        Ok(())
    }
}
