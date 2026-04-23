use async_trait::async_trait;
use core_domain::{Core, CoreError, CoreId, CoreRepository};

pub struct NoopCoreRepository;

impl NoopCoreRepository {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopCoreRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CoreRepository for NoopCoreRepository {
    async fn find_by_id(&self, _id: &CoreId) -> Result<Option<Core>, CoreError> {
        Ok(None)
    }

    async fn save(&self, _core: &Core) -> Result<(), CoreError> {
        Ok(())
    }

    async fn next_identity(&self) -> Result<CoreId, CoreError> {
        Ok(CoreId::new())
    }
}
