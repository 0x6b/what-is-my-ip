use anyhow::Result;
use reqwest::get;

use crate::Metadata;

/// A client to get metadata from Cloudflare.
#[derive(Default)]
pub struct Client {}

impl Client {
    pub async fn get() -> Result<Metadata> {
        let response = get("https://speed.cloudflare.com/__down?bytes=0").await?;

        Metadata::try_from(
            &response
                .headers()
                .iter()
                .filter(|(k, _)| k.as_str().starts_with("cf-"))
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                .collect::<_>(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() {
        let metadata = crate::Client::get().await.unwrap();
        assert!(metadata.ip_address.is_ipv4() || metadata.ip_address.is_ipv6());
    }
}
