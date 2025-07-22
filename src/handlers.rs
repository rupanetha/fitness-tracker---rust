use actix_web::{get, post, web, Responder, HttpResponse};
use sqlx::PgPool;
use crate::types::{TodayStats, SummaryResponse, Workout, User, LoginRequest, RegisterRequest};
use crate::calculator::*;
use argon2::{self};
use uuid::Uuid;

#[get("/summary")]
pub async fn summary(
    pool: web::Data<PgPool>,
    query: web::Query<SummaryQuery>, // 👈 Add user_id as query param
) -> impl Responder {
    let user_id = query.user_id;

    let workouts: Vec<Workout> = sqlx::query_as::<_, Workout>(
        "SELECT * FROM workouts WHERE user_id = $1"
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

// #[post("/register")]
// pub async fn register_user(
//     pool: web::Data<PgPool>,
//     data: web::Json<RegisterRequest>,
// ) -> impl Responder {
//     let hashed = hash_password(&data.password);

//     let user = sqlx::query_as::<_, User>(
//         "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *"
//     )
//     .bind(&data.username)
//     .bind(&hashed)
//     .fetch_one(pool.get_ref())
//     .await;

//     match user {
//         Ok(u) => HttpResponse::Ok().json(u),
//         Err(_) => HttpResponse::InternalServerError().body("Could not register user"),
//     }
// }

// #[post("/login")]
// pub async fn login_user(
//     pool: web::Data<PgPool>,
//     data: web::Json<LoginRequest>,
// ) -> impl Responder {
//     let user = sqlx::query_as::<_, User>(
//         "SELECT * FROM users WHERE username = $1"
//     )
//     .bind(&data.username)
//     .fetch_optional(pool.get_ref())
//     .await;

//     match user {
//         Ok(Some(u)) => {
//             if verify_password(&data.password, &u.password) {
//                 HttpResponse::Ok().json(u)
//             } else {
//                 HttpResponse::Unauthorized().body("Invalid credentials")
//             }
//         },
//         _ => HttpResponse::Unauthorized().body("Invalid credentials"),
//     }
// }

// // ========== Password Helpers ==========
// fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
//     let salt = SaltString::generate(&mut OsRng);
//     let argon2 = Argon2::default();

//     let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
//     Ok(password_hash)
// }

// fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
//     let parsed_hash = PasswordHash::new(hash)?;
//     Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
// }