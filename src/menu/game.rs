use rand::prelude::*;

#[derive(Clone)]
pub struct Game {
    pub start: bool,
    pub score: u8,
    pub max_steps: u8,
    pub steps: u8,
    pub episode: u8,
    pub episodes_run: u8,
    pub chests: Vec<f32>,
    pub highlighted_chest: u8,
    pub mode: GameMode
}

impl Default for Game {
    fn default() -> Self {
        let mut game = Game {
            start: false,
            score: 0,
            max_steps: 20,
            steps: 0,
            episode: 0,
            episodes_run: 0,
            chests: vec![0.0; 8],
            highlighted_chest: 0,
            mode: GameMode::Human
        };

        game.set_chest_probability();

        game
    }
}

#[derive(Clone)]
pub enum GameMode {
    Human,
    AgentTrain,
    AgentInfer
}

pub enum ScoreType {
    Jackpot(i32),
    Treasure(i32),
    Bust(i32),
    Loss(i32),
    Robbed(i32)
}

impl ScoreType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ScoreType::Jackpot(_) => "Jackpot 🎰: ",
            ScoreType::Treasure(_) => "Treasure 💰: ",
            ScoreType::Bust(_) => "Bust ❌: ",
            ScoreType::Loss(_) => "Loss 😞: ",
            ScoreType::Robbed(_) => "Robbed 💀: "
        }
    }
}

impl GameMode {
    pub fn to_string(&self) -> &'static str {
        match self {
            GameMode::Human => "Human",
            GameMode::AgentTrain => "AgentTrain",
            GameMode::AgentInfer => "AgentInfer"
        }
    }
}

impl Game {

    /// Set Chest's static probabilities, these do not change for now.
    /// Probabilities are converted from percentages:
    /// - Chest 1: -20%  → -0.2
    /// - Chest 2:  35%  →  0.35
    /// - Chest 3:  10%  →  0.1
    /// - Chest 4:  65%  →  0.65
    /// - Chest 5: -80%  → -0.8
    /// - Chest 6:  80%  →  0.8
    /// - Chest 7: -30%  → -0.3 
    /// - Chest 8: -90%  → -0.9
    pub fn set_chest_probability(&mut self) {
        self.chests = vec![
            -0.2,  0.35,  0.1,  0.65,
            -0.8,  0.9,  -0.3,  -0.9,
        ];
    }

    /// Determines the score based on the probability of the selected chest compared to a randomly generated probability.
    ///
    /// - If the selected chest's probability (`chest_selected`) is greater than the random probability (`rand_prob`), the user gains:
    ///   - **2 points** if the gap (`chest_selected - rand_prob`) is **0.5 or more** (big win).
    ///   - **1 point** if the gap is **less than 0.5** (small win).
    ///
    /// - If the selected chest's probability is less than the random probability, the user loses:
    ///   - **-2 points** if the absolute gap (`rand_prob - chest_selected`) is **0.5 or more** (big loss).
    ///   - **-1 point** if the gap is **less than 0.5** (small loss).
    ///
    /// - If the probabilities match exactly (`gap == 0`), the score remains **0**.

    pub fn get_chest_score(&mut self) -> ScoreType {

        let mut rng = rand::rng();
        let rand_prob:f32  = rng.random_range(0.0..1.0);

        let chest_selected = self.chests[self.highlighted_chest as usize];

        // Gap between chest probability and random probability
        let gap = chest_selected - rand_prob; 

        if gap > 0.0 {
            if gap >= 0.5 {
                ScoreType::Jackpot(2)
            } else {
                ScoreType::Treasure(1)
            }
        } else if gap < 0.0 {
            if gap.abs() >= 0.5 {
                ScoreType::Robbed(-2)
            } else {
                ScoreType::Loss(-1)
            }
        } else {
            ScoreType::Bust(0)
        }
    }

    pub fn start(&mut self) {
        self.start=true;
    }

    pub fn end(&mut self) {
        self.start = false;
        self.score = 0;
        self.steps = 0;
        self.set_chest_probability();
    }

    pub fn reset_steps(&mut self) {
        self.score = 0;
        self.steps = 0;
        self.set_chest_probability();
    }

    pub fn move_right(&mut self) {
        self.highlighted_chest = (self.highlighted_chest + 1) % self.chests.len() as u8;
    }

    pub fn move_left(&mut self) {
        if self.highlighted_chest == 0 {
            self.highlighted_chest = (self.chests.len() - 1) as u8;
        } else {
            self.highlighted_chest -= 1;
        }
    }

    pub fn step(&mut self){
        self.steps+=1;
    }

    pub fn step_episode(&mut self){
        self.episode +=1;
    }

    pub fn game_over(&mut self) -> bool {
        self.steps == self.max_steps
    }

    pub fn train_run_done(&mut self) -> bool {
        match self.mode {
            GameMode::AgentTrain => self.episode == 50,
            _ => true
        }
    }
    
    
}