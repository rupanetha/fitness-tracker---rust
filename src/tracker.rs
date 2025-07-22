use crate::types::{Workout, TodayStats, FitnessLevel};
use crate::calculator::*;
use crate::display::display_summary;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::types::User;

pub struct FitnessTracker {
    pub user_id: i32,
    pub workouts: Vec<Workout>,
    pub today: TodayStats,
    pub weekly_steps: Vec<u32>,
    pub goal: u32,
}


impl FitnessTracker {
    pub fn new(user_id: i32, today: TodayStats, goal: u32) -> Self {
        FitnessTracker {
            user_id,
            workouts: Vec::new(),
            today,
            weekly_steps: Vec::new(),
            goal,
        }
    }

    pub fn display_report(&self) {
        let total_duration = total_duration(&self.workouts);
        let total_distance = total_distance(&self.workouts);
        let total_calories = total_calories(&self.workouts);
        let average = average_steps(&self.weekly_steps);
        let progress = goal_progress(average, self.goal);
        let burned = calories_burned(self.today.steps, self.today.duration as u32);
        let level = determine_fitness_level(burned.into());

        display_summary(
            &self.workouts, 
            &self.today, 
            &self.weekly_steps, 
            total_duration as u32, 
            total_distance, 
            total_calories as f32, 
            burned, 
            &level, 
            progress, 
            average,
        );
    }

    pub fn add_workout(&mut self, workout: Workout) {
        if workout.user_id == self.user_id {
            self.workouts.push(workout);
        }
    }

    pub fn update_today_stats(&mut self, stats: TodayStats) {
        self.today = stats;
    }

    pub fn update_weekly_steps(&mut self, steps: Vec<u32>) {
        self.weekly_steps = steps;
    }
}


