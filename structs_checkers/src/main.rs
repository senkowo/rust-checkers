use std::collections::HashMap;
use std::io::{self, Write};
// .flush() depends on std::io::Write

#[derive(Debug)]
struct Tile {
    state: Occupancy,
    level: Level,
}

#[derive(Debug)]
enum Occupancy {
    Emp,
    P1,
    P2,
}

#[derive(Debug)]
enum Level {
    NA,
    Single,
    Double,
}

fn main() {
    introduction();

    // Hashmap "stats" stores a key of tuple (u8, u8), which represents
    // the x and y coordinates of a given tile on the checkerboard.
    // For the value of each respective key, there is an instance of
    // struct called "Tile", which stores the tile state (e.g. occupied
    // by Player 1, Player 2, or empty) and the type of piece on that
    // tile (e.g. single, double, or NA).
    let mut stats: HashMap<(u8, u8), Tile> = HashMap::new();

    for x in 0..10 {
        for y in 0..10 {
            stats.insert(
                (x, y),
                Tile {
                    state: Occupancy::Emp,
                    level: Level::NA,
                },
            );
        }
    }

    for (key, value) in stats.into_iter() {
        println!("Key: {:?}\tValue: {:?}", key, value);
    }
}

fn introduction() {
    clear();
    println!("\n\n<Insert Introduction/Spash Screen>");
    sleep(2);
    clear();
    println!("{}{}{}",
             "\n\n\tcli-checkers\n\n",
             "\tType \"h\" for how to play the game\n\n",
             "\t<insert ascii art here>\n\n\n\n",
             "\tPress enter key to begin"
    );
    loop {
        let uwu = user_input();
        match &uwu[..] {
            "h" => intro_help(&uwu),
            _ => break,
        }
    }
}
fn intro_help(input: &str) {
    clear();
    match input {
        "h" => println!("{}{}{}{}",
                        "\n\n\tWelcome to cli-checkers!\n\n",
                        "\tThis is a two player game where both players",
                        " take turns making a move.\n",
                        "\tThere are several ways to perform a move:"
        ),
        _ => println!("error"),
    }
}

fn sleep(s: u64) {
    std::thread::sleep(std::time::Duration::from_secs(s));
}
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}
fn ioflush() {
    let _ = io::stdout().flush();
}
fn user_input() -> String {
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("OwO what's this? Failed to read line");
    ret.pop();
    ret
}
