extern crate hyper;
extern crate futures;
extern crate url;

#[macro_use]
extern crate slog;
extern crate slog_json;

use slog::Drain;
use std::sync::Mutex;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod api;
mod env;

fn main() {

    let root = slog::Logger::root(
        Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );

    info!(root, "logger successfully initialized");
    info!(root, "initializing environment");

    let environment = match env::env::get() {
        Ok(res) => res,
        Err(e) => {
            error!(root, "unable to load environment: {}", e);
            std::process::exit(1);
        },
    };

    let listen_to = format!("{}:{}", environment.hostname, environment.port);

    info!(root, "environment successfully initialized");
    info!(root, "initializing routes");

    let address = listen_to.parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(api::MicroService {}))
        .unwrap();

    info!(root, "routes successfully initialized");
    info!(root, "server running at: {}", listen_to);

    server.run().unwrap();

}
