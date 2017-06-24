#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate lettre;
extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket::State;

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

fn main() {
    let settings = read_settings().expect("failed to read settings");
    let table = Mutex::new(EventTable::new());
    //email::connect(&settings).expect("could not connect to email provider");
    rocket::ignite().manage(table);
}

fn read_settings() -> Result<Settings, Error> {
    let mut buf = String::new();
    let mut settings_file = BufReader::new(File::open(SETTINGS_FILE)?);
    settings_file.read_to_string(&mut buf)?;
    let result: Settings = serde_json::from_str(&buf)?;
    Ok(result)
}


