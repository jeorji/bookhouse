use crate::ports::{
    repositories::{permission, user},
    services::hashing_service,
};

use crate::entities;

pub struct AuthUseCase {
    user_repository: user::BxUserRepository,
    permission_repository: permission::BxPermissionRepository,
    hashing_service: hashing_service::BxHashingService,
}

pub struct AuthUserDTO {
    pub email: String,
    pub password: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("User not found")]
    UserNotFound,
    #[error("Incorrect credentials")]
    WrongCredentials,
    #[error("Password cannot be empty")]
    EmptyPassword,
    #[error("Email cannot be empty")]
    EmptyEmail,
    #[error("Repository error [{0}]")]
    RepositoryError(#[from] crate::ports::Error),
}

impl AuthUseCase {
    pub fn new(
        ur: user::BxUserRepository,
        pr: permission::BxPermissionRepository,
        hs: hashing_service::BxHashingService,
    ) -> Self {
        Self {
            user_repository: ur,
            permission_repository: pr,
            hashing_service: hs,
        }
    }

    pub async fn execute(&self, dto: AuthUserDTO) -> Result<entities::User, Error> {
        if dto.password.is_empty() {
            return Err(Error::EmptyPassword);
        }
        if dto.email.is_empty() {
            return Err(Error::EmptyEmail);
        }

        let find_dto = user::FindUserDTO { email: dto.email };
        let user = self
            .user_repository
            .find(find_dto)
            .await?;

        let user = user.ok_or(Error::UserNotFound)?;

        let data = hashing_service::VerifyPasswordDTO {
            plain_password: dto.password,
            salt: user.salt,
            password_hash: user.password_hash.clone(),
        };

        if !self.hashing_service.verify(data) {
            return Err(Error::WrongCredentials);
        }

        let permission_group = permission::FindPermissionDTO { user_id: user.id };
        let user_permission = self
            .permission_repository
            .find(permission_group)
            .await?;

        Ok(entities::User::new(
            user.email,
            user.password_hash,
            user_permission.group,
        ))
    }
}
