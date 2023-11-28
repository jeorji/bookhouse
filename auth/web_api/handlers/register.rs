use actix_web::{web, Responder};

use crate::response::APIResponse;
use crate::AppState;

use crate::adapters;
use adapters::permission::PostgresPermissonRepository;
use adapters::sha2::Sha256Hasher;
use adapters::user::PostgresUserRepository;

use auth;
use auth::usecases::user::register::{RegisterUseCase, RegisterUserDTO};

#[derive(serde::Deserialize)]
pub struct UserRegistrationForm {
    login: String,
    password: String,
    group_id: i32,
}

#[derive(serde::Serialize)]
pub struct UserRegistrationResp {}

pub async fn register(
    form: web::Form<UserRegistrationForm>,
    state: web::Data<AppState>,
) -> impl Responder {
    let user_repository = PostgresUserRepository {
        conncection_pool: state.db.clone(),
    };
    let permission_repository = PostgresPermissonRepository {
        conncection_pool: state.db.clone(),
    };

    let hasher = Sha256Hasher {};

    let register = RegisterUseCase::new(&user_repository, &permission_repository, &hasher);
    let new_user = RegisterUserDTO {
        email: form.login.clone(),
        password: form.password.clone(),
        permission_group: form.group_id,
    };

    register.execute(new_user).await.unwrap();

    APIResponse::success(UserRegistrationResp {})
}
