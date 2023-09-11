use what_is_my_ip::WhatIsMyIpClient;

fn main() {
    let metadata = WhatIsMyIpClient::default().get().unwrap();
    println!("Metadata: {metadata:#?}");
}
