use actix_web::{web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};

mod adapters;
use adapters::jwt::JWTService;

mod handlers;
mod response;

struct AppConfig {
    rt_ttl: u64,
}

pub struct AppState {
    config: AppConfig,
    db: PgPool,
    token_service: JWTService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let jwt_private_pem_path = dotenvy::var("JWT_PRIVATE_PEM_PATH").unwrap();
    let jwt_public_pem_path = dotenvy::var("JWT_PUBLIC_PEM_PATH").unwrap();
    let jwt_ttl: usize = dotenvy::var("JWT_TTL").unwrap().parse().unwrap();
    let rt_ttl: u64 = dotenvy::var("RT_TTL").unwrap().parse().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    sqlx::migrate!("web_api/adapters/repositories/postgresql/migrations")
        .run(&pool)
        .await.unwrap();

    let jwt_private_pem = std::fs::read(jwt_private_pem_path).unwrap();
    let jwt_public_pem = std::fs::read(jwt_public_pem_path).unwrap();
    let jwt_token_service =
        JWTService::es256_from_pem_with_ttl(&jwt_public_pem, &jwt_private_pem, jwt_ttl).unwrap();

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
            .route("/register", web::post().to(handlers::register))
            .route("/authorize", web::post().to(handlers::authorize))
            .route("/refresh", web::post().to(handlers::refresh))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
