use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    P1(Stack),
    P2(Stack),
    Emp,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Stack {
    pub Single,
    pub Double,
}

#[derive(Debug, PartialEq)]
enum PlayerTurn {
    P1,
    P2,
}

fn main() {
    let mut stats: HashMap<(i8, i8), Tile> = HashMap::new();

    stats.insert((0, 0), Tile::P1(Single));
    stats.insert((1, 0), Tile::P2(Double));
    stats.insert((2, 0), Tile::Emp);
    println!(
        "{:#?}\n{:#?}",
        match stats.get(&(0, 0)) {
            None => panic!("OwO whats this? Failed to .get() stats"),
            Some(i) => i,
        },
        stats,
    );
    //stats.get(&(0, 0)).unwrap().init_fill(&mut stats);
}

impl Tile {
    fn init_fill(&self, &mut ) {
    //    stats
    }
    fn empty(&self, x_and_y: (u8, u8)) -> Tile {
    //    Tile {
    //        Tile::P1(Stack::Double)
    //    }
    }
    fn update(&mut self, before_x_and_y: (u8, u8), after_x_and_y: (u8, u8)) {
        
    }
}