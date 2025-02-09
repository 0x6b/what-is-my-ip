use std::{fmt, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct Number {
    inner: u32,
}

impl TryFrom<u32> for Number {
    type Error = anyhow::Error;

    /// AS numbers, or ASNs, are unique 16-bit numbers between 1 and 65,534 or 32-bit numbers
    /// between 131,072 and 4,294,967,294.
    ///
    /// Reference: https://www.cloudflare.com/learning/network-layer/what-is-an-autonomous-system/
    fn try_from(inner: u32) -> Result<Self, Self::Error> {
        match inner {
            n if (1..=65534).contains(&n) => Ok(Self { inner }),
            n if (131072..=4294967294).contains(&n) => Ok(Self { inner }),
            _ => Err(anyhow::anyhow!("Invalid ASN: {inner}")),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AS{number} (https://bgp.tools/as/{number})", number = self.inner)
    }
}
