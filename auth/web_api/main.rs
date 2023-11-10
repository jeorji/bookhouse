use actix_web::{post, web, App, HttpServer, Responder};
use sqlx::postgres::{PgPool, PgPoolOptions};

mod adapters;
use adapters::sha2::Sha256Hasher;
use adapters::user::PostgresUserRepository;
use auth::usecases::user::register::{RegisterUseCase, RegisterUserDTO};

struct AppState {
    db: PgPool,
}

#[derive(serde::Deserialize)]
struct UserCredentials {
    login: String,
    password: String,
}

#[post("/register")]
async fn register(form: web::Form<UserCredentials>, state: web::Data<AppState>) -> impl Responder {
    let repository = PostgresUserRepository {
        conncection_pool: state.db.clone(),
    };
    let hasher = Sha256Hasher {};

    let register = RegisterUseCase::new(Box::new(repository), Box::new(hasher));
    let new_user = RegisterUserDTO {
        email: form.login.clone(),
        password: form.password.clone(),
    };

    register.execute(new_user).await.unwrap();

    actix_web::HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let app_state = AppState {
        db: pool,
    };

    let app_data = web::Data::new(app_state);

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(register))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
