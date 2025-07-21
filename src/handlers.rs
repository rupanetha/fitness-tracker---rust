use actix_web::{get, web, Responder, HttpResponse};
use sqlx::PgPool;
use crate::types::{TodayStats, SummaryResponse};
use crate::calculator::*;
use crate::types::Workout;

#[get("/summary")]
pub async fn summary(pool: web::Data<PgPool>) -> impl Responder {
    let workouts: Vec<Workout> = sqlx::query_as::<_, Workout>("SELECT * FROM workouts")
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();



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
        total_duration: total_duration.try_into().unwrap(),
        total_distance,
        total_calories,
        average_steps: avg_steps,
        goal_progress: progress,
        calories_today: burned,
        fitness_level: level,
    };

    HttpResponse::Ok().json(response)
}