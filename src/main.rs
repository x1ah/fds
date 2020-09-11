mod fund;
use std::io::Result;
use clap::{App, SubCommand, Arg, AppSettings};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("search")
                .about("search funds by name")
                .version("1.0.0")
                .arg(
                    Arg::with_name("name").help("name of fund")
                )
        )
        .subcommand(
            SubCommand::with_name("col")
                .about("list of collected funds")
        ).get_matches();

    match matches.subcommand() {
        ("search", Some(arg)) => {
            let name = arg.value_of("name").expect("miss fund name");
            if let Ok(v) = fund::App::new().search(name).await {
                println!("{:?}", v);
            } else {
                println!("Not found {} !", name);
            }
        },
        ("col", Some(_)) => println!("🤪 coming soon"),
        _ => println!("something charred.")
    };
    Ok(())
}
