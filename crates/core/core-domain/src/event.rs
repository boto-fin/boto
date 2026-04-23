use crate::CoreId;
use chrono::{DateTime, Utc};
use shared_kernel::DomainEvent;

#[derive(Debug, Clone)]
pub struct CoreCreated {
    pub core_id: CoreId,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for CoreCreated {
    fn event_type(&self) -> &'static str {
        "CoreCreated"
    }

    fn aggregate_id(&self) -> &str {
        self.core_id.as_str()
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn version(&self) -> u64 {
        1
    }
}
