use crate::types::{TodayStats, Workout, FitnessLevel};

pub fn display_summary(
    workouts: &[Workout],
    today: &TodayStats,
    weekly_steps: &[u32],
    total_duration: u32,
    total_distance: f32,
    total_calories: f32,
    calories_today: f32,
    fitness_level: &FitnessLevel,
    goal_progress: f32,
    average_steps: u32,
) {
    println!("🏃 Daily Fitness Tracker");
    println!("========================\n");

    println!("📊 Today's workout data:");
    println!("   Duration: {} minutes", today.duration);
    println!("   Steps: {:}", today.steps);
    println!("   Heart rate: {} bpm", today.heart_rate);
    println!("   Activity: {} ({})\n", today.activity, today.activity_short);

    println!("💪 Weekly progress:");
    println!("   Steps this week: {:?}", weekly_steps);
    println!("   Daily average: {:} steps", average_steps);
    println!("   Goal progress: {:.2}%\n", goal_progress);

    println!("🔥 Calorie calculations:");
    println!("   Calories burned: {:.2}", calories_today);
    println!("   Fitness level: {:?}", fitness_level);
}