use ratatui::{style::{Color, Style}, symbols, widgets::{Dataset, GraphType}};

use super::player::ScoreCount;

#[derive(Debug)]
pub struct EpisodeEntry {
    pub episode: u32,
    pub score: i32,
    pub steps: u32,
    pub estimates: Vec<f32>,
    pub breakdown: ScoreCount,
    pub learning_rate: f32,
    pub action_selections: Vec<u8>
}


#[derive(Debug)]
pub struct StatRecords {
    pub has_records: bool,
    pub entries: Vec<EpisodeEntry>
}

impl Default for EpisodeEntry {
    fn default() -> Self {
        EpisodeEntry { episode: 0, score: 0, steps: 0, estimates: vec![0.0;8], breakdown: ScoreCount::default(), learning_rate: 0.1, action_selections: vec![0;20]}
    }
}

impl Default for StatRecords {
    fn default() -> Self {
        StatRecords{ has_records: false, entries: vec![]}
    }
}

impl StatRecords {
    pub fn prepare_value_estimates_chart(&self) -> (Vec<Dataset<'static>>, [f64; 2], [f64; 2]) {
        let num_actions = 8;
        let num_episodes = self.entries.len();

        // Get the most recent 100 episodes or fewer
        let start_index = if num_episodes > 100 { num_episodes - 100 } else { 0 };
        let relevant_entries = &self.entries[start_index..];

        let mut action_data: Vec<Vec<(f64, f64)>> = vec![Vec::new(); num_actions];

        let mut min_estimate = f64::MAX;
        let mut max_estimate = f64::MIN;

        for (i, entry) in relevant_entries.iter().enumerate() {
            let episode_num = (start_index + i + 1) as f64;

            for (action_idx, estimate) in entry.estimates.iter().enumerate() {
                let estimate = *estimate as f64;
                action_data[action_idx].push((episode_num, estimate));

                if estimate < min_estimate {
                    min_estimate = estimate;
                }
                if estimate > max_estimate {
                    max_estimate = estimate;
                }
            }
        }

        // Ensure reasonable y-axis bounds
        let y_min = min_estimate.min(-2.5);
        let y_max = max_estimate.max(2.5);

        // Colors for actions
        let action_colors = [
            Color::Cyan, Color::Magenta, Color::Yellow, Color::Green,
            Color::Blue, Color::Red, Color::LightCyan, Color::White,
        ];

        // Convert action data into datasets
        let datasets: Vec<Dataset<'static>> = action_data
            .into_iter()
            .enumerate()
            .map(|(idx, data)| {
                Dataset::default()
                    .name(format!("Action {}", idx))
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(action_colors[idx]))
                    .data(Box::leak(Box::new(data)))
            })
            .collect();

        let x_bounds = [start_index as f64, num_episodes as f64];
        let y_bounds = [y_min, y_max];

        (datasets, x_bounds, y_bounds)
    }


    pub fn normalized_outcome_breakdown(&self) -> Vec<(&'static str, u64)> {
        if self.entries.is_empty() {
            return vec![];
        }

        // Compute total counts for each outcome
        let total_jackpot: u64 = self.entries.iter().map(|e| e.breakdown.jackpot as u64).sum();
        let total_treasure: u64 = self.entries.iter().map(|e| e.breakdown.treasure as u64).sum();
        let total_bust: u64 = self.entries.iter().map(|e| e.breakdown.bust as u64).sum();
        let total_loss: u64 = self.entries.iter().map(|e| e.breakdown.loss as u64).sum();
        let total_robbed: u64 = self.entries.iter().map(|e| e.breakdown.robbed as u64).sum();

        let total_outcomes = total_jackpot + total_treasure + total_bust + total_loss + total_robbed;
        
        if total_outcomes == 0 {
            return vec![];
        }

        // Normalize values to percentages (100%)
        let normalize = |value: u64| (value as f64 / total_outcomes as f64 * 100.0) as u64;

        vec![
            ("Jackpot", normalize(total_jackpot)),
            ("Treasure", normalize(total_treasure)),
            ("Bust", normalize(total_bust)),
            ("Loss", normalize(total_loss)),
            ("Robbed", normalize(total_robbed)),
        ]
    }


    pub fn recent_score_progress(&self) -> Vec<(String, u64)> {
        if self.entries.is_empty() {
            return vec![];
        }

        let num_entries = self.entries.len();
        let max_entries = 100;
        let max_bars = 5;

        let start_index = num_entries.saturating_sub(max_entries);
        let relevant_entries: Vec<&EpisodeEntry> = self.entries[start_index..].iter().collect();

        let mut selected_episodes: Vec<(String, u64)> = relevant_entries
            .iter()
            .enumerate()
            .filter(|(index, _)| (index + 1) % 5 == 0) 
            .map(|(_, entry)| {
                let label = format!("Ep {}", entry.episode);
                let score = entry.score.max(0) as u64;
                (label, score)
            })
            .collect();

        while selected_episodes.len() > max_bars {
            selected_episodes.remove(0);
        }

        selected_episodes
    }

    pub fn max_score(&self) -> u64 {
        let max_in_selected = self.recent_score_progress()
            .iter()
            .map(|(_, v)| *v)
            .max()
            .unwrap_or(10);

        // safe range
        max_in_selected.clamp(10, 500)
    }
}

