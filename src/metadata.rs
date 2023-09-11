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
    pub city: Option<String>,

    /// Country of the client.
    pub country: Option<String>,

    /// ASN of the client.
    pub asn: String,

    /// Colo of the client.
    pub colo: Option<String>,

    /// Timezone of the client.
    pub timezone: Option<String>,

    /// Request time of the client.
    pub request_time: Option<i64>,
}

impl TryFrom<&HeaderMap> for Metadata {
    type Error = Box<dyn Error>;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let latitude = parse_header::<f64>(headers, "latitude")?;
        let longitude = parse_header::<f64>(headers, "longitude")?;

        let ip_address = parse_header::<String>(headers, "ip")?.map_or_else(
            || Ok(None),
            |ip| {
                ip.parse::<IpAddr>()
                    .map(Some)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
            },
        )?;

        let city = parse_header::<String>(headers, "city")?;
        let country = parse_header::<String>(headers, "country")?;
        let asn = parse_header::<String>(headers, "asn").map_or_else(
            |_| "".to_string(),
            |val| val.map_or_else(|| "".to_string(), |asn| format!("AS{}", asn)),
        );
        let colo = parse_header::<String>(headers, "colo")?;
        let timezone = parse_header::<String>(headers, "timezone")?;
        let request_time = parse_header::<i64>(headers, "request-time")?;

        Ok(Self {
            coordinate: (latitude, longitude).into(),
            ip_address,
            city,
            country,
            asn,
            colo,
            timezone,
            request_time,
        })
    }
}

/// Parse a header value into a type.
///
/// # Arguments
///
/// * `headers` - The headers to parse.
/// * `name` - The name of the header to parse. Prefix `cf-meta-` will be added.
///
/// # Returns
///
/// * `Ok(Some(T))` - The parsed value.
/// * `Ok(None)` - The header was not present.
/// * `Err(e)` - The header was present but could not be parsed.
fn parse_header<T>(headers: &HeaderMap, name: &'static str) -> Result<Option<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    headers
        .get(format!("cf-meta-{name}"))
        .map(|v| {
            v.to_str()
                .map_err(|e| Box::new(e) as Box<dyn Error>)?
                .parse::<T>()
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
        .transpose()
}
