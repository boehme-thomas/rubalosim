use std::borrow::{Borrow, BorrowMut};
use std::io;
use std::io::{Error, ErrorKind};
use chrono::NaiveTime;
use petgraph::graph::NodeIndex;

#[derive(Clone)]
pub struct Event {
    // necessary?
    id: String,
    // when should this be executed, imaginary time (like 12:50 pm, would be 12:50:00.000)
    time: NaiveTime,
    // relative time to the start of the execution
    relative_time: f64,
    // An event that is part of the enum
    action: Events,

}

impl Event {

    /// Creates a new Event
    pub fn new(id:String, time: NaiveTime, action: Events) -> Event {
        return Event {
            id,
            time,
            relative_time: 0.0,
            action,
        }
    }

    /// Sets relative time of the event. This function is called, when the simulation is executing this event.
    pub fn set_relative_time(&mut self, time: f64) {
        self.relative_time = time;
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_time(&self) -> NaiveTime {
        self.time.clone()
    }

    pub fn get_action(&self) -> Events {
        self.action.clone()
    }

}
#[derive(Clone)]
pub struct EventList {
    pub events: Vec<Event>,
}

impl EventList {
    /// Creates new EventList.
    pub fn new() -> EventList {
        return EventList {
            events: Vec::new(),
        }
    }

    /// Adds event to event queue.
    pub fn add_event(&mut self, event: Event) {
        let len = self.events.len();
        if len == 0 {
            self.events.push(event);
        } else {
            let mut i = 0;
            for old_event in &self.events {
                if old_event.time >= event.time {
                    break;
                }
                i = i + 1;
            }
            if i <= len {
                self.events.insert(i, event);
            } else {
                self.events.push(event);
            }
        }
    }

    pub fn get_eventlist_length(&self) -> usize {
        self.events.len()
    }

    pub fn get_event_list_mut(&mut self) -> &mut [Event] {
        return self.events.borrow_mut()
    }

    pub fn get_event_list(&self) -> &[Event] {
        return self.events.borrow()
    }

    pub fn get_event_list_copy(&self) -> Vec<Event> {
        return self.events.clone();
    }
}

/// List of possible Events.
#[derive(Debug, Clone)]
pub enum Events {
    /// Creation (1st time on the graph) at the node with the specific id.
    Create(NodeIndex),

    /// Movable Object moves to a specific node in the underlying structure.
    Move(NodeIndex),

    /// Move to the last place on the graph and deletion.
    Delete(NodeIndex),

    /// Event for sending Messages in a regular period.
    Message(String),
}

impl Events {
    pub fn get_message(&mut self) -> String {
        return match self {
            Events::Message(message) => message.clone(),
            _ => "".to_string()
        }
    }

    /// Gets the NodeIndex. It's only possible for Create, Move and Delete, if tired with Message a Error will be returned.
    pub fn get_node_index(&mut self) -> Result<NodeIndex, io::Error> {
        return match self {
            Events::Create(index) => Ok(*index),
            Events::Move(index) => Ok(*index),
            Events::Delete(index) => Ok(*index),
            Events::Message(_) => Err(Error::new(ErrorKind::NotFound, "Message does not continue NodeIndex!")),
        }
    }
}

impl ToString for Events {
    fn to_string(&self) -> String {
        return match self {
            Events::Create(node_index) => "Create(".to_owned() + node_index.index().to_string().as_str() + ")",
            Events::Move(node_index) => "Move(".to_owned() + node_index.index().to_string().as_str() + ")",
            Events::Delete(node_index) => "Delete(".to_owned() + node_index.index().to_string().as_str() + ")",
            Events::Message(message) => "Message(".to_owned() + message.as_str() + ")",
        }
    }
}