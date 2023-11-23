use crate::entities;
use crate::ports::{repositories::session, services::token_service};

pub struct RefreshUseCase<'a> {
    session_repository: &'a dyn session::SessionRepository,
    token_service: &'a dyn token_service::TokenService,
}

pub struct RefreshSessionDTO {
    pub token: String,
    pub refresh_token: String,
    /// ttl in seconds
    pub refresh_token_ttl: u64,
}

pub struct RefreshDTO {
    pub entity: entities::Session,
    pub token: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Session has expired")]
    SessionExpired,
    #[error("Session not found")]
    SessionNotFound,
    #[error("Repository error [{0}]")]
    RepositoryError(#[from] crate::ports::Error),
}

impl<'a> RefreshUseCase<'a> {
    pub fn new(
        sr: &'a dyn session::SessionRepository,
        ts: &'a dyn token_service::TokenService,
    ) -> Self {
        Self {
            session_repository: sr,
            token_service: ts,
        }
    }

    pub async fn execute(&self, dto: RefreshSessionDTO) -> Result<RefreshDTO, Error> {
        if !self.session_exist(dto.refresh_token.clone()).await? {
            return Err(Error::SessionNotFound);
        }
        if !self.session_expire(dto.refresh_token.clone()).await? {
            return Err(Error::SessionExpired);
        }

        let token_data = token_service::RenewTokenDTO {
            token: dto.token,
            refresh_token: dto.refresh_token.clone(),
        };
        let token_pair = self.token_service.renew(token_data)?;

        let new_refresh_token_exp = current_timestamp() + dto.refresh_token_ttl;

        let session_data = session::UpdateSessionDTO {
            refresh_token: dto.refresh_token,
            new_refresh_token: token_pair.refresh_token.clone(),
            new_refresh_token_exp,
        };

        let session = self
            .session_repository
            .update(session_data)
            .await?
            .ok_or(Error::SessionNotFound)?;

        let updated_session = entities::Session::new(
            session.user_id,
            token_pair.refresh_token,
            new_refresh_token_exp,
        );

        Ok(RefreshDTO {
            entity: updated_session,
            token: token_pair.token,
        })
    }

    async fn session_exist(&self, refresh_token: String) -> Result<bool, Error> {
        Ok(self
            .session_repository
            .find(session::FindSessionDTO { refresh_token })
            .await?
            .is_some())
    }

    async fn session_expire(&self, refresh_token: String) -> Result<bool, Error> {
        let session = self
            .session_repository
            .find(session::FindSessionDTO { refresh_token })
            .await?
            .ok_or(Error::SessionNotFound)?;

        if current_timestamp() > session.refresh_token_exp {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs() as u64
}
