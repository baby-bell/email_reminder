use std::collections::BTreeSet;
use std::cmp::Ordering;

use chrono::DateTime;
use chrono::offset::local::Local;

type Time = DateTime<Local>;

#[derive(Debug, PartialEq, Eq)]
struct Event {
    name: String,
    location: Option<String>,
    time: Time,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.time.cmp(&other.time))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

pub struct EventTable {
    events: BTreeSet<Event>,
}