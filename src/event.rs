use chrono::DateTime;
use chrono::offset::local::Local;

type Time = DateTime<Local>;

struct Event {
    name: String,
    location: Option<String>,
    time: Time,
}

pub struct EventTable {
    events: Vec<Event>,
}