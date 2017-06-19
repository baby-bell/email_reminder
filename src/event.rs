use std::cmp::Ordering;
use std::rc::Rc;
use std::collections::{BTreeSet, HashMap, Bound};
use std::collections::btree_set::Range;

use chrono::DateTime;
use chrono::offset::local::Local;

type Time = DateTime<Local>;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Event {
    time: Time,
    name: Rc<String>,
    location: Option<Rc<String>>,
}

impl Event {
    pub fn new(name: String, location: Option<String>, time: Time) -> Self {
        Event {
            name: Rc::new(name),
            location: location.map(Rc::new),
            time: time,
        }
    }
}

pub struct EventTable {
    events: BTreeSet<Event>,
    reminders: HashMap<Event, Vec<Time>>,
}

pub struct EventIter<'a> {
    inner: Range<'a, Event>,
}

impl<'a> Iterator for EventIter<'a> {
    type Item = &'a Event;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl EventTable {
    pub fn new() -> Self {
        EventTable {
            events: BTreeSet::new(),
            reminders: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, e: Event) {
        self.events.insert(e.clone());
        self.reminders.insert(e, vec![]);
    }

    pub fn events_in_date_range<'a>(&'a self, start: Time, end: Time) -> EventIter<'a> {
        let range_start = Bound::Included(Event::new("".into(), None, start));
        let range_end = Bound::Excluded(Event::new("".into(), None, end));

        EventIter {
            inner: self.events.range((range_start, range_end)),
        }
    }
}