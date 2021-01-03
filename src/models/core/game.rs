use crate::models::opengl::{block_face::BlockFace, block_face_type::BlockFaceType, direction::Direction};

use super::{grid::Grid, slot::Slot};

pub struct Game {
    player: Vec<(usize, usize)>,
    width: usize,
    height: usize,
    pub grid: Grid,
    pub vertices: Vec<f32>
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

        grid.set(1, 1, Slot::Apple);
        let player = vec![(width / 2, height / 2)];
        grid.set(player[0].0, player[0].1, Slot::Snake);

        Game { player, grid, width, height, vertices: vec![] }
    }

    pub fn update(&mut self) {
        //self.player.push((self.player[0].0 + 1, self.player[0].1));
        self.grid.set(self.player[0].0, self.player[0].1, Slot::Air);
        self.player[0].0 = self.player[0].0 + 1;
        self.grid.set(self.player[0].0, self.player[0].1, Slot::Snake);

        self.gen_mesh();
    }

    pub fn gen_mesh(&mut self) {
        let mut vertices = Vec::new();
        for x in 0..16 {
            for y in 0..16 {
                let slot = self.grid.get(x, y);
                if slot == Slot::Air {
                    continue;
                }
                
                vertices.append(&mut Game::generate_block_vertices(slot, x as f32, y as f32, x == self.player[0].0 && y == self.player[0].1));
            }
        }
        self.vertices = vertices;
    }

    fn generate_block_vertices(slot: Slot, x: f32, y: f32, is_head: bool) -> Vec<f32> {
        let mut vertices = Vec::new();
        let mut block_face_type;
        if is_head {
            block_face_type = BlockFaceType::SnakeHead;
        } else {
            block_face_type = match slot {
                Slot::Wall => BlockFaceType::Wall,
                Slot::Snake => BlockFaceType::Snake,
                Slot::Apple => BlockFaceType::Apple,
                Slot::Air => panic!("Attempted to create block face from Slot::Air")
            };
        }
        
        let top = BlockFace::new(block_face_type, Direction::Top)
            .transform(x, 0.0, y);

        // only top is BlockFaceType::SnakeHead for head, other faces are 
        // BlockFaceType::Snake 
        if is_head {
            block_face_type = BlockFaceType::Snake;
        }

        let right = BlockFace::new(block_face_type, Direction::Right)
            .transform(x, 0.0, y);

        let left = BlockFace::new(block_face_type, Direction::Left)
            .transform(x, 0.0, y);

        let front = BlockFace::new(block_face_type, Direction::Front)
            .transform(x, 0.0, y);

        let back = BlockFace::new(block_face_type, Direction::Back)
            .transform(x, 0.0, y);

        let bottom = BlockFace::new(block_face_type, Direction::Bottom)
            .transform(x, 0.0, y);

        vertices.append(&mut top.vertices.to_vec());
        vertices.append(&mut right.vertices.to_vec());
        vertices.append(&mut left.vertices.to_vec());
        vertices.append(&mut front.vertices.to_vec());
        vertices.append(&mut back.vertices.to_vec());
        vertices.append(&mut bottom.vertices.to_vec());
        vertices
    }
}