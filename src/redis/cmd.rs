use std::fmt::Display;
use std::sync::{Arc, mpsc, Mutex};
use std::{fmt, thread};
use uuid::Uuid;
use crate::redis::connection::ManagedConnection;
use crate::redis::pool::{HPool};

pub struct PoolCommand {
    id: Uuid,
    name: String,
    am_pool: Arc<Mutex<HPool>>
}

impl PoolCommand {
    pub fn new(pool: Box<HPool>, name: &str) -> PoolCommand {
        PoolCommand {
            id: Uuid::new_v4(),
            name: String::from(name),
            am_pool: Arc::new(Mutex::new(*pool))
        }
    }

    pub fn execute<F>(&mut self, func: F) where F: Fn(&mut ManagedConnection) {
        let (tx, rx) = mpsc::channel();
        let pool = self.am_pool.clone();
        let cmd_fmt = self.to_string();
        let builder = thread::Builder::new();
        builder.spawn(move || {
            match pool.lock().unwrap().get() {
                Some(xconn) => tx.send(xconn).unwrap(),
                None => println!("No connections available in command {}", cmd_fmt)
            }
        }).unwrap();
        match rx.recv() {
            Ok(mut received) => {
                println!("Executing cmd {}", self);
                func(received.as_mut());
                self.am_pool.lock().unwrap().take_back(received);
            },
            Err(e) => println!("No connection received {}!!!", e)
        }

    }
}

impl Display for PoolCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PoolCommand: [{}@{}]", self.id, self.name)
    }
}
