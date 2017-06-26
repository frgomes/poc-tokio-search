extern crate bytes;
extern crate futures;
extern crate httparse;
extern crate base64;
extern crate native_tls;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;
extern crate tokio_proto;
extern crate tokio_service;
extern crate io_dump;

//extern crate clickshop_search;

//use clickshop_search::request;
//use clickshop_search::response;
//use clickshop_search::http;
//use clickshop_search::ebay;



extern crate tokio_minihttp;

use std::io;

use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct StatusService;

impl Service for StatusService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        let (code, message) = match _request.path() {
            "/200" => (200, "OK"),
            "/400" => (400, "Bad Request"),
            "/500" => (500, "Internal Server Error"),
            _ => (404, "Not Found")
        };

        let mut resp = Response::new();
        resp.status_code(code, message);
        resp.body(message);
        future::ok(resp)
    }
}

fn main() {
    //TODO drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    TcpServer::new(Http, addr)
        .serve(|| Ok(StatusService));
}
