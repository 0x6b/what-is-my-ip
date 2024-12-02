use std::{ops::Deref, str::FromStr};

use anyhow::Error;

#[derive(Debug, Clone)]
pub struct TimeZone {
    inner: jiff::tz::TimeZone,
}

impl FromStr for TimeZone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { inner: jiff::tz::TimeZone::get(s)? })
    }
}

impl Deref for TimeZone {
    type Target = jiff::tz::TimeZone;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
