use bevy::prelude::Resource;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

#[derive(Resource)]
pub struct Performance {
    data: HashMap<String, VecDeque<Duration>>,
    max_entries: usize,
}

pub struct FunctionPerformance {
    pub name: String,
    pub total_duration: f64,
    pub min: Duration,
    pub p5: Duration,
    pub median: Duration,
    pub p95: Duration,
    pub max: Duration,
}

impl Performance {
    pub fn new(max_entries: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_entries,
        }
    }

    pub fn add_duration(&mut self, function_name: &str, duration: Duration) {
        let entry = self
            .data
            .entry(function_name.to_string())
            .or_insert_with(|| VecDeque::with_capacity(self.max_entries));

        if entry.len() == self.max_entries {
            entry.pop_front();
        }

        entry.push_back(duration);
    }

    pub fn describe_all(&self) -> Vec<FunctionPerformance> {
        let mut function_stats: Vec<FunctionPerformance> = Vec::new();

        let total_duration_secs = &self.data.iter().fold(0.0, |acc, (_, durations)| {
            acc + durations.iter().sum::<Duration>().as_secs_f64()
        });

        for (name, durations) in &self.data {
            let count = durations.len();
            if count == 0 {
                continue;
            }

            let mut sorted_durations = durations.clone().into_iter().collect::<Vec<_>>();
            sorted_durations.sort_unstable();

            let min = sorted_durations[0];
            let p5 = sorted_durations[(count as f64 * 0.05) as usize];
            let median = sorted_durations[count / 2];
            let p95 = sorted_durations[(count as f64 * 0.95) as usize];
            let max = sorted_durations[count - 1];

            let total_duration =
                durations.iter().sum::<Duration>().as_secs_f64() / total_duration_secs * 100.0;

            function_stats.push(FunctionPerformance {
                name: name.to_string(),
                total_duration,
                min,
                p5,
                median,
                p95,
                max,
            });
        }

        function_stats.sort_by(|a, b| {
            b.total_duration
                .partial_cmp(&a.total_duration)
                .unwrap_or(Ordering::Equal)
        });

        function_stats
    }
}
