#![feature(proc_macro_hygiene, decl_macro)]

use clap::{App, Arg};
#[macro_use]
extern crate rocket;
use rocket::config::{Config, Environment};

mod server;

fn main() {
    let matches = App::new("deploy-tool")
        .version("0.1.0")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .value_name("PORT")
                .help("Sets a custom port for the tool to listen on")
                .takes_value(true),
        )
        .get_matches();
    let config = matches.value_of("config").unwrap_or("default.conf");
    let port = matches
        .value_of("port")
        .unwrap_or("6601")
        .parse::<u16>()
        .unwrap();

    println!("Value for config: {}", config);

    println!("Hello, world!");

    let config = Config::build(Environment::Staging)
        .port(port)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![server::index])
        .launch();
}
