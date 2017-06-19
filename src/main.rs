extern crate hyper;
extern crate futures;
extern crate lettre;
extern crate chrono;

use futures::future::{self, FutureResult};
use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};
use hyper::header::ContentLength;
use lettre::transport::smtp::{SmtpTransport, SmtpTransportBuilder};

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::cell::Cell;

mod error;
mod event;
use event::EventTable;
use error::Error;

const SETTINGS_FILE: &str = "settings.json";

#[derive(Deserialize)]
struct Settings {
    pub username: String,
    pub password: String,
    pub destination_email: String,
}

struct ReminderService {
    events: Cell<EventTable>,
}

impl ReminderService {
    pub fn handle_post(&self, path: &str) -> FutureResult<Response, hyper::Error> {
        unimplemented!()
    }

    pub fn handle_get(&self, path: &str) -> FutureResult<Response, hyper::Error> {
        let mut path_iter = path.split('/');
        unimplemented!()
    }

    pub fn handle_put(&self, path: &str) -> FutureResult<Response, hyper::Error> {
        unimplemented!()
    }

    pub fn handle_delete(&self, path: &str) -> FutureResult<Response, hyper::Error> {
        unimplemented!()
    }
}

impl Service for ReminderService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response: Response = Response::new();
        let path = req.path();

        match req.method() {
            &Method::Get => self.handle_get(path),
            &Method::Put => self.handle_put(path),
            &Method::Post => self.handle_post(path),
            &Method::Delete => self.handle_delete(path),
            _ => unimplemented!(),
        }
    }
}

fn main() {
}

fn read_settings() -> Result<Settings, Error> {
    let mut buf = String::new();
    let mut settings_file = BufReader::new(File::open(SETTINGS_FILE)?);
    settings_file.read_to_string(&mut buf)?;
    let result: Settings = serde_json::from_str(&buf)?;
    Ok(result)
}

fn connect_to_email(settings: &Settings) -> Result<SmtpTransport, Error> {
    let builder = SmtpTransportBuilder::new("smtp.gmail.com:587")?
        .encrypt()
        .credentials(&settings.username, &settings.password);
    Ok(builder.build())
}
