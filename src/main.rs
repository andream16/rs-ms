extern crate hyper;
extern crate futures;
extern crate url;
extern crate env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod api;

fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(api::MicroService {}))
        .unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}
