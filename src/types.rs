pub struct Workout {
    pub day: String,
    pub activity: String,
    pub duration: u32,
    pub distance: f32,
    pub calories: f32,
}

pub struct TodayStats {
    pub duration: u32,
    pub steps: u32,
    pub heart_rate: u32,
    pub activity: String,
    pub activity_Short: String,
}

pub enum FitnessLevel {
    Low,
    Moderate,
    High,
}