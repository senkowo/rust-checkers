// Documentation for MultiMaps:
// https://docs.rs/multimap/0.8.3/multimap/
// Documentation for HashMaps:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// https://users.rust-lang.org/t/can-we-make-vector-of-structs-if-yes-than-how-to-make-and-use-it/19476
// I should've done this instead, maybe?
// Maybe fork it and try it again using structs later. + impl.
// Maybe I could change the vector of strings into a vector of
// structs.

extern crate multimap;

use multimap::MultiMap;
//use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // Introduction:
    clear();
    println!("\n\nHewwo");
    println!("Initializing... pls wait");
    sleep(2);
    clear();
    println!(
        "\n\n\tcli-checkers uwu\n\n\n\n\n
            <insert ascii art here>\n\n\n\n\n
            \tPress enter key to begin"
    );

    // waits for enter key
    let mut _uwu = String::new();
    _uwu = user_input();

    // The following creates a MultiMap (similar to hashmap but with more than
    // two values) that stores the following:
    //         MultiMap<'x_y coords', Vec<'tile occupancy', 'piece type'>>
    // This will later be used to keep track of the board and its pieces, as well
    // as in checking if a given movement option is available.

    let mut stats: MultiMap<String, Vec<String>> = MultiMap::new();

    for y in 0..10 {
        for x in 0..10 {
            stats.insert(
                String::from(format!("{}_{}", x.to_string(), y.to_string())),
                vec![
                    // init_pieces_status() initializes player 1's and player 2's
                    // pieces. The code is spaghetti, so I thought I should put
                    // it in a separate function.
                    String::from(init_pieces_status(x, y)),
                    String::from("single"),
                ],
            );
        }
    }

    //println!("{:#?}", stats); //debug

    print_board(&stats);

    // user input for coordinates

    let input: String = user_input();
    let mut input_mod = String::new();

    for i in 0..2 {

        input_mod = input_alternate_input_syntax_check(&input, i);

        match &stats.get(&input_mod[..]) {
            Some(occupancy) => {
                println!("occupancy: {}", occupancy[0]);
                break;
            }
            None => println!("error, {:#?}", &input),
        }
    }
}

fn init_pieces_status(x: u8, y: u8) -> String {
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

fn input_alternate_input_syntax_check(input: &String, i: u8) -> String {
    // or instead, I could make it so there is a vector array of bites 0-9,
    // and if one letter matches, add that letter to another vector array/
    // or add it to a string, and then return that (might want here too?).
    // So both initial and end coordinates are received and put in all
    // together at once? maybe I could have both available: if single
    // coordinate, prompt for another... or maybe return tuple from this
    // function... or maybe after returning "x_y x_y", separate the two
    // and return with the separated half, and push that into the next
    // prompt.
    // If I use structs, that would eliminate the need to create a String
    // to represent coordinates, and instead have "x: u8, y: u8" or a
    // tuple. Multiple types too, in contract to Vec's one type.


    match i {
        0 => {
            // syntax for "x_y" or "x y" or "x-y"
            let char_vec: Vec<char> = input.chars().collect();
            if char_vec.len() == 3 {
                format!("{}_{}", char_vec[0], char_vec[2] ).to_string()
            } else {
                input.clone()
            }
        },
        1 => {
            // syntax for "xy"
            let char_vec: Vec<char> = input.chars().collect();
            if char_vec.len() == 2 {
                format!("{}_{}", char_vec[0], char_vec[1] ).to_string()
            } else {
                input.clone()
            }
        },
        _ => {
            input.clone()
        },
    }
}

fn logic() {}

fn user_input() -> String {
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    ret.pop();
    ret
}

fn sleep(s: u64) {
    std::thread::sleep(std::time::Duration::from_secs(s));
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H"); // escape sequence for clearing terminal
}

fn ioflush() {
    let _ = io::stdout().flush();
}

fn print_board(stats: &MultiMap<String, Vec<String>>) {
    println!(
        "\n
    for reference:
    ,______ ______ ______ ______ ______ ______ ______ ______ ______ ______,
    |      |      |      |      |      |      |      |      |      |      |
 9  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 8  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 7  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 6  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 5  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 4  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 3  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 2  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 1  |      |      |      |      |      |      |      |      |      |      |
    |------ ------ ------ ------ ------ ------ ------ ------ ------ ------|
    |      |      |      |      |      |      |      |      |      |      |
 0  |      |      |      |      |      |      |      |      |      |      |
    '---------------------------------------------------------------------'
        0      1      2      3      4      5      6      7      8      9
    
    "
    );

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
                let mut val = String::new();
                for (k, v) in stats {
                    if k == &format!("{}_{}", x, y) {
                        val = v[0][0].clone();
                        //println!("\nCurrent ret {:#?}\n", val);
                        break;
                    }
                }
                //println!("Outer ret {:#?}", val);
                if val == "emp" {
                    print!("      |");
                    ioflush();
                } else if val == "p1" {
                    print!(" OOOO |");
                    ioflush();
                } else if val == "p2" {
                    print!(" //// |");
                    ioflush();
                } else {
                    println!("error");
                }
            }
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
