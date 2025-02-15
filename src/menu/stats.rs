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

impl Default for EpisodeEntry {
    fn default() -> Self {
        EpisodeEntry { episode: 0, score: 0, steps: 0, estimates: vec![0.0;8], breakdown: ScoreCount::default(), learning_rate: 0.1, action_selections: vec![0;20]}
    }
}