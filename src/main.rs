fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", what_is_my_ip::Client::get()?);

    Ok(())
}
