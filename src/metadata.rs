use std::{fmt, fmt::Display, net::IpAddr};

use anyhow::{Error, Result};

use crate::{Asn, Coordinate, Headers, TimeZone};

/// Metadata contains the metadata returned by the Cloudflare.
#[derive(Debug, Clone)]
pub struct Metadata {
    /// Coordinate of the client.
    pub coordinate: Coordinate,

    /// IP address of the client.
    pub ip_address: IpAddr,

    /// City of the client.
    pub city: String,

    /// Country of the client.
    pub country: String,

    /// ASN of the client.
    pub asn: Asn,

    /// Timezone of the client.
    pub time_zone: TimeZone,

    /// Request time of the client.
    pub request_time: i64,
}

impl Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"- IP address: {}
- Coordinate: {}
- City: {}
- Country: {}
- Network: {}
- Time zone: {}"#,
            self.ip_address,
            self.coordinate,
            self.city,
            self.country,
            self.asn,
            self.time_zone.iana_name().unwrap_or("Unknown"),
        )
    }
}

impl TryFrom<&Headers> for Metadata {
    type Error = Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(Self {
            coordinate: Coordinate::from((
                headers.get::<f64>("latitude")?,
                headers.get::<f64>("longitude")?,
            )),
            ip_address: headers.get::<IpAddr>("ip")?,
            city: headers.get::<String>("city")?,
            country: headers.get::<String>("country")?,
            asn: Asn::try_from(headers.get::<u32>("asn")?)?,
            time_zone: headers.get::<TimeZone>("timezone")?,
            request_time: headers.get::<i64>("request-time")?,
        })
    }
}
