use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
struct Args {
    /// Display detailed information.
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let Args { verbose } = Args::parse();
    let metadata = what_is_my_ip::Client::get().await?;

    if verbose {
        println!("{metadata}");
    } else {
        println!("{}", metadata.ip_address);
    }

    Ok(())
}
