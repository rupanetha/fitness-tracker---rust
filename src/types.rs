use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, sqlx::FromRow)]
pub struct Workout {
    pub id: i32, 
    pub duration: i32,
    pub distance: f32,
    pub calories: f32,
    pub date: Option<NaiveDate>,
    
}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct TodayStats {
    pub duration: i32,
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
    pub goal_progress: f64,
    pub calories_today: f64,
    pub fitness_level: FitnessLevel,
}