use actix_web::{web, Responder};

use crate::response::APIResponse;
use crate::AppState;

use crate::adapters;
use adapters::permission::PostgresPermissonRepository;
use adapters::session::PostgresSessionRepository;
use adapters::sha2::Sha256Hasher;
use adapters::user::PostgresUserRepository;

use auth::usecases::session;
use auth::usecases::session::create::CreateSessionDTO;
use auth::usecases::user;
use auth::usecases::user::authorize::AuthUserDTO;

#[derive(serde::Deserialize)]
pub struct UserAuthorizationForm {
    login: String,
    password: String,
}

#[derive(serde::Serialize)]
pub struct UserAuthorizationResp {
    token: String,
    refresh_token: String,
}

pub async fn authorize(
    form: web::Form<UserAuthorizationForm>,
    state: web::Data<AppState>,
) -> impl Responder {
    let user_repository = PostgresUserRepository {
        conncection_pool: state.db.clone(),
    };
    let permission_repository = PostgresPermissonRepository {
        conncection_pool: state.db.clone(),
    };

    let hasher = Sha256Hasher {};

    let authorize = user::AuthUseCase::new(&user_repository, &permission_repository, &hasher);
    let auth_data = AuthUserDTO {
        email: form.login.clone(),
        password: form.password.clone(),
    };
    let user = authorize.execute(auth_data).await.unwrap();

    let session_repository = PostgresSessionRepository {
        conncection_pool: state.db.clone(),
    };

    let new_session = session::CreateUseCase::new(&session_repository, &state.token_service);
    let session_data = CreateSessionDTO {
        user_id: user.user_id,
        user_group: user.entity.group_id,
        refresh_token_ttl: state.config.rt_ttl,
    };
    let session = new_session.execute(session_data).await.unwrap();

    APIResponse::success(UserAuthorizationResp {
        token: session.token,
        refresh_token: session.entity.refresh_token,
    })
}
