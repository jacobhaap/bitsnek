use glfw::{Action, Key};
use rand::Rng;

use crate::models::{opengl::{block_face::BlockFace, block_face_type::BlockFaceType, block_face_direction::BlockFaceDirection}};

use super::{direction::Direction, grid::Grid, slot::Slot};

pub struct Game {
    segments: Vec<(usize, usize)>,
    apple: (usize, usize),
    direction: Direction,
    next_direction: Direction,
    pub width: usize,
    pub height: usize,
    pub grid: Grid,
    pub vertices: Vec<f32>,
    pub apple_vertices: Vec<f32>,
    pub score: u32,
    pub lost: bool
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        let mut grid = Grid::new(width, height);
        for x in 0..width {
            grid.set(x, 0, Slot::Wall);
            grid.set(x, height - 1, Slot::Wall);
        }

        for y in 0..height {
            grid.set(0, y, Slot::Wall);
            grid.set(width - 1, y, Slot::Wall); 
        }

        let player_x = width / 2;
        let player_y = height / 2;
        let segments = vec![(player_x, player_y), (player_x - 1, player_y), (player_x - 2, player_y)];
       
        let apple = Game::get_apple_loc(&segments, width, height);

        let default_direction = Direction::Right;
        Game { lost: false, score: 0, grid, segments, width, height, apple, vertices: vec![], apple_vertices: vec![], direction: default_direction, next_direction: default_direction }
    }

    pub fn update(&mut self) {
        //self.player.push((self.player[0].0 + 1, self.player[0].1))
        let mut next_position = self.segments[self.segments.len() - 1];

        match self.direction {
            Direction::Up => {
                next_position.1 += 1;
            },
            Direction::Down => {
                next_position.1 -= 1;
            },
            Direction::Right => {
                next_position.0 -= 1;
            },
            Direction::Left => {
                next_position.0 += 1;
            }
        }
        if self.grid.get(next_position.0, next_position.1) == Slot::Wall
            || self.segments.contains(&next_position) {
            self.lost = true;
            return;
        }

        self.score += 5;
        self.segments.push(next_position);

        if self.apple == next_position {
            self.apple = Game::get_apple_loc(&self.segments, self.width, self.height);
            self.score += 100;
        } else {
            // only delete last segment if apple was not eaten
            self.segments.remove(0);
        }

        self.direction = self.next_direction;
        self.gen_mesh();
    }

    pub fn get_apple_loc(segments: &Vec<(usize, usize)>, width: usize, height: usize) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..width - 1);
        let y = rng.gen_range(1..height - 1);
        if segments.contains(&(x, y)) {
            return Game::get_apple_loc(&segments, width, height);
        }
        (x, y)
    } 

    pub fn process_keyboard(&mut self, key: Key, action: Action) {
        if action != Action::Press {
            return;
        }

        self.next_direction = match key {
            Key::Up => {
                if self.direction == Direction::Down {
                    return;
                }
                Direction::Up
            },
            Key::Down => {
                if self.direction == Direction::Up {
                    return;
                }
                Direction::Down
            },
            Key::Right => {
                if self.direction == Direction::Left {
                    return;
                }
                Direction::Right
            },
            Key::Left => {
                if self.direction == Direction::Right {
                    return;
                }
                Direction::Left
            },
            _ => self.next_direction
        }
    }

    pub fn gen_mesh(&mut self) {
        let mut vertices = Vec::new();
        for x in 0..16 {
            for y in 0..16 {
                let slot = self.grid.get(x, y);
                if slot == Slot::Air {
                    continue;
                }
                vertices.append(&mut self.generate_block_vertices(slot, x as f32, y as f32, false));
            }
        }

        for i in 0..self.segments.len() - 1 {
            let segment = self.segments[i];
            vertices.append(&mut self.generate_block_vertices(Slot::Snake, segment.0 as f32, segment.1 as f32, false));
        }

        let segment = self.segments[self.segments.len() - 1];
        vertices.append(&mut self.generate_block_vertices(Slot::Snake, segment.0 as f32, segment.1 as f32, true));

        self.vertices = vertices;
        self.apple_vertices = self.generate_block_vertices(Slot::Apple, self.apple.0 as f32, self.apple.1 as f32, false);
    }

    fn generate_block_vertices(&self, slot: Slot, x: f32, y: f32, is_head: bool) -> Vec<f32> {
        let mut vertices = Vec::new();
        let mut block_face_type;
        if is_head {
            block_face_type = match self.direction {
                Direction::Up => BlockFaceType::SnakeHeadUpLeft,
                Direction::Down | Direction::Right => BlockFaceType::SnakeHeadUpLeft,
                Direction::Left => BlockFaceType::SnakeHeadUpRight
            };
        } else {
            block_face_type = match slot {
                Slot::Wall => BlockFaceType::Wall,
                Slot::Snake => BlockFaceType::Snake,
                Slot::Apple => BlockFaceType::Apple,
                Slot::Air => panic!("Attempted to create block face from Slot::Air")
            };
        }
        
        let top = BlockFace::new(block_face_type, BlockFaceDirection::Top)
            .transform(x, 0.0, y);
        vertices.append(&mut top.vertices.to_vec());

        // only top is BlockFaceType::SnakeHead for head, other faces are 
        // BlockFaceType::Snake 
        if is_head {
            block_face_type = BlockFaceType::Snake;
        } else if slot == Slot::Apple {
            return vertices;
        }

        let right = BlockFace::new(block_face_type, BlockFaceDirection::Right)
            .transform(x, 0.0, y);

        let left = BlockFace::new(block_face_type, BlockFaceDirection::Left)
            .transform(x, 0.0, y);

        let front = BlockFace::new(block_face_type, BlockFaceDirection::Front)
            .transform(x, 0.0, y);

        let back = BlockFace::new(block_face_type, BlockFaceDirection::Back)
            .transform(x, 0.0, y);

        let bottom = BlockFace::new(block_face_type, BlockFaceDirection::Bottom)
            .transform(x, 0.0, y);

        vertices.append(&mut right.vertices.to_vec());
        vertices.append(&mut left.vertices.to_vec());
        vertices.append(&mut front.vertices.to_vec());
        vertices.append(&mut back.vertices.to_vec());
        vertices.append(&mut bottom.vertices.to_vec());
        vertices
    }
}