// for deleting items/taking ownership from hashmap:
// https://stackoverflow.com/questions/43416196/return-exact-value-in-rust-hashmap
//

use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Write};
// .flush() depends on std::io::Write

#[derive(Debug)]
struct Tile {
    state: Occupancy,
    level: Level,
}

#[derive(Debug, Copy, Clone)]
enum Occupancy {
    Emp,
    P1,
    P2,
}

#[derive(Debug, Copy, Clone)]
enum Level {
    Emp,
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

    clear();

    // Hashmap "stats" stores a key of tuple (i8, i8), which represents
    // the x and y coordinates of a given tile on the checkerboard.
    // For the value of each respective key, there is an instance of
    // struct "Tile", which stores the enum tile state (e.g. occupied by
    // Player 1, Player 2, or empty) and the enum for the type of piece
    // on that tile (e.g. single, double, or empty).
    let mut stats: HashMap<(i8, i8), Tile> = HashMap::new();

    // essentially puts all the pieces in the right place by filling in
    // the keys and values of "stats" appropriately.
    for y in 0..8 {
        for x in 0..8 {
            stats.insert((x, y), initialize_pieces(x, y));
        }
    }

    let mut whos_turn = PlayerTurn::P1;
    let mut player_goes_again = false;
    loop {
        clear();
        print_board(&stats);
        let (full_move_argument, enter_pressed_in_second_play, escape_current_entry) =
            input_full_coords(&whos_turn, player_goes_again, &stats);

        if enter_pressed_in_second_play {
            change_current_player(&mut whos_turn);
            player_goes_again = false;
            continue;
        }

        if escape_current_entry {
            player_goes_again = true;
            continue;
        }

        //println!("full move argument: {:?}", full_move_argument); //debug

        // logic: check if first coord belongs to P1, then if the
        // destination is a valid movement (also consider double).
        let valid = logic_check(&whos_turn, &full_move_argument, &stats);
        //println!("logic_check: {:?}", valid); //debug

        //print_board(&stats);

        if !valid {
            println!("invalid input, please try again.");
            continue;
        }

        player_goes_again = logic_move(&whos_turn, &full_move_argument, &mut stats);
        if player_goes_again {
            continue;
        } else {
            player_goes_again = false;
        }
        // now, make it so if player_goes_again is true, player
        // gets to go again. perhaps declare as mutable at the
        // top, so can start at the top of loop, and print line
        // if player gets to go again, and allow the option for
        // the user to skip by entering without arguments. Maybe
        // make this action always available...? Actually, maybe
        // that wont work... check rules for checkers...

        change_current_player(&mut whos_turn);
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

fn initialize_pieces(x: i8, y: i8) -> Tile {
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
                level: Level::Emp,
            },
        }
    } else {
        Tile {
            state: Occupancy::Emp,
            level: Level::Emp,
        }
    }
}

fn input_full_coords(
    whos_turn: &PlayerTurn,
    player_goes_again: bool,
    stats: &HashMap<(i8, i8), Tile>,
) -> (((i8, i8), (i8, i8)), bool, bool) {
    let mut full_move_action: Vec<char> = Vec::new();

    let first_output_of_chars: Vec<char> =
        input_single_coords(1, player_goes_again, &whos_turn, &stats);
    println!("log: first_output_of_chars?: {:#?}", first_output_of_chars);
    match first_output_of_chars.get(0) {
        Some(v) => {
            if *v == 'e' {
                println!("log: char e received");
                return (((0, 0), (0, 0)), true, false);
            }
        }
        None => {} // convoluted, but it works
                   // true, false means that blank enter was pressed...
    }
        for v in first_output_of_chars.iter() {
        full_move_action.push(*v);
    }

    //println!("full_move action before second: {:?}", full_move_action); //debug

    if full_move_action.len() == 2 {
        let second_output_of_chars = input_single_coords(2, player_goes_again, whos_turn, &stats);
        match second_output_of_chars.get(0) {
            Some(v) => {
                if *v == 'x' {
                    println!("log: char x received");
                    return (((0, 0), (0, 0)), false, true);
                }
            }
            None => {}
        }
for v in second_output_of_chars.iter() {
            full_move_action.push(*v);
        }
    }

    // What's happening below:
    // |> ((char, char), (char, char))
    // |=> ((u32, u32), (u32, u32))
    // |==> ((i8, i8), (i8, i8))
    // chaos.
    (
        (
            (
                full_move_action[0] // is char (e.g. '4')
                    .to_digit(10) // method to_digit() returns u32.
                    .unwrap()
                    .try_into() // method try_into() returns appropriate (u8).
                    .unwrap(),
                full_move_action[1]
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
            (
                full_move_action[2]
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap(),
                full_move_action[3]
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
        ),
        false,
        false,
    )
}
fn input_single_coords(
    first_or_second: u8,
    player_goes_again: bool,
    whos_turn: &PlayerTurn,
    stats: &HashMap<(i8, i8), Tile>,
) -> Vec<char> {
    'outer: loop {
        if player_goes_again {
            print!(
                "{} goes again. Press \"enter\" without arguments to end turn.",
                if matches![whos_turn, PlayerTurn::P1] {
                    "Player 1"
                } else {
                    "Player 2"
                }
            );
        }
        ioflush();
        print!(
            "\n{}: {}",
            if matches![whos_turn, PlayerTurn::P1] {
                "Player 1"
            } else {
                "Player 2"
            },
            if first_or_second == 1 {
                "Input piece location or full move argument: "
            } else {
                "Input move location: "
            }
        );
        ioflush();
        let input = user_input();
        println!("single user input {:?}", input);
        if player_goes_again {
            match &input[..] {
                // messy code
                "" | "end" => {
                    println!("log: e is being returned from single input");
                    sleep(2);
                    return vec!['e'];
                }
                _ => {}
            }
        }
        println!("is it first or second: {:?}", first_or_second);
        if first_or_second == 2 {
            match &input[..] {
                "esc" => {
                    println!("log: detect esc, return 'x' ");
                    sleep(2);
                    return vec!['x'];
                }
                _ => {}
            }
        }

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

        //println!("output_as_chars: {:?}", output_as_chars); //debug

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

fn logic_check(
    whos_turn: &PlayerTurn,
    double_coodinates: &((i8, i8), (i8, i8)),
    stats: &HashMap<(i8, i8), Tile>,
) -> bool {
    let ((a, b), (c, d)) = double_coodinates;
    //println!("stats status: {:?}", stats.get(&(*a, *b)).unwrap().state); //debug

    // checks if the first coordinate is the current player's piece
    // and if the second coordinate is empty.
    match whos_turn {
        &PlayerTurn::P1 => {
            if matches![stats.get(&(*a, *b)).unwrap().state, Occupancy::P1] {
                //println!("coord 1 is correct"); //debug
                if matches![stats.get(&(*c, *d)).unwrap().state, Occupancy::Emp] {
                    //println!("coord 2 is correct"); //debug
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        &PlayerTurn::P2 => {
            if matches![stats.get(&(*a, *b)).unwrap().state, Occupancy::P2] {
                //println!("coord 1 is correct"); //debug
                if matches![stats.get(&(*c, *d)).unwrap().state, Occupancy::Emp] {
                    //println!("coord 2 is correct"); //debug
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    // checks if the move performed is a single movement diagonally,
    // and if the piece being moved is a double, backwards movement
    // is also accepted. Also, if the piece moves 2 spaces diagonally,
    // it checks if there is an enemy piece in between the original
    // and moved direction. If yes, the enemy piece is removed, and
    // the player gets to make another move. Also, after a capture
    // is performed, make it so the player gets to chose whether or
    // not he would like to try again (type "end" to end turn).

    let mut check_for_capture = false;
    // check if the movement in the y-direction is appropriate.
    match whos_turn {
        &PlayerTurn::P1 => match stats.get(&(*a, *b)).unwrap().level {
            Level::Single => match d - b {
                1 => {}
                2 => check_for_capture = true,
                _ => return false,
            },
            Level::Double => match d - b {
                1 | -1 => {}
                2 | -2 => check_for_capture = true,
                _ => return false,
            },
            Level::Emp => {
                panic!("error occurred at logic test 2.0");
            }
        },
        &PlayerTurn::P2 => match stats.get(&(*a, *b)).unwrap().level {
            Level::Single => match d - b {
                -1 => {}
                -2 => check_for_capture = true,
                _ => return false,
            },
            Level::Double => match d - b {
                1 | -1 => {}
                2 | -2 => check_for_capture = true,
                _ => return false,
            },
            Level::Emp => {
                panic!("error occurred at logic test 2.1");
            }
        },
    }

    // check if the movement in the x direction is appropriate,
    // relative to the y.
    match d - b {
        1 | -1 => match c - a {
            1 | -1 => {}
            _ => return false,
        },
        2 | -2 => match c - a {
            2 | -2 => {}
            _ => return false,
        },
        _ => return false,
    }

    // if move 2 spaces diagonally, then check what it jumped over.
    // if it jumped over an empty space or your own piece, then
    // return false.
    if check_for_capture {
        match stats.get(&((a + c) / 2, (b + d) / 2)).unwrap().state {
            Occupancy::Emp => return false,
            Occupancy::P1 => {
                if matches![whos_turn, PlayerTurn::P1] {
                    return false;
                }
            }
            Occupancy::P2 => {
                if matches![whos_turn, PlayerTurn::P2] {
                    return false;
                }
            }
        }
    }

    true // the default output if it survives all the checks
}
fn logic_move(
    whos_turn: &PlayerTurn,
    double_coodinates: &((i8, i8), (i8, i8)),
    stats: &mut HashMap<(i8, i8), Tile>,
) -> bool {
    //println!("whos_turn: {:?}", whos_turn); //debug
    //println!("double_coordinates: {:?}", double_coodinates); //debug
    //println!("stats: {:?}", stats); //debug

    let ((a, b), (c, d)) = double_coodinates;

    // first, change the state(P1/emp) and level(Single/emp) of
    // the initial and destination moves.

    // create/new (from Emp)
    stats.insert(
        (*c, *d),
        Tile {
            state: stats.get(&(*a, *b)).unwrap().clone().state,
            level: stats.get(&(*a, *b)).unwrap().clone().level,
        },
    );
    // overwrite initial
    stats.insert(
        (*a, *b),
        Tile {
            state: Occupancy::Emp,
            level: Level::Emp,
        },
    );

    // then, if it jumps over a piece (already confirmed to be
    // enemy's), then remove that piece (state.Occupancy::Emp)
    match d - b {
        2 | -2 => {
            stats.insert(
                ((a + c) / 2, (b + d) / 2),
                Tile {
                    state: Occupancy::Emp,
                    level: Level::Emp,
                },
            );
            true
        }
        // if an enemy piece is taken, return true and if not,
        // return false. The function return goes into variable
        // "player_goes_again: bool".
        _ => false,
    }
}

fn print_board(stats: &HashMap<(i8, i8), Tile>) {
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
fn change_current_player(whos_turn: &mut PlayerTurn) {
    if matches![whos_turn, PlayerTurn::P1] {
        *whos_turn = PlayerTurn::P2;
    } else {
        *whos_turn = PlayerTurn::P1;
    }
}
