use std::{collections::HashMap, fmt::Display, iter::FromIterator, ops::Deref, str::FromStr};

use anyhow::{anyhow, Result};

pub struct Headers {
    inner: HashMap<String, String>,
}

impl Deref for Headers {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromIterator<(String, String)> for Headers {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        Self { inner: iter.into_iter().collect() }
    }
}

impl Headers {
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
