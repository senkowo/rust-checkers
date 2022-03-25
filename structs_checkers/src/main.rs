
extern crate multimap;

use multimap::MultiMap;


struct Data {
    tile_stat: String,
    piece_stat: String,
}

fn main() {
    println!("Hello, world!");
    let mut stats: MultiMap<String, Vec<Data>> = MultiMap::new();

    for y in 0..10 {
        for x in 0..10 {
            stats.insert(
                String::from(format!("{}_{}", x.to_string(), y.to_string())),
                vec![
                    let format!("{}_{}", x.to_string(), y.to_string()) = Data {
                        tile_stat: init_pieces_location(x, y),
                        piece_stat: String::from("single"),
                    }
                ],
            );
        }
    }

    println!("{:#?}", stats);
}

fn init_pieces_location(x: u8, y: u8) -> String {
    String::from(if ((x + (y % 2)) % 2) == 1 {
        match y {
            0..=2 => "p1",
            3..=6 => "emp",
            7..=9 => "p2",
            _ => {
                println!("error");
                "emp"
            }
        }
    } else {
        "emp"
    })
}
