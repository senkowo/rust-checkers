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
    // struct "Tile", which stores the tile state (e.g. occupied by
    // Player 1, Player 2, or empty) and the type of piece on that tile
    // (e.g. single, double, or NA).
    let mut stats: HashMap<(u8, u8), Tile> = HashMap::new();

    // essentially puts all the pieces in the right place by filling in
    // the keys and values of "stats" appropriately.
    for y in 0..10 {
        for x in 0..10 {
            stats.insert((x, y), initialize_pieces(x, y));
        }
    }

    //for (key, value) in &stats {
    //    println!("Key: {:?}\tValue: {:?}", key, value);
    //}

    print_board(&stats);

    let mut full_move_action = Vec::new();

    let first_ret = input_first_coord();
    for v in first_ret.iter() {
        full_move_action.push(v)
    }
    println!("full_move action before second: {:#?}", full_move_action);


    input_second_coord();
}

fn introduction() {
    clear();
    println!("\n\n<Insert Introduction/Spash Screen>");
    sleep(1);
    clear();
    println!(
        "{}{}{}{}",
        "\n\n\tcli-checkers\n\n\n",
        "\tType \"h\" for how to play the game\n\n\n",
        "\t<insert ascii art here>\n\n\n\n",
        "\tPress enter key to begin"
    );
    loop {
        print!("\n\n\n\n\nCommand: ");
        ioflush();
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
        "h" => println!(
            "{}{}{}{}",
            "\n\n\tWelcome to cli-checkers!\n\n",
            "\tThis is a two player game where both players",
            " take turns making a move.\n",
            "\tThere are several ways to perform a move:"
        ),
        _ => println!("error"),
    }
}

fn initialize_pieces(x: u8, y: u8) -> Tile {
    if ((x + (y % 2)) % 2) == 1 {
        match y {
            0..=2 => Tile {
                state: Occupancy::P1,
                level: Level::Single,
            },
            3..=6 => Tile {
                state: Occupancy::Emp,
                level: Level::NA,
            },
            7..=9 => Tile {
                state: Occupancy::P2,
                level: Level::Single,
            },
            _ => {
                println!("error!!!");
                Tile {
                    state: Occupancy::Emp,
                    level: Level::NA,
                }
            }
        }
    } else {
        Tile {
            state: Occupancy::Emp,
            level: Level::NA,
        }
    }
}

fn input_first_coord() -> Vec<char> {
    loop {
        print!("Input move argument (e.g. \"3 2\", \"32\", \"32 54\", \"3254\"): ");
        ioflush();
        let input = user_input();

        let mut input_as_chars: Vec<char> = input.chars().collect();
        let number_chars: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut chars_to_remove_from_number_chars: Vec<usize> = Vec::new();
        for (i, c) in input_as_chars.iter_mut().enumerate() {
            if !number_chars.contains(c) {
                chars_to_remove_from_number_chars.push(i);
            }
        }
        println!("chars_to_remove_from_number_chars: {:#?}", chars_to_remove_from_number_chars);
        for i in chars_to_remove_from_number_chars {
            input_as_chars.remove(i);
            println!("i value: {:#?}", i);
        }

        println!("input_as_chars: {:#?}", input_as_chars);

        if (input_as_chars.len() == 2) || (input_as_chars.len() == 4) {
            return input_as_chars;
        }
    }
}
fn input_second_coord() {

}



fn print_board(stats: &HashMap<(u8, u8), Tile>) {
    // spaghetti-code UI printing algorithm:
    println!("    ,______ ______ ______ ______ ______ ______ ______ ______ ______ ______,");
    for y in (0..10).rev() {
        for s in 0..2 {
            print!(
                "  {} |",
                if s == 1 {
                    y.to_string()
                } else {
                    String::from(" ")
                }
            );
            ioflush();
            for x in 0..10 {
                for (k, v) in stats {
                    // "*k" or "&(x, y)"???
                    if *k == (x, y) {
                        match v.state {
                            Occupancy::Emp => {
                                print!("      |");
                                ioflush();
                            }
                            Occupancy::P1 => {
                                print!(" 0000 |");
                                ioflush();
                            }
                            Occupancy::P2 => {
                                print!(" //// |");
                                ioflush();
                            }
                        }
                    }
                }
            }
            // only here to create newline
            println!("");
        }
        if y == 0 {
            continue;
        };
        println!("    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|");
    }
    println!("    '---------------------------------------------------------------------'");
    println!("        0      1      2      3      4      5      6      7      8      9");
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
