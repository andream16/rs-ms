extern crate hyper;
extern crate futures;
extern crate url;
extern crate env_logger;

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

fn main() {

    let root = slog::Logger::root(
        Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );

    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(api::MicroService {}))
        .unwrap();

    info!(root, "service running on port: {}", 8080);

    server.run().unwrap();
}
