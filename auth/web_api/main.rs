use actix_web::{post, web, App, HttpServer, Responder};
use sqlx::postgres::{PgPool, PgPoolOptions};

mod adapters;
use adapters::jwt::JWTService;
use adapters::permission::PostgresPermissonRepository;
use adapters::session::PostgresSessionRepository;
use adapters::sha2::Sha256Hasher;
use adapters::user::PostgresUserRepository;

use auth::usecases::session;
use auth::usecases::session::create::CreateSessionDTO;
use auth::usecases::session::refresh::RefreshSessionDTO;
use auth::usecases::user;
use auth::usecases::user::authorize::AuthUserDTO;
use auth::usecases::user::register::{RegisterUseCase, RegisterUserDTO};

struct AppConfig {
    rt_ttl: u64,
}

struct AppState {
    config: AppConfig,
    db: PgPool,
    token_service: JWTService,
}

#[derive(serde::Deserialize)]
struct UserAuthorizationCredentials {
    login: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct TokenCredentials {
    token: String,
    refresh_token: String,
}

#[post("/refresh")]
async fn refresh(form: web::Form<TokenCredentials>, state: web::Data<AppState>) -> impl Responder {
    let session_repository = PostgresSessionRepository {
        conncection_pool: state.db.clone(),
    };

    let session_updater = session::RefreshUseCase::new(&session_repository, &state.token_service);

    let data = RefreshSessionDTO {
        token: form.token.clone(),
        refresh_token: form.refresh_token.clone(),
        refresh_token_ttl: state.config.rt_ttl,
    };
    let updated_session = session_updater.execute(data).await.unwrap();

    dbg! { updated_session.entity.refresh_token };
    dbg! { updated_session.entity.refresh_token_exp };
    dbg! { updated_session.token };

    actix_web::HttpResponse::Ok().finish()
}

#[post("/authorize")]
async fn authorize(
    form: web::Form<UserAuthorizationCredentials>,
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

    dbg! { session.token };
    dbg! { session.entity.refresh_token };

    actix_web::HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct UserRegistrationCredentials {
    login: String,
    password: String,
    group_id: i32,
}
#[post("/register")]
async fn register(
    form: web::Form<UserRegistrationCredentials>,
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

    actix_web::HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let jwt_pem_path = dotenvy::var("JWT_PEM_PATH").unwrap();
    let jwt_ttl: usize = dotenvy::var("JWT_TTL").unwrap().parse().unwrap();
    let rt_ttl: u64 = dotenvy::var("RT_TTL").unwrap().parse().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let jwt_pem = std::fs::read(jwt_pem_path).unwrap();
    let jwt_token_service = JWTService::es256_from_pem_with_ttl(&jwt_pem, jwt_ttl).unwrap();

    let app_conf = AppConfig { rt_ttl };

    let app_state = AppState {
        config: app_conf,
        db: pool,
        token_service: jwt_token_service,
    };

    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(register)
            .service(authorize)
            .service(refresh)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
