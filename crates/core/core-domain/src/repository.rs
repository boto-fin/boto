use crate::{Core, CoreError, CoreId};
use async_trait::async_trait;

#[async_trait]
pub trait CoreRepository: Send + Sync {
    async fn find_by_id(&self, id: &CoreId) -> Result<Option<Core>, CoreError>;
    async fn save(&self, core: &Core) -> Result<(), CoreError>;
    async fn next_identity(&self) -> Result<CoreId, CoreError>;
}
