use anyhow::Result;

use crate::Metadata;

/// A client to get metadata from Cloudflare.
#[derive(Default)]
pub struct Client {}

impl Client {
    pub fn get() -> Result<Metadata> {
        let response = ureq::get("https://speed.cloudflare.com/__down?bytes=0").call()?;

        Metadata::try_from(
            &response
                .headers_names()
                .iter()
                .filter(|k| k.as_str().starts_with("cf-"))
                .map(|k| (k.to_string(), response.header(k).unwrap().to_string()))
                .collect::<_>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[test]
    fn test() {
        let metadata = Client::get().unwrap();
        assert!(metadata.ip_address.is_some());
    }
}
