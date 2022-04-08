use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    P1(Stack),
    P2(Stack),
    Emp,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Stack {
    Single,
    Double,
}

#[derive(Debug, PartialEq)]
enum PlayerTurn {
    P1,
    P2,
}

const X_LEN = 8;
const Y_LEN = 8;

impl Tile {
    fn init_fill_hashmap(stats: &mut HashMap<(u8, u8), Tile>) {
        for y in 1..X_LEN {
            for x in 1..X_LEN {
                stats.insert(
                    (x, y),
                    if ((x + (y % 2)) % 2) == 1 {
                        match y {
                            1..=3 => Tile::P1(Stack::Single),
                            6..=8 => Tile::P2(Stack::Single),
                            _ => Tile::Emp,
                        }
                    } else {
                        Tile::Emp,
                    }
                )
            }
        }
    }
}

fn main() {

    introduction();

    let mut stats: HashMap<(u8, u8), Tile> = HashMap::new();
    
    Tile::init_fill_hashmap(&mut stats);

    println!(
        "test: {:#?}\n{:#?}",
        match stats.get(&(0, 0)) {
            None => panic!("OwO whats this? Failed to .get() stats"),
            Some(i) => i,
        },
        stats,
    );
    //stats.get(&(0, 0)).unwrap().init_fill(&mut stats);
    // Tile::update(&mut stats, (2, 3), &Tile::P1(Stack::Double));
    // stats.insert((4, 4), Tile::Emp);
    
    println!("{:#?}", stats);
}

fn introduction() {
    
}

impl Tile {
//    fn init_fill(&self, &mut ) {
    //    stats
//    }
    fn empty(&self, x_and_y: (u8, u8)) -> Tile {
        Tile::P1(Stack::Double)
    }
    fn update(
        stats: &mut HashMap<(u8, u8), Tile>,
        xy: (u8, u8),
        v: &Tile,
    ) {
        //*stats.entry((xy.0, xy.1)).or_insert(Tile::Emp) = *v;
        stats.insert((xy.0, xy.1), Tile::P1(Stack::Double));
    }
}