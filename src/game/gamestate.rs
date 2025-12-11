use crossterm::terminal;
use std::io::Result;

pub struct GameState {
    pub score: u32,
    pub high_score: u32,
    pub snake: Vec<(u16, u16)>,
    pub direction: String,
}

fn get_snake_coordinates() -> Vec<(u16, u16)> {
    let (width, height) = terminal::size().expect("Error with read terminal params");

    let mut coordinates = vec![];

    coordinates.push((width / 2, height / 2));
    coordinates.push((width / 2 - 1, height / 2));
    coordinates.push((width / 2 - 2, height / 2));
    return coordinates;
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            high_score: 0,
            snake: get_snake_coordinates(),
            direction: "left".to_string(),
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
    }

    pub fn update(&mut self) {
        if self.direction == "left" {
            self.snake = self.snake.iter().map(|&(x, y)| (x - 1, y)).collect();
        } else if self.direction == "right" {
            self.snake = self.snake.iter().map(|&(x, y)| (x + 1, y)).collect();
        } else if self.direction == "up" {
            self.snake = self.snake.iter().map(|&(x, y)| (x, y - 1)).collect();
        } else if self.direction == "down" {
            self.snake = self.snake.iter().map(|&(x, y)| (x, y + 1)).collect();
        }
    }
}
