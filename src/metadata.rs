use std::{error::Error, net::IpAddr, str::FromStr};

use reqwest::header::HeaderMap;

use crate::coordinate::Coordinate;

/// Metadata contains the metadata returned by the Cloudflare.
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    /// Coordinate of the client.
    pub coordinate: Coordinate,

    /// IP address of the client.
    pub ip_address: Option<IpAddr>,

    /// City of the client.
    pub city: String,

    /// Country of the client.
    pub country: String,

    /// ASN of the client.
    pub asn: String,

    /// Timezone of the client.
    pub timezone: String,

    /// Request time of the client.
    pub request_time: i64,
}

impl TryFrom<&HeaderMap> for Metadata {
    type Error = Box<dyn Error>;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let coordinate = (
            get_header_value::<f64>(headers, "latitude")?,
            get_header_value::<f64>(headers, "longitude")?,
        )
            .into();
        let ip_address =
            get_header_value_and_process::<String, Option<IpAddr>, _>(headers, "ip", |ip| {
                ip.parse::<IpAddr>().ok()
            })?;
        let city = get_header_value::<String>(headers, "city")?;
        let country = get_header_value::<String>(headers, "country")?;
        let asn = get_header_value_and_process::<String, String, _>(headers, "asn", |asn| {
            format!("AS{}", asn)
        })?;
        let timezone = get_header_value::<String>(headers, "timezone")?;
        let request_time = get_header_value::<i64>(headers, "request-time")?;

        Ok(Self {
            coordinate,
            ip_address,
            city,
            country,
            asn,
            timezone,
            request_time,
        })
    }
}

/// Get a header value.
///
/// # Arguments
///
/// - `headers` - The headers to parse.
/// - `name` - The name of the header to parse.
///
/// # Returns
///
/// The parsed value.
fn get_header_value<'a, T>(headers: &HeaderMap, name: &'a str) -> Result<T, Box<dyn Error>>
where
    T: FromStr + Default,
    <T as FromStr>::Err: Error + 'static,
{
    get_header_value_and_process(headers, name, |x: T| x.into())
}

/// Get a header value and process it with given function.
///
/// # Arguments
///
/// - `headers` - The headers to parse.
/// - `name` - The name of the header to parse.
/// - `processor` - The function to process the parsed value.
///
/// # Returns
///
/// The parsed value.
fn get_header_value_and_process<'a, T, U, F>(
    headers: &HeaderMap,
    name: &'a str,
    processor: F,
) -> Result<U, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
    U: Default,
    F: Fn(T) -> U,
{
    headers
        .get(format!("cf-meta-{name}"))
        .map(|v| {
            v.to_str()
                .map_err(|e| Box::new(e) as Box<dyn Error>)?
                .parse::<T>()
                .map_err(|e| Box::new(e) as Box<dyn Error>)
                .map(|s| processor(s))
        })
        .unwrap_or(Ok(U::default()))
}
