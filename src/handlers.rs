use actix_web::{get, post, web, Responder, HttpResponse};
use sqlx::PgPool;
use crate::types::{TodayStats, SummaryResponse, Workout, User, LoginRequest, RegisterRequest};
use crate::calculator::*;
use uuid::Uuid;
use argon2::{Argon2, password_hash::{PasswordHash, PasswordVerifier, PasswordHasher, SaltString}};
use rand::rngs::OsRng;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;
use chrono::{Utc, Duration};
use crate::types::{Claims, AuthResponse};


#[get("/summary")]
pub async fn summary(
    pool: web::Data<PgPool>,
    query: web::Query<SummaryQuery>, // 👈 Add user_id as query param
) -> impl Responder {
    let user_id = query.user_id;

    let workouts: Vec<Workout> = sqlx::query_as::<_, Workout>(
        "SELECT id, user_id, distance, duration, calories FROM FROM workouts WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    // Mock today stats — you can fetch from DB if stored
    let today = TodayStats {
        duration: 45,
        steps: 8532,
        heart_rate: 145,
        activity: "Running".to_string(),
        activity_short: "R".to_string(),
    };

    let weekly_steps = vec![8532, 9234, 7845, 9100, 8650, 8920, 8450];
    let goal = 10000;

    let total_duration = total_duration(&workouts);
    let total_distance = total_distance(&workouts);
    let total_calories = total_calories(&workouts);
    let avg_steps = average_steps(&weekly_steps);
    let progress = goal_progress(avg_steps, goal);
    let burned = calories_burned(today.steps, today.duration as u32);
    let level = determine_fitness_level(burned);

    let response = SummaryResponse {
        total_duration: total_duration.try_into().unwrap_or(0),
        total_distance,
        total_calories,
        average_steps: avg_steps,
        goal_progress: progress,
        calories_today: burned,
        fitness_level: level,
    };

    HttpResponse::Ok().json(response)
}

#[derive(serde::Deserialize)]
pub struct SummaryQuery {
    pub user_id: i32,
}

#[post("/register")]
pub async fn register_user(
    pool: web::Data<PgPool>,
    data: web::Json<RegisterRequest>,
) -> impl Responder {
    match hash_password(&data.password) {
        Ok(hashed) => {
            let user = sqlx::query_as::<_, User>(
                "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING *"
            )
            .bind(&data.username)
            .bind(&hashed)
            .fetch_one(pool.get_ref())
            .await;

            match user {
                Ok(u) => HttpResponse::Ok().json(u),
                Err(e) => {
                    eprintln!("DB Error: {:?}", e);
                    HttpResponse::InternalServerError().body("Could not register user")
                },
            }
        }
        Err(e) => {
            eprintln!("Hash Error: {:?}", e);
            HttpResponse::InternalServerError().body("Password hashing failed")
        }
    }
}



#[post("/login")]
pub async fn login_user(
    pool: web::Data<PgPool>,
    data: web::Json<LoginRequest>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&data.username)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(u)) => match verify_password(&data.password, &u.password_hash) {
            Ok(true) => {
                // JWT token creation
                let secret = env::var("JWT_SECRET").unwrap_or("secret".into());
                let expiration = Utc::now()
                    .checked_add_signed(Duration::hours(24))
                    .expect("valid timestamp")
                    .timestamp() as usize;

                let claims = Claims {
                    sub: u.id.to_string(),
                    exp: expiration,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(secret.as_bytes()),
                )
                .unwrap();

                HttpResponse::Ok().json(AuthResponse {
                    message: "Login successful".to_string(),
                    token,
                })
            },
            _ => HttpResponse::Unauthorized().body("Invalid credentials"),
        },
        _ => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

#[post("/logout")]
pub async fn logout_user() -> impl Responder {
    HttpResponse::Ok().body("User logged out successfully")
}

// ========== Password Helpers ==========
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}



