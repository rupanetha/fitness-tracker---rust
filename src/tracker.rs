use crate::types::{Workout, TodayStats, FitnessLevel};
use crate::calculator::*;
use crate::display::display_summary;

pub struct FitnessTracker {
    pub workouts: Vec<Workout>,
    pub today: TodayStats,
    pub weekly_steps: Vec<u32>,
    pub goal: u32,
}

impl FitnessTracker {
    pub fn display_report(&self) {
        let total_duration = total_duration(&self.workouts);
        let total_distance = total_distance(&self.workouts);
        let total_calories = total_calories(&self.workouts);
        let average = average_steps(&self.weekly_steps);
        let progress = goal_progress(average, self.goal);
        let burned = calories_burned(self.today.steps, self.today.duration);
        let level = determine_fitness_level(burned);

        display_summary(
            &self.workouts, 
            &self.today, 
            &self.weekly_steps, 
            total_duration, 
            total_distance, 
            total_calories, 
            burned, 
            &level, 
            progress, 
            average,
        );
    }
}