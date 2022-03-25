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
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    clear();
    println!("\n\nHewwo");
    println!("Initializing... pls wait");
    sleep(2);
    clear();
    println!(
        "\n\n\tcli-checkers OwO\n\n\n\n\n
            <insert ascii art here>\n\n\n\n\n
            \tPress enter key to begin"
    );

    // waits for enter key
    let mut uwu = String::new();
    io::stdin()
        .read_line(&mut uwu)
        .expect("Failed to read line");

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
                    // init_pieces_status initializes the location of player
                    // 1's and player 2's pieces, respectively. The code is
                    // spaghetti, so I thought I should put it in a separate fn.
                    String::from(init_pieces_status(x, y)),
                    String::from("single"),
                ],
            );
        }
    }

    println!("{:#?}", stats); //debug

    print_board(&stats);


    let mut input: String = user_input();
    for i in 0..3 {
        match &stats.get(&input[..]) {
            Some(occupancy) => {
                println!("occupancy: {}", occupancy[0]);
                break;
            },
            None => println!("error, {:#?}", &input)
        }

        if i == 0 {
            let byte_list = input.as_bytes();
            let byte_possible_nums = vec![b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            let mut new_string = String::new();

            for (i, &byte_ch) in byte_list.iter().enumerate() {
                if byte_possible_nums.contains(&byte_ch) {
                    match std::str::from_utf8(&byte_list) {
                        Ok(ch) => {
                            &new_string.push_str(ch);
                            println!("new ch {}", ch);
                        },
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                }
                //if i == 1 {
                //    new_byte.insert(b" ");
                //}
                
            }
            println!("second test {:#?}", new_string);
        }
    
    }

    //println!("{}", stats[format!("{}_{}", )][0])

    //now change value?

    
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

fn logic() {
        
}

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
