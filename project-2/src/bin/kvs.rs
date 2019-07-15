extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use kvs::KvStore;
use std::process::exit;

fn main() {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .help("Set the value of a string key to a string")
                .arg(Arg::with_name("key").index(1).required(true))
                .arg(Arg::with_name("value").index(2).required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .help("Get the string value of a given string key")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .help("Remove a given key")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .get_matches();

    let mut kv_store = KvStore::new();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let key = _matches.value_of("key").unwrap().to_string();
            let value = _matches.value_of("value").unwrap().to_string();
            kv_store.set(key, value);
            println!("ok");
            exit(1);
        }
        ("get", Some(_matches)) => {
            let key = _matches.value_of("key").unwrap().to_string();
            let value = kv_store.get(key);
            println!("{:?}", value);
            exit(1);
        }
        ("rm", Some(_matches)) => {
            let key = _matches.value_of("key").unwrap().to_string();
            kv_store.remove(key);
            println!("ok");
            exit(1);
        }
        _ => unreachable!(),
    }
}
