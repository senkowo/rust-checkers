use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Tile {
    P1(Stack),
    P2(Stack),
    Emp,
}

#[derive(Debug, PartialEq)]
enum Stack {
    Single,
    Double,
}

#[derive(Debug, PartialEq)]
enum PlayerTurn {
    P1,
    P2,
}

const X_LEN: u8 = 8;
const Y_LEN: u8 = 8;

impl Tile {
    fn init_fill_hashmap(stats: &mut HashMap<(u8, u8), Tile>) {
        for y in 1..=Y_LEN {
            for x in 1..=X_LEN {
                stats.insert(
                    (x, y),
                    if ((x + (y % 2)) % 2) == 1 {
                        match y {
                            1..=3 => Tile::P1(Stack::Single),
                            6..=8 => Tile::P2(Stack::Single),
                            _ => Tile::Emp,
                        }
                    } else {
                        Tile::Emp
                    },
                );
            }
        }
    }
    fn change_tile_st(&mut self, variant: Tile) {
        *self = variant;
    }
    fn change_tile_state(
        stats: &mut HashMap<(u8, u8), Tile>,
        location: (u8, u8),
        variant: Tile,
    ) {
        *stats.get_mut(&location).expect("OwO whats this?") = variant;
    }
}

fn main() {
    introduction();

    let mut stats: HashMap<(u8, u8), Tile> = HashMap::new();

    Tile::init_fill_hashmap(&mut stats);

    //stats.get(&(0, 0)).unwrap().init_fill(&mut stats);
    // Tile::update(&mut stats, (2, 3), &Tile::P1(Stack::Double));
    // stats.insert((4, 4), Tile::Emp);

    // i could just change the end with a equals Tile::Emp...
    stats.get_mut(&(1, 1)).unwrap().change_tile_st(Tile::Emp);
    // simplified, abstraction, maybe put the above in the function?
    Tile::change_tile_state(&mut stats, (8, 8), Tile::Emp);

    println!("{:?}", stats);
}

fn introduction() {}

impl Tile {
    fn update(stats: &mut HashMap<(u8, u8), Tile>, xy: (u8, u8), v: &Tile) {
        //*stats.entry((xy.0, xy.1)).or_insert(Tile::Emp) = *v;
        stats.insert((xy.0, xy.1), Tile::P1(Stack::Double));
    }
}
