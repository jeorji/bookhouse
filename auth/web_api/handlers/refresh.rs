use actix_web::{web, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::response::APIResponse;
use crate::AppState;

use crate::adapters;
use adapters::session::PostgresSessionRepository;

use auth::usecases::session;
use auth::usecases::session::refresh::RefreshSessionDTO;

#[derive(serde::Deserialize)]
pub struct TokenForm {
    token: String,
}

#[derive(serde::Serialize)]
struct TokenRefreshResp {
    token: String,
    refresh_token: String,
    refresh_token_exp: u64,
}

pub async fn refresh(
    auth: BearerAuth,
    form: web::Form<TokenForm>,
    state: web::Data<AppState>,
) -> impl Responder {
    let session_repository = PostgresSessionRepository {
        conncection_pool: state.db.clone(),
    };

    let session_updater = session::RefreshUseCase::new(&session_repository, &state.token_service);

    let data = RefreshSessionDTO {
        token: form.token.clone(),
        refresh_token: auth.token().to_string(),
        refresh_token_ttl: state.config.rt_ttl,
    };
    let updated_session = session_updater.execute(data).await.unwrap();

    APIResponse::success(TokenRefreshResp {
        token: updated_session.token,
        refresh_token: updated_session.entity.refresh_token,
        refresh_token_exp: updated_session.entity.refresh_token_exp,
    })
}
