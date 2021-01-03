use crate::models::{opengl::block_face_type::BlockFaceType};

use super::block_face_direction::BlockFaceDirection;

pub struct BlockFace {
    pub vertices: [f32; 30]
}

impl BlockFace {
    pub fn new(face: BlockFaceType, direction: BlockFaceDirection) -> BlockFace {
        let vt_left = face_to_vt(face) - 2.0;
        let vt_right = vt_left + 1.0;
        let vt_top = 0.0;
        let vt_bottom = 1.0;
        let vertices = match direction {
            BlockFaceDirection::Top => [
                -0.5,  0.5, -0.5,  vt_left,  vt_bottom,  // bottom-left
                 0.5,  0.5, -0.5,  vt_right, vt_bottom,  // bottom-right
                 0.5,  0.5,  0.5,  vt_right, vt_top,     // top-right
                 0.5,  0.5,  0.5,  vt_right, vt_top,     // top-right
                -0.5,  0.5,  0.5,  vt_left,  vt_top,     // top-left
                -0.5,  0.5, -0.5,  vt_left,  vt_bottom   // bottom-left
            ],
            BlockFaceDirection::Bottom => [
                -0.5, -0.5, -0.5,  vt_left,  vt_bottom, // bottom-left
                 0.5, -0.5, -0.5,  vt_right, vt_bottom, // bottom-right
                 0.5, -0.5,  0.5,  vt_right, vt_top, // top-right
                 0.5, -0.5,  0.5,  vt_right, vt_top, // top-right
                -0.5, -0.5,  0.5,  vt_left,  vt_top, // top-left
                -0.5, -0.5, -0.5,  vt_left,  vt_bottom  // bottom-left
            ],
            BlockFaceDirection::Left => [
                -0.5,  0.5,  0.5,  vt_left, vt_top, // top-right
                -0.5,  0.5, -0.5,  vt_right, vt_top, // bottom-right
                -0.5, -0.5, -0.5,  vt_right,  vt_bottom, // bottom-left
                -0.5, -0.5, -0.5,  vt_right,  vt_bottom, // bottom-left
                -0.5, -0.5,  0.5,  vt_left, vt_bottom, // bottom-right
                -0.5,  0.5,  0.5,  vt_left, vt_top  // top-right
            ],
            BlockFaceDirection::Right => [
                0.5,  0.5,  0.5,  vt_left, vt_top, // top-right
                0.5,  0.5, -0.5,  vt_right, vt_top, // bottom-right
                0.5, -0.5, -0.5,  vt_right,  vt_bottom, // bottom-left
                0.5, -0.5, -0.5,  vt_right,  vt_bottom, // bottom-left
                0.5, -0.5,  0.5,  vt_left, vt_bottom, // bottom-right
                0.5,  0.5,  0.5,  vt_left, vt_top  // top-right
            ],
            // turn 90 degrees
            BlockFaceDirection::Front => [
                -0.5, -0.5, -0.5,  vt_left,  vt_bottom, // bottom-left
                 0.5, -0.5, -0.5,  vt_right, vt_bottom, // bottom-right
                 0.5,  0.5, -0.5,  vt_right, vt_top,    // top-right
                 0.5,  0.5, -0.5,  vt_right, vt_top,    // top-right
                -0.5,  0.5, -0.5,  vt_left,  vt_top,    // top-left
                -0.5, -0.5, -0.5,  vt_left,  vt_bottom  // bottom-left
            ],
            BlockFaceDirection::Back => [
                -0.5, -0.5,  0.5,  vt_left,  vt_bottom, // bottom-left
                 0.5, -0.5,  0.5,  vt_right, vt_bottom, // bottom-right
                 0.5,  0.5,  0.5,  vt_right, vt_top, // top-right
                 0.5,  0.5,  0.5,  vt_right, vt_top, // top-right
                -0.5,  0.5,  0.5,  vt_left,  vt_top, // top-left
                -0.5, -0.5,  0.5,  vt_left,  vt_bottom  // bottom-left
            ]
        };
        BlockFace { vertices }
    }

    pub fn transform(&self, x: f32, y: f32, z: f32) -> BlockFace {
        let mut vertices = self.vertices.clone();
        for (i, v) in vertices.iter_mut().enumerate() {
            *v = match i % 5 {
                0 => *v + x,
                1 => *v + y,
                2 => *v + z,
                _ => *v
            }
        }
        BlockFace { vertices }
    }
}

fn face_to_vt(slot: BlockFaceType) -> f32 {
    match slot {
        BlockFaceType::Wall => 0.0,
        BlockFaceType::Snake => 1.0,
        BlockFaceType::SnakeHeadUpLeft => 2.0,
        BlockFaceType::SnakeHeadUpRight => 4.0,
        BlockFaceType::Apple => 5.0
    }
}