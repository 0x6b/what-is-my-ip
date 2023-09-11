pub mod coordinate;
pub mod metadata;

use std::error::Error;

use reqwest::blocking::Client;

use crate::metadata::Metadata;

/// WhatIsMyIpClient is a client to get metadata from Cloudflare.
#[derive(Default)]
pub struct WhatIsMyIpClient {
    client: Client,
}

impl WhatIsMyIpClient {
    pub fn new_with_client(client: Client) -> WhatIsMyIpClient {
        WhatIsMyIpClient { client }
    }

    pub fn get(&self) -> Result<Metadata, Box<dyn Error>> {
        Metadata::try_from(
            self.client
                .get("https://speed.cloudflare.com/__down?bytes=0")
                .send()?
                .headers(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::WhatIsMyIpClient;

    #[test]
    fn test() {
        let metadata = WhatIsMyIpClient::default().get().unwrap();
        println!("Metadata: {metadata:#?}");
    }
}
