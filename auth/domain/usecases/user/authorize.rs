use crate::ports::{
    repositories::{permission, user},
    services::hashing_service,
};

use crate::entities;

pub struct AuthUseCase<'a> {
    user_repository: &'a dyn user::UserRepository,
    permission_repository: &'a dyn permission::PermissionRepository,
    hashing_service: &'a dyn hashing_service::HashingService,
}

pub struct AuthUserDTO {
    pub email: String,
    pub password: String,
}

pub struct AuthDTO {
    pub entity: entities::User,
    pub user_id: uuid::Uuid,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Permission group not found")]
    PermissionGroupNotFound,
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

impl<'a> AuthUseCase<'a> {
    pub fn new(
        ur: &'a dyn user::UserRepository,
        pr: &'a dyn permission::PermissionRepository,
        hs: &'a dyn hashing_service::HashingService,
    ) -> Self {
        Self {
            user_repository: ur,
            permission_repository: pr,
            hashing_service: hs,
        }
    }

    pub async fn execute(&self, dto: AuthUserDTO) -> Result<AuthDTO, Error> {
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
            .await?
            .ok_or(Error::UserNotFound)?;

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
            .await?
            .ok_or(Error::PermissionGroupNotFound)?;

        let authenticated_user = entities::User::new(user.email, user.password_hash, user_permission.group);

        Ok(AuthDTO {
            entity: authenticated_user,
            user_id: user.id,
        })
    }
}
