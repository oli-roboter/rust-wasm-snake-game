// mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern {
    fn rnd(max: usize) -> usize;
}

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Playing,
}

#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Self {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_head: Option<SnakeCell>,
    apple_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> Self {
        let size = width * width;
        let snake = Snake::new(snake_idx, 3);

        Self {
            width,
            size,
            apple_cell: Self::generate_apple_cell(size, &snake.body),
            snake,
            next_head: None,
            status: None,
            points: 0,
        }
    }

    fn generate_apple_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut apple_cell;

        loop {
            apple_cell = rnd(max);
            if !snake_body.contains(&SnakeCell(apple_cell)) {
                break;
            }
        }
        Some(apple_cell)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_apple(&self) -> Option<usize> {
        // println!("Generated another apple at position {:?}", self.apple_cell);
        self.apple_cell
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn get_snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Playing);
    }

    pub fn get_game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn get_game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Lost) => { String::from("You have lost") }
            Some(GameStatus::Won) => { String::from("You have won") }
            Some(GameStatus::Playing) => { String::from("Playing") }
            None => { String::from("No Status") }
        }
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_head = self.generate_next_snake_head(&direction);
        if self.snake.body[1].0 == next_head.0 {
            return;
        }

        self.next_head = Some(next_head);
        self.snake.direction = direction;
    }

    /*
    pub fn snake_cells(&self) -> &Vec<SnakeCell> {
        &self.snake.body
    }
    If you try to return Vec<SnakeCell> (self.snake.body) this will destroy the self and then it can't be updated anymore
    If we try to return a reference like the function above, the compiler does not allow it because:
    "cannot return a borrowed ref with #[wasm_bindgen]"
    I think this is because by passing this to js and then later borrowing this to some other funtion
    in rust might create a dangling reference in javascript
    */

    // *const is a pointer to the first cell in the vector
    // borrowing rules don't apply to it
    pub fn snake_cell_ptr(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn next_position(&mut self) -> () {
        match self.status {
            Some(GameStatus::Playing) => {
                let temp = self.snake.body.clone();
                // let next_cell = self.generate_next_snake_head(&self.snake.direction);
                match self.next_head {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_head = None;
                    }
                    None => {
                        self.snake.body[0] = self.generate_next_snake_head(&self.snake.direction);
                    }
                }

                //from 1 as 0 is the snake head
                for i in 1..self.get_snake_length() {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..self.get_snake_length()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }

                if self.apple_cell == Some(self.get_snake_head()) {
                    if self.get_snake_length() < self.size {
                        self.points += 1;
                        self.apple_cell = Self::generate_apple_cell(self.size, &self.snake.body);
                    } else {
                        self.apple_cell = None;
                        self.status = Some(GameStatus::Won);
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
        }
    }

    fn generate_next_snake_head(&self, direction: &Direction) -> SnakeCell {
        let snake_head_idx = self.get_snake_head();
        let row = snake_head_idx / self.width;

        return match direction {
            Direction::Right => {
                // optimisation to avoid expensive module and division
                let treshold = (row + 1) * self.width;
                let cell_idx = if snake_head_idx + 1 == treshold {
                    treshold - self.width
                } else {
                    snake_head_idx + 1
                };
                SnakeCell(cell_idx)
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_head_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_head_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_head_idx - row * self.width;
                if snake_head_idx == treshold {
                    SnakeCell(self.size - self.width + treshold)
                } else {
                    SnakeCell(snake_head_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_head_idx + (self.width - row) * self.width;
                if snake_head_idx + self.width == treshold {
                    SnakeCell(treshold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_head_idx + self.width)
                }
            }
        };
    }
}
