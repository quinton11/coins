use super::game::ScoreType;
use rand::Rng;
use serde::{Serialize, Deserialize};


#[derive(Clone)]
pub struct Player {
    pub mode: PlayerMode,
    pub score: i32,
    pub value_estimates: Vec<f32>,
    pub alpha: f32,
    pub score_count: ScoreCount,
    pub action_selections: Vec<u8>,
    pub epsilon: f32,
    pub epsilon_decay: f32,
    pub alpha_decay: f32,
}

impl Default for Player{
    fn default() -> Self {
        Player { 
            mode: PlayerMode::Human, score: 0,
            value_estimates: vec![0.0;8], 
            alpha: 0.1, 
            score_count: ScoreCount::default(), 
            action_selections: vec![0;20], 
            epsilon:0.3,
            epsilon_decay: 100.0,
            alpha_decay: 100.0
         }
    }
}

#[derive(Clone)]
pub enum PlayerMode {
    Human,
    Cpu
}

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct ScoreCount {
    pub jackpot: u8,
    pub treasure: u8,
    pub bust: u8,
    pub loss: u8,
    pub robbed: u8
}

impl Default for ScoreCount {
    fn default() -> Self {
        ScoreCount {jackpot: 0, treasure: 0, bust: 0, loss: 0, robbed: 0}
    }
}

impl Player {

    /// Update action estimates using the cumulative average (incremental mean)
    /// 
    /// Here we take the old value, and weight the difference between the old and new value
    /// Then we shift the old value by adding the weighted difference to move it to the mean
    /// Qnew = Qold + alpha * (Rnew - Qold)
    pub fn update_estimate(&mut self, selection: usize, reward: ScoreType, episode: f32) {

        let score =     match reward {
            ScoreType::Jackpot(value) 
            | ScoreType::Treasure(value) 
            | ScoreType::Bust(value) 
            | ScoreType::Loss(value) 
            | ScoreType::Robbed(value) => value,
        };

        let alpha = self.alpha /(1.0 + (episode/ self.alpha_decay));

        let old_value = self.value_estimates[selection];
        let new_value = old_value + alpha * (score as f32 - old_value);

        self.value_estimates[selection] = new_value;
        self.score += score;

        self.update_score_count(reward);
    }

    fn update_score_count(&mut self, reward: ScoreType) {
        match reward {
            ScoreType::Jackpot(_) => self.score_count.jackpot +=1,
            ScoreType::Treasure(_) => self.score_count.treasure +=1,
            ScoreType::Bust(_) => self.score_count.bust +=1,
            ScoreType::Loss(_) => self.score_count.loss +=1,
            ScoreType::Robbed(_) => self.score_count.robbed +=1
        }
    }

    pub fn zero_out_all(&mut self){
        self.score = 0;
        self.score_count = ScoreCount::default();
        self.value_estimates = vec![0.0;8];
        self.action_selections.clear();

    }

    pub fn zero_out(&mut self){
        self.score = 0;
        self.score_count = ScoreCount::default();
        self.action_selections = vec![0;20]
    }

    pub fn set_action_selection(&mut self, action: u8, step: usize){
        if step == self.action_selections.len() {
            return
        }
        self.action_selections[step] = action;
    }

    pub fn model_step(&mut self, episode: f32) -> u8 {
        let mut rng = rand::rng();
        let rand_prob: f32 = rng.random_range(0.0..1.0);

        let epsilon = self.epsilon /(1.0 + (episode/ self.epsilon_decay));
    
        if rand_prob <= epsilon {
            rng.random_range(0..8)
        } else {
            self.value_estimates
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map_or(0, |(index, _)| index as u8)
        }
    }
}

// to be able to track selection, and where the agent wants to move
// first make it such that, when that method is called then the agent makes a selection
// and 