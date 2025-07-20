use crate::types::{Workout, FitnessLevel};

pub fn total_duration(workouts: &[Workout]) -> i32 {
    workouts.iter().map(|w| w.duration).sum()
}

pub fn total_distance(workouts: &[Workout]) -> f32 {
    workouts.iter().map(|w| w.distance).sum()
}

pub fn total_calories(workouts: &[Workout]) -> f32 {
    workouts.iter().map(|w| w.calories).sum()
}

pub fn average_steps(steps: &[u32]) -> u32 {
    let total:u32 = steps.iter().sum();
    total / steps.len() as u32
}

pub fn goal_progress(average_steps: u32, goal: u32) -> f32 {
    (average_steps as f32 / goal as f32) * 100.0
}

pub fn calories_burned(steps: u32, duration: u32) -> f32 {
    (steps as f32 * 0.04) + (duration as f32 * 2.0)
}

pub fn determine_fitness_level(calories: f32) -> FitnessLevel {
    if calories < 200.0 {
        FitnessLevel::Low
    } else if calories < 500.0 {
        FitnessLevel::Moderate
    } else {
        FitnessLevel::High
    }
}

