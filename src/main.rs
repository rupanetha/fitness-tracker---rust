mod types;
mod calculator;
mod display;
mod tracker;

use types::{TodayStats, Workout};
use tracker::FitnessTracker;

fn main() {
    let workouts = vec! [
        Workout { day: "Day 1".to_string(), activity: "Running".to_string(), duration: 30, distance: 5.0, calories: 50.0 },
        Workout { day: "Day 2".to_string(), activity: "Walking".to_string(), duration: 45, distance: 4.0, calories: 16.0 },
        Workout { day: "Day 3".to_string(), activity: "Cycling".to_string(), duration: 60, distance: 15.0, calories: 105.0 },
        Workout { day: "Day 4".to_string(), activity: "Yoga".to_string(), duration: 40, distance: 0.0, calories: 2.3 },
        Workout { day: "Day 5".to_string(), activity: "Strength".to_string(), duration: 50, distance: 0.0, calories: 5.0 },
    ];

    let today = TodayStats {
        duration: 45,
        steps: 8532,
        heart_rate: 145,
        activity: "Running".to_string(),
        activity_short: "R".to_string()
    };

    let weekly_steps = vec![8532, 9234, 7845, 9100, 8650, 8920, 8450];
    let goal = 10000;

    let tracker = FitnessTracker {
        workouts,
        today,
        weekly_steps,
        goal,
    };

    tracker.display_report();
}

