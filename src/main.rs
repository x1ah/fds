mod fund;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let resp = fund::App::new()
        .get_detail(String::from("003494"))
        .await
        .unwrap();
    println!("{:?}", resp);

    let v = fund::App::new().search("富国天").await;

    println!("{:?}", v);
    Ok(())
}
