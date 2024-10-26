use crate::data::{Ticket, TicketId, TicketStatus};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, title: String, description: String) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id: id,
            title,
            description,
            status: TicketStatus::ToDo,
        };
        
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        self.tickets.get(&id).cloned()
    }
}
