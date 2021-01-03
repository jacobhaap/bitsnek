use super::slot::{Slot, slot_to_u8, u8_to_slot};

// two-dimensional arbitrary n-by-k map to represent grid
pub struct Grid {
    map: Vec<u8>,
    height: usize
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        if height % 4 != 0 {
            panic!("Height must be divisible by 4 for grid");
        } else if width % 4 != 0 {
            panic!("Width must be divisible by 4 for grid");
        }

        // each section of map is represented with 2 bits
        // with 4 grid sections per byte
        let map = vec![slot_to_u8(Slot::Air); (height * width) / 4];
        Grid { map, height }
    }

    pub fn get(&self, x: usize, y: usize) -> Slot {
        let byte = self.map[y * self.height / 4 + x / 4];
        let index_in_byte = x % 4;

        let mask = Grid::create_mask(index_in_byte);

        u8_to_slot((byte & !mask) >> (6 - index_in_byte * 2)) 
    }

    pub fn set(&mut self, x: usize, y: usize, slot: Slot) {
        let map_index = y * self.height / 4 + x / 4;
        let byte = self.map.get_mut(map_index).unwrap();
        let index_in_byte = x % 4;

        let n = slot_to_u8(slot);

        let mask = Grid::create_mask(index_in_byte);

        *byte = (*byte & mask) | (n << (3 - index_in_byte) * 2);
    }

    fn create_mask(index: usize) -> u8 {
        match index {
            0 => 0b00111111,
            1 => 0b11001111,
            2 => 0b11110011,
            3 => 0b11111100,
            _ => panic!("Invalid index for mask creation (0-3)")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::core::{grid::Grid, slot::Slot};

    #[test]
    fn get_and_set() {
        let mut map = Grid::new(16, 16);
        map.set(1, 1, Slot::Wall);
        assert_eq!(map.get(1, 1), Slot::Wall);
        assert_eq!(map.get(0, 0), Slot::Air);
        assert_ne!(map.get(1, 0), Slot::Wall);
    }
}