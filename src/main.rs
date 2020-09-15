mod config;
mod fund;

use clap::{App, AppSettings, Arg, SubCommand};
use config::Config;
use std::io::Result;
use std::path::PathBuf;

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
                .arg(Arg::with_name("name").help("name of fund")),
        )
        .subcommand(
            SubCommand::with_name("l")
                .about("list of collected funds")
                .arg(Arg::with_name("c").short("-c").help("config file path")),
        )
        .get_matches();

    match matches.subcommand() {
        ("search", Some(arg)) => {
            let name = arg.value_of("name").expect("miss fund name");
            if let Ok(v) = fund::App::new().search(name).await {
                for _fund in v.iter() {
                    println!("{}", _fund);
                }
            } else {
                println!("Not found {} !", name);
            }
        }
        ("l", Some(arg)) => {
            let cfg_path = arg.value_of("c");
            let path = match cfg_path {
                Some(v) => Some(PathBuf::from(v)),
                _ => None,
            };
            let cfg = Config::new(path)?;
            let funds = fund::App::new().bulk_get_detail(cfg.funds).await;
            for _fund in funds {
                println!("{}", _fund);
            }
        }
        _ => println!("something charred."),
    };
    Ok(())
}
