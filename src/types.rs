use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Workout {
    pub day: String,
    pub activity: String,
    pub duration: u32,
    pub distance: f32,
    pub calories: f32,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct TodayStats {
    pub duration: u32,
    pub steps: u32,
    pub heart_rate: u32,
    pub activity: String,
    pub activity_short: String,
}

#[derive(Debug, Serialize)]
pub enum FitnessLevel {
    Low,
    Moderate,
    High,
}

#[derive(Serialize)]
pub struct SummaryResponse {
    pub total_duration: u32,
    pub total_distance: f32,
    pub total_calories: f32,
    pub average_steps: u32,
    pub goal_progress: f32,
    pub calories_today: f32,
    pub fitness_level: FitnessLevel,
}