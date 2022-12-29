use redis::{Connection};
use uuid::Uuid;
use std::fmt::Display;
use std::{fmt};

pub struct ManagedConnection {
    id: Uuid,
    pub(crate) conn: Box<Connection>,
}

impl ManagedConnection {
    pub fn new(conn: Connection) -> ManagedConnection {
        ManagedConnection {
            id: Uuid::new_v4(),
            conn: Box::new(conn),
        }
    }
}

impl Display for ManagedConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ManagedConnection: [{}]", self.id)
    }
}

