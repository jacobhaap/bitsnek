#[derive(PartialEq, Debug)]
pub enum Slot {
    Air,
    Wall,
    Snek,
    Apple
}

pub fn u8_to_slot(n: u8) -> Slot {
    match n {
        0 => Slot::Air,
        1 => Slot::Wall,
        2 => Slot::Snek,
        3 => Slot::Apple,
        _ => panic!(format!("Invalid integer to map to Slot (received {})", n))
    }
}

pub fn slot_to_u8(slot: Slot) -> u8 {
    match slot {
        Slot::Air => 0,
        Slot::Wall => 1,
        Slot::Snek => 2,
        Slot::Apple => 3
    }
}
