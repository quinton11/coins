use super::game::ScoreType;



#[derive(Clone)]
pub struct Player {
    pub mode: PlayerMode,
    pub score: i32,
    pub value_estimates: Vec<f32>,
    pub alpha: f32,
    pub score_count: ScoreCount
}

impl Default for Player{
    fn default() -> Self {
        Player { mode: PlayerMode::Human, score: 0, value_estimates: vec![0.0;8], alpha: 0.1, score_count: ScoreCount::default() }
    }
}

#[derive(Clone)]
pub enum PlayerMode {
    Human,
    Cpu
}

#[derive(Clone)]
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
    pub fn update_estimate(&mut self, selection: usize, reward: ScoreType) {

        let score =     match reward {
            ScoreType::Jackpot(value) 
            | ScoreType::Treasure(value) 
            | ScoreType::Bust(value) 
            | ScoreType::Loss(value) 
            | ScoreType::Robbed(value) => value,
        };

        let old_value = self.value_estimates[selection];
        let new_value = old_value + self.alpha * (score as f32 - old_value);

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
}