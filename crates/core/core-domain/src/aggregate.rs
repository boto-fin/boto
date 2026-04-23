use crate::{CoreError, CoreId};
use shared_kernel::DomainEvent;

#[derive(Debug)]
pub struct Core {
    id: CoreId,
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
}

impl Core {
    pub fn create(id: CoreId) -> Result<Self, CoreError> {
        Ok(Self {
            id,
            uncommitted_events: Vec::new(),
        })
    }

    pub fn id(&self) -> &CoreId {
        &self.id
    }

    pub fn uncommitted_events(&self) -> &[Box<dyn DomainEvent>] {
        &self.uncommitted_events
    }

    pub fn clear_events(&mut self) {
        self.uncommitted_events.clear();
    }
}
