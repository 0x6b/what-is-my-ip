use std::{collections::HashMap, fmt::Display, iter::FromIterator, ops::Deref, str::FromStr};

use anyhow::{anyhow, Result};

pub mod autonomous_system;
pub mod coordinate;
pub mod time_zone;

pub use autonomous_system::Number as Asn;
pub use coordinate::Coordinate;
pub use time_zone::TimeZone;

pub struct ResponseHeaderMap {
    inner: HashMap<String, String>,
}

impl Deref for ResponseHeaderMap {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromIterator<(String, String)> for ResponseHeaderMap {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        Self { inner: iter.into_iter().collect() }
    }
}

impl ResponseHeaderMap {
    pub fn get<T>(&self, name: &str) -> Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        self.inner
            .get(&format!("cf-meta-{name}"))
            .ok_or_else(|| anyhow!("Header not found"))?
            .parse::<T>()
            .map_err(|e| anyhow!("{e}"))
    }
}
