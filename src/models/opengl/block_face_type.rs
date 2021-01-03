// each possible block face used for texturing 
// in world mesh rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockFaceType {
    Wall,
    Snake,
    SnakeHeadUpLeft,
    SnakeHeadUpRight,
    Apple
}