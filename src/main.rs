mod fund;
use std::io::Result;
use std::time::SystemTime;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("now: {}", now);

    let resp = fund::App::new()
        .get_detail(String::from("519983"))
        .await
        .unwrap();
    println!("{:?}", resp);
    Ok(())
}
