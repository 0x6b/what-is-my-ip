use anyhow::Result;
fn main() -> Result<()> {
    println!("{}", what_is_my_ip::Client::get()?);

    Ok(())
}
