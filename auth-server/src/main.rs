use axum::{routing::post, Router};
use axum::{extract::{State, Json}, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use chrono::{Utc, Duration};
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::routing::get;
use once_cell::sync::Lazy;
use sqlx::{query, query_scalar, Acquire, PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;

// Hardcoded secret for demo — use env in real projects
const SECRET_KEY: &[u8] = b"super_secret_key_1234567890";

pub static ARGON2: Lazy<Argon2<'static>> = Lazy::new(|| {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
});


#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

struct User {
    id: Uuid,
    email: String,
    username: String,
    password: String,
}


#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    is_premium: bool,
    exp: usize,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    
    // build database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/game-demo".to_string());

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // build our application with a route
    let app = Router::new()
        .route("/register", post(register))
        .route("/authenticate", post(authenticate))
        .route("/renew", post(renew))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



async fn register(
    Json(payload): Json<RegisterRequest>,
    State(pool): State<Pool<Postgres>>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = pool.acquire().await.map_err(|err| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", err))
    })?;

    if !is_valid_email(&payload.email) {
        return Err((StatusCode::BAD_REQUEST, "Invalid email".into()));
    }

    if !is_secure_password(&payload.password) {
        return Err((StatusCode::BAD_REQUEST, "Weak password".into()));
    }
    let connection = conn.acquire().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    });

    let existing: bool = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users WHERE email = $1
            )
            "#,
            payload.email
        )
        .fetch_one(&pool) // ✅ This is simpler and avoids `conn`
        .await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    });

    if existing {
        return Err((StatusCode::BAD_REQUEST, "Already registered".into()));
    }

    let id = Uuid::new_v4();

    query!(
        r#"
        INSERT INTO users (id, email, username, password)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        payload.email,
        payload.username,
        payload.password // ⚠️ Consider hashing this!
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Insert error: {}", err),
            )
        })?;


    Ok("Registration completed")
}



async fn authenticate(
    Json(payload): Json<LoginRequest>,
    State(pool): State<Pool<Postgres>>
) -> Json<TokenResponse> {
    if payload.email != "test@example.com" || payload.password != "Password123" {
        return Json(TokenResponse {
            access_token: "invalid_credentials".to_string(),
            refresh_token: "invalid_refresh_token".to_string(),
        });
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: "user-uuid-1234".to_string(),
        email: payload.email,
        is_premium: true,
        exp: expiration,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    ).unwrap();

    Json(TokenResponse { access_token: token, refresh_token: "token".to_string() })
}

async fn renew(
    Json(payload): Json<LoginRequest>,
    State(pool): State<Pool<Postgres>>
) -> impl IntoResponse  {

    Json(TokenResponse { access_token: "token".to_string(), refresh_token: "refresh_token".to_string() })
}



fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    ARGON2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn hash_password(password: &str
) -> String {
    let salt = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}


fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}


fn is_secure_password(password: &str) -> bool {
    password.len() >= 8 && password.chars().any(char::is_uppercase)
}

