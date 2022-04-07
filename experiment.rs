use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    P1(Level),
    P2(Level),
    Emp,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Level {
    Single,
    Double,
}

#[derive(Debug, PartialEq)]
enum PlayerTurn {
    P1,
    P2,
}

fn main() {
    let mut stats: HashMap<(i8, i8), Tile> = HashMap::new();

    stats.insert((0, 0), P1(Single));
    stats.insert((0, 0), P2(Double));
    stats.insert((0, 0), Emp);
    println!("{:#?}", stats);
}

impl Tile {
    fn init_fill(&mut self) {
        
    }
    fn empty(&mut self, x_and_y: (u8, u8)) {
        
    }
    fn update(&mut self, before_x_and_y: (u8, u8), after_x_and_y: (u8, u8)) {
        
    }
}