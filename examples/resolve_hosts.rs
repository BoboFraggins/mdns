use mdns::Error;
use std::time::Duration;

const SERVICE_NAME: &'static str = "_http._tcp.local";
const HOSTS: [&'static str; 2] = ["server1._http._tcp.local", "server2._http._tcp.local"];

#[async_std::main]
async fn main() -> Result<(), Error> {
    let responses = mdns::resolve::multiple(SERVICE_NAME, &HOSTS, Duration::from_secs(15),
                                            mdns::ResponseType::Multicast).await?;

    for response in responses {
        if let (Some(host), Some(ip)) = (response.hostname(), response.ip_addr()) {
            println!("found host {} at {}", host, ip)
        }
    }

    Ok(())
}
