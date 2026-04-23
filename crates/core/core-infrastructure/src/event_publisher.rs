use async_trait::async_trait;
use core_domain::CoreError;
use core_domain::EventPublisher;
use shared_kernel::DomainEvent;

pub struct NoopEventPublisher;

impl NoopEventPublisher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventPublisher for NoopEventPublisher {
    async fn publish(&self, _events: &[Box<dyn DomainEvent>]) -> Result<(), CoreError> {
        Ok(())
    }
}
