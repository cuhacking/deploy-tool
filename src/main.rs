#![feature(proc_macro_hygiene, decl_macro)]

use clap::{App, Arg};
#[macro_use]
extern crate rocket;
use log::LevelFilter;
use rocket::config::{Config, Environment};
use bollard::Docker;
use tokio::runtime::Runtime;

mod deployments;
mod server;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug);

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
    let config = matches.value_of("config").unwrap_or("config.yaml");
    let deploy_config = deployments::read_config(config);

    let port = matches
        .value_of("port")
        .unwrap_or("6601")
        .parse::<u16>()
        .unwrap();

    let config = Config::build(Environment::Staging)
        .port(port)
        .finalize()
        .unwrap();

    let docker = Docker::connect_with_local_defaults().expect("could not connect to docker daemon");

    let mut rt = Runtime::new().unwrap();
    rt.block_on(print_docker_version(&docker));

    rocket::custom(config)
        .manage(deploy_config)
        .mount("/", routes![server::index, server::config])
        .launch();
}

async fn print_docker_version(docker: &Docker) -> () {
    let version = docker.version().await.unwrap();
        println!("Version: {:?}", version);
}