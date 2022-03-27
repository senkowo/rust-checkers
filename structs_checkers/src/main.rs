use std::collections::HashMap;
use std::convert::TryInto;
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

#[derive(Debug)]
enum PlayerTurn {
    P1,
    P2,
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
    for y in 0..8 {
        for x in 0..8 {
            stats.insert((x, y), initialize_pieces(x, y));
        }
    }

    print_board(&stats);


    let mut whos_turn = PlayerTurn::P1;
    loop {
        print_board(&stats);
        println!(
            "\n{}'s turn!",
            if matches![whos_turn, PlayerTurn::P1] {
                "Player 1"
            } else {
                "Player 2"
            }
        );
        let full_move_argument = input_full_coords();
        let ((a, b), (c, d)) = full_move_argument;
        let a: u8 = a.try_into().unwrap();
        let b: u8 = b.try_into().unwrap();
        let c: u8 = c.try_into().unwrap();
        let d: u8 = d.try_into().unwrap();
        let full_move_argument = ((a, b), (c, d));

        println!("full move argument: {:?}", full_move_argument);

        // logic: check if first coord belongs to P1, then if the
        // destination is a valid movement (also consider double).
        let valid: bool = logic_check(&full_move_argument, &stats);


        if matches![whos_turn, PlayerTurn::P1] {
            whos_turn = PlayerTurn::P2;
        } else {
            whos_turn = PlayerTurn::P1;
        }

    }
}

fn introduction() {
    clear();
    println!("\n\n<Insert Introduction/Spash Screen>");
    sleep(1);
    clear();
    println!(
        "{}{}{}{}",
        "\n\n\n\tcli-checkers\n\n\n",
        "\tType \"h\" for how to play the game\n\n\n\n",
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
            5..=7 => Tile {
                state: Occupancy::P2,
                level: Level::Single,
            },
            _ => Tile {
                state: Occupancy::Emp,
                level: Level::NA,
            },
        }
    } else {
        Tile {
            state: Occupancy::Emp,
            level: Level::NA,
        }
    }
}

fn input_full_coords() -> ((u32, u32), (u32, u32)) {
    let mut full_move_action: Vec<char> = Vec::new();

    let first_output_of_chars = input_single_coords(1);
    for v in first_output_of_chars.iter() {
        full_move_action.push(*v);
    }

    println!("full_move action before second: {:?}", full_move_action);

    if full_move_action.len() == 2 {
        let second_output_of_chars = input_single_coords(2);
        for v in second_output_of_chars.iter() {
            full_move_action.push(*v);
        }
    }

    // ((char, char), (char, char)) => ((u32, u32), (u32, u32))
    (
        (
            full_move_action[0].to_digit(10).expect("not a number"),
            full_move_action[1].to_digit(10).expect("not a number"),
        ),
        (
            full_move_action[2].to_digit(10).expect("not a number"),
            full_move_action[3].to_digit(10).expect("not a number"),
        ),
    )
}
fn input_single_coords(first_or_second: u8) -> Vec<char> {
    'outer: loop {
        if first_or_second == 1 {
            print!("\nInput piece to move or full move argument: ");
        } else {
            print!("\nInput move location: ");
        }
        ioflush();
        let input = user_input();

        let input_as_chars: Vec<char> = input.chars().collect();
        let number_chars: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut output_as_chars: Vec<char> = Vec::new();
        for c in input_as_chars.iter() {
            if *c == '8' || *c == '9' {
                println!(
                    "Error: The number {} is not on the board. Please try again.",
                    c
                );
                continue 'outer;
            }
            if number_chars.contains(c) {
                output_as_chars.push(*c);
            }
        }

        println!("output_as_chars: {:?}", output_as_chars);

        match first_or_second {
            1 => {
                if (output_as_chars.len() == 2) || (output_as_chars.len() == 4) {
                    return output_as_chars;
                } else {
                    println!("incorrect input; can only be 2 or 4 numbers.");
                }
            }
            2 => {
                if output_as_chars.len() == 2 {
                    return output_as_chars;
                } else {
                    println!("incorrent input; can only be 2 numbers.");
                }
            }
            _ => println!("error at first_or_second"),
        };
    }
}

fn logic_check(double_coodinates: &((u8, u8), (u8, u8)), stats: &HashMap<(u8, u8), Tile>) -> bool {
    true
}

fn print_board(stats: &HashMap<(u8, u8), Tile>) {
    // spaghetti-code UI printing algorithm:
    println!("    ,______ ______ ______ ______ ______ ______ ______ ______,");
    for y in (0..8).rev() {
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
            for x in 0..8 {
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
        println!("    |------ ------ ------ ------ ------ ------ ------ ------|");
    }
    println!("    '--------------------------------------------------------'");
    println!("        0      1      2      3      4      5      6      7   ");
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
