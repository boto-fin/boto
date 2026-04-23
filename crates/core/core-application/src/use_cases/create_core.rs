use core_domain::Core;
use core_domain::CoreId;
use core_domain::CoreRepository;
use core_domain::EventPublisher;

pub struct CreateCoreUseCase<R, E> {
    repo: R,
    events: E,
}

impl<R, E> CreateCoreUseCase<R, E>
where
    R: CoreRepository,
    E: EventPublisher,
{
    pub fn new(repo: R, events: E) -> Self {
        Self { repo, events }
    }

    pub async fn execute(
        &self,
        _cmd: crate::CreateCoreCommand,
    ) -> Result<CoreId, crate::CoreApplicationError> {
        let id = self.repo.next_identity().await?;
        let core = Core::create(id.clone())?;
        self.repo.save(&core).await?;
        self.events.publish(core.uncommitted_events()).await?;
        Ok(id)
    }
}
