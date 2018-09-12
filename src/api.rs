extern crate hyper;
extern crate futures;
extern crate url;

extern crate serde;
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

pub struct MicroService;

#[derive(Deserialize)]
struct Message {
    #[serde(rename="message")]
    _message: String,
    #[serde(rename="username")]
    _username: String,
}

fn parse_request(chunk: Chunk) -> FutureResult<Message, hyper::Error> {

    let s = str::from_utf8(&chunk);

    if let Err(_) = s {
        make_error_result(
            io::ErrorKind::InvalidInput,
            "Bad body",
        )
    } else {
        let j = serde_json::from_str(s.unwrap());
        if let Err(_) = j {
            make_error_result(
                io::ErrorKind::InvalidInput,
                "Missing one or more required fields",
            )
        } else {
            let res : Message = j.unwrap();
            futures::future::ok(res)
        }
    }

}

fn write_to_db(entry: Message) -> FutureResult<Message, hyper::Error> {
    futures::future::ok(entry)
}

fn make_error_result<T>(error_kind: io::ErrorKind, error_message: &str) -> FutureResult<T, hyper::Error> {
    futures::future::err(
        hyper::Error::from(
            io::Error::new(
                error_kind,
                error_message,
            )
        )
    )
}

fn new_created_response<T>(result: Result<T, hyper::Error>) -> FutureResult<hyper::Response, hyper::Error> {
    match result {
        Ok(_r) => {
            let response = Response::new()
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
    futures::future::ok(response)
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
                        .and_then(parse_request)
                        .and_then(write_to_db)
                        .then(new_created_response);
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
