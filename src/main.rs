extern crate hyper;
extern crate futures;
extern crate url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
extern crate serde_json;

use std::error::Error;
use std::io;
use std::str;

use hyper::server::{Request, Response, Service};
use hyper::{Chunk, StatusCode};
use hyper::Method::{Get, Post};
use hyper::header::{ContentLength, ContentType};

use futures::Stream;
use futures::future::{Future, FutureResult};

struct MicroService;

#[derive(Deserialize)]
struct NewMessage {
    message: String,
    username: String,
}

fn parse_form(chunk: Chunk) -> FutureResult<NewMessage, hyper::Error> {

    let s = str::from_utf8(&chunk);

    if let Err(_) = s {

        futures::future::err(hyper::Error::from(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Bad body",
        )))

    } else {

        let j = serde_json::from_str(s.unwrap());

        if let Err(_) = j {

            futures::future::err(hyper::Error::from(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing one or more required fields",
            )))

        } else {

            let res : NewMessage = j.unwrap();

            futures::future::ok(res)

        }

    }

}

fn make_post_response(result: Result<i64, hyper::Error>) -> FutureResult<hyper::Response, hyper::Error> {
    match result {
        Ok(timestamp) => {
            let payload = json!({"timestamp": timestamp}).to_string();
            let response = Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_status(StatusCode::Created)
            ;
            futures::future::ok(response)
        }
        Err(error) => make_error_response(error.description()),
    }
}

fn make_error_response(error_message: &str) -> FutureResult<hyper::Response, hyper::Error> {
    let payload = json!({"error": error_message}).to_string();
    let response = Response::new()
        .with_status(StatusCode::InternalServerError)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    debug!("{:?}", response);
    futures::future::ok(response)
}

fn write_to_db(entry: NewMessage) -> FutureResult<i64, hyper::Error> {
    futures::future::ok(0)
}

impl Service for MicroService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        match (request.method(), request.path()) {
            (&Post, "/") => {
                let future =
                    request
                    .body()
                    .concat2()
                    .and_then(parse_form)
                    .and_then(write_to_db)
                    .then(make_post_response);
                Box::new(future)
            }
            (&Get, "/") => {
                Box::new(futures::future::ok(
                    Response::new().with_status(StatusCode::Ok)
                ))
            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}

fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(MicroService {}))
        .unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}
