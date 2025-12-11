pub struct GameState {
    pub score: u32,
    pub high_score: u32,
    pub snake_length: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            high_score: 0,
            snake_length: 3,
        }
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.snake_length = 3;
    }
}
