extern crate chrono;
extern crate lettre;
extern crate iron;
#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::middleware::Handler;
use router::Router;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::sync::Mutex;

mod error;
mod event;
mod email;
use event::{Event, EventTable};
use error::Error;

const SETTINGS_FILE: &str = "./settings.json";

#[derive(Deserialize)]
pub struct Settings {
    pub username: String,
    pub password: String,
    pub destination_email: String,
    pub serving_port: u16,
}

struct State {
    table: Mutex<EventTable>,
}

impl State {
    pub fn new() -> Self {
        State {
            table: Mutex::new(EventTable::new()),
        }
    }
}

impl Handler for State {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let dispatcher = req.extensions.get::<Router>().unwrap();
        if let Some(ref name) = dispatcher.find("name") {
            Ok(Response::with((status::Ok,
            serde_json::to_string(&self.table.lock().unwrap().events_by_name(name)).unwrap())))
        } else {
            Ok(Response::with(status::Ok))
        }
    }
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World")))
}

fn main() {
    let state = State::new();
    let router = router! {
        index: get "/" => handler,
        by_name: get "/events/:name/" => state
    };
    let settings = read_settings().expect("failed to read settings");
    //email::connect(&settings).expect("could not connect to email provider");
    Iron::new(router).http("localhost:8000").unwrap();
}

fn read_settings() -> Result<Settings, Error> {
    let mut buf = String::new();
    let mut settings_file = BufReader::new(File::open(SETTINGS_FILE)?);
    settings_file.read_to_string(&mut buf)?;
    let result: Settings = serde_json::from_str(&buf)?;
    Ok(result)
}


