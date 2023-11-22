use crate::entities;
use crate::ports::{repositories::session, services::token_service};

pub struct CreateUseCase<'a> {
    session_repository: &'a dyn session::SessionRepository,
    token_service: &'a dyn token_service::TokenService,
}

pub struct CreateSessionDTO {
    pub user_id: uuid::Uuid,
    pub user_group: i32,
    /// ttl in seconds
    pub refresh_token_ttl: u64,
}

pub struct CreateDTO {
    pub entity: entities::Session,
    pub token: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Repository error [{0}]")]
    RepositoryError(#[from] crate::ports::Error),
}

impl<'a> CreateUseCase<'a> {
    pub fn new(
        sr: &'a dyn session::SessionRepository,
        ts: &'a dyn token_service::TokenService,
    ) -> Self {
        Self {
            session_repository: sr,
            token_service: ts,
        }
    }

    pub async fn execute(&self, dto: CreateSessionDTO) -> Result<CreateDTO, Error> {
        let token_data = token_service::IssueTokenDTO {
            user_id: dto.user_id,
            permission_group: dto.user_group,
        };
        let token_pair = self.token_service.issue(token_data)?;
        let refresh_token_exp = current_timestamp() + dto.refresh_token_ttl;

        let session_data = session::CreateSessionDTO {
            user_id: dto.user_id,
            refresh_token: token_pair.refresh_token.clone(),
            refresh_token_exp,
        };

        self.session_repository.create(session_data).await?;

        let session = entities::Session::new(dto.user_id, token_pair.refresh_token, refresh_token_exp);

        Ok(CreateDTO {
            entity: session,
            token: token_pair.token,
        })
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs() as u64
}
