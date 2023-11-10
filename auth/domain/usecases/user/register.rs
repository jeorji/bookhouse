use crate::ports::{repositories::user, services::hashing_service};

pub struct RegisterUseCase {
    user_repository: user::BxUserRepository,
    hashing_service: hashing_service::BxHashingService,
}

pub struct RegisterUserDTO {
    pub email: String,
    pub password: String,
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

impl RegisterUseCase {
    pub fn new(ur: user::BxUserRepository, hs: hashing_service::BxHashingService) -> Self {
        Self {
            user_repository: ur,
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

        self.user_repository
            .create(new_user)
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
