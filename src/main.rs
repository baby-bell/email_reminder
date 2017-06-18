extern crate hyper;
extern crate futures;
extern crate lettre;

use lettre::transport::smtp::{SmtpTransport, SmtpTransportBuilder};

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

mod error;
use error::Error;

const CREDENTIALS_FILE: &str = "credentials.json";

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn authenticate(&self, builder: SmtpTransportBuilder) -> SmtpTransportBuilder {
        builder.credentials(&self.username, &self.password)
    }
}

fn main() {
}

fn connect_to_email() -> Result<SmtpTransport, Error> {
    let mut buf = String::new();
    let mut file = BufReader::new(File::open(CREDENTIALS_FILE)?);
    file.read_to_string(&mut buf)?;
    let creds: Credentials = serde_json::from_str(&buf)?;

    let builder = creds.authenticate(SmtpTransportBuilder::new("smtp.gmail.com:587")?.encrypt());
    Ok(builder.build())
}
