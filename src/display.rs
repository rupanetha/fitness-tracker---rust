use crate::types::{TodayStats, Workout, FitnessLevel};

pub fn display_summary(
    workouts: &[Workout],
    today: &TodayStats,
    weekly_steps: &[u32],
    total_duration: u32,
    total_distance: f32,
    total_calories: f32,
    calories_today: f64,
    fitness_level: &FitnessLevel,
    goal_progress: f64,
    average_steps: u32,
) {
    println!("🏃 Daily Fitness Tracker");
    println!("========================\n");

    println!("📊 Today's workout data:");
    println!("   Duration: {} minutes", today.duration);
    println!("   Steps: {:}", today.steps);
    println!("   Heart rate: {} bpm", today.heart_rate);
    println!("   Activity: {} ({})\n", today.activity, today.activity_short);

    println!("📈 Weekly Overview");
    println!("------------------");
    println!("   Steps (7 days): {:?}", weekly_steps);
    println!("   Avg. Daily Steps: {}", average_steps);
    println!("   Goal Progress   : {:.2}%\n", goal_progress);

    println!("🔥 Calories & Fitness");
    println!("---------------------");
    println!("   Calories Burned Today : {:.2}", calories_today);
    println!("   Total Weekly Calories : {:.2}", total_calories);
    println!("   Fitness Level         : {:?}\n", fitness_level);

    println!("🧭 Distance & Time");
    println!("------------------");
    println!("   Total Duration (min): {}", total_duration);
    println!("   Total Distance (km) : {:.2}\n", total_distance);
}