use std::collections::BTreeMap;
use serde_json::json;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    NewConnection,
    Offer,
    Candidate,
    Hangup
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    pub event: Event,
    pub to: String,
    pub from: String,
    pub data: serde_json::Value
}

impl Message {
    pub fn new(event: Event, to: &str, from: &str) -> Message {
        Message {
            event,
            to: String::from(to),
            from: String::from(from),
            data: json!(null)
        }
    }

    pub fn with_null(mut self) -> Message {
        self.data = json!(null);
        self
    }

    pub fn with_str(mut self, value: &str) -> Message {
        self.data = json!(String::from(value));
        self
    }

    pub fn with_number(mut self, value:  serde_json::value::Number) -> Message {
        self.data = json!(value);
        self
    }

    pub fn with_boolean(mut self, value: bool) -> Message {
        self.data = json!(value);
        self
    }

    pub fn with_object<T>(mut self, value: BTreeMap<String, T>) -> Message where T: Serialize {
        self.data = json!(value);
        self
    }

    pub fn with_vector<T>(mut self, value: Vec<T>) -> Message where T: Serialize {
        self.data = json!(value);
        self
    }
}