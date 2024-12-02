use std::net::IpAddr;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
struct Args {
    /// Display detailed information.
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let Args { verbose } = Args::parse();
    let metadata = what_is_my_ip::Client::get()?;

    if verbose {
        println!("{metadata}");
    } else {
        println!("{}", metadata.ip_address.unwrap_or(IpAddr::from([0, 0, 0, 0])));
    }

    Ok(())
}
