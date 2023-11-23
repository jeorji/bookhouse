use crate::ports::{
    repositories::{permission, user},
    services::hashing_service,
};

pub struct RegisterUseCase<'a> {
    user_repository: &'a dyn user::UserRepository,
    permission_repository: &'a dyn permission::PermissionRepository,
    hashing_service: &'a dyn hashing_service::HashingService,
}

pub struct RegisterUserDTO {
    pub email: String,
    pub password: String,
    pub permission_group: i32,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Password cannot be empty")]
    EmptyPassword,
    #[error("Email cannot be empty")]
    EmptyEmail,
    #[error("User already exists")]
    UserAlreadyExist,
    #[error("Repository error [{0}]")]
    RepositoryError(crate::ports::Error),
}

impl<'a> RegisterUseCase<'a> {
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

    pub async fn execute(&self, dto: RegisterUserDTO) -> Result<(), Error> {
        if dto.password.is_empty() {
            return Err(Error::EmptyPassword);
        }
        if dto.email.is_empty() {
            return Err(Error::EmptyEmail);
        }
        if self.user_exists(&dto).await? {
            return Err(Error::UserAlreadyExist);
        }

        let password_dto = hashing_service::HashPasswordDTO {
            plain_password: dto.password,
        };
        let (password_hash, salt) = self.hashing_service.hash(password_dto);

        let new_user = user::CreateUserDTO {
            email: dto.email.clone(),
            password_hash,
            salt,
        };
        let user = self
            .user_repository
            .create(new_user)
            .await
            .map_err(Error::RepositoryError)?;

        let user_permission = permission::CreatePermissionDTO {
            user_id: user.id,
            group: dto.permission_group,
        };
        self.permission_repository
            .create(user_permission)
            .await
            .map_err(Error::RepositoryError)?;

        Ok(())
    }

    async fn user_exists(&self, dto: &RegisterUserDTO) -> Result<bool, Error> {
        let find_user_dto = user::FindUserDTO {
            email: dto.email.clone(),
        };

        match self.user_repository.find(find_user_dto).await {
            Ok(user) => Ok(user.is_some()),
            Err(err) => Err(Error::RepositoryError(err)),
        }
    }
}
