mod fund;
use std::io::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let resp = fund::App::new()
        .get_detail(String::from("003494"))
        .await
        .unwrap();
    println!("{:?}", resp);

    let v = fund::App::new().search("广发纳斯达克").await;

    println!("{:?}", v);
    Ok(())
}
