use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};
use redis::Client;
use crate::redis::connection::ManagedConnection;

pub struct HPool {
    connections: Arc<Mutex<VecDeque<Option<ManagedConnection>>>>,
    size: usize,
    client:  Client
}

impl HPool {
    pub fn new(addr: &str, size: usize) -> Option<HPool> {
        match Client::open(addr) {
            Ok(client) => Some(HPool {
                connections: Arc::new(Mutex::new(HPool::init(size))),
                size,
                client
            }),
            Err(_) => None
        }
    }

    fn init(size: usize) -> VecDeque<Option<ManagedConnection>> {
        let mut conns = VecDeque::with_capacity(size);
        for _i in 1..size {
            conns.push_back(None);
        }
        conns
    }

    fn get_connection(&self, oconn: Option<ManagedConnection>) -> Box<ManagedConnection> {
        match oconn {
            Some(mconn) => Box::new(mconn),
            None => Box::new(ManagedConnection::new(self.client.get_connection().unwrap()))
        }
    }

    pub fn get(&mut self) -> Option<Box<ManagedConnection>> {
        self.connections.lock().unwrap().pop_front().map(|conn_box| self.get_connection(conn_box))
    }

    pub fn take_back(&mut self, conn: Box<ManagedConnection>) {
        if self.connections.lock().unwrap().len() < self.size {
            self.connections.lock().unwrap().push_back(Some(*conn));
        }
    }
}

