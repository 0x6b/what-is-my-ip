use what_is_my_ip::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Client::get()?);

    Ok(())
}
