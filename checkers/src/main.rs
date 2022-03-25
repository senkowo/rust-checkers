
// Documentation for MultiMaps:
// https://docs.rs/multimap/0.8.3/multimap/

extern crate multimap;

use std::io::{self, Write};
//use std::collections::HashMap;
use multimap::MultiMap;


fn main() {
    clear();
    println!("\n\nHewwo~");
    println!("Initiawizing... pls waitt :3");
    sleep(2);
    clear();
    println!(
        "\n\n\tcli-checkers uwu\n\n\n\n\n
            <insert ascii art here>\n\n\n\n\n
            \tPress enter key to begin"
    );
    
    // waits for enter key
    let mut uwu = String::new();
    io::stdin()
        .read_line(&mut uwu)
        .expect("Failed to read line");


    // The following creates a MultiMap (similar to hashmap, but with more than 
    // two values) that stores the following: 
    //         HashMap<'x_y coords', Vec<'tile occupancy', 'piece type'>>
    // This will later be used to keep track of the board and its pieces, as well as
    // in checking if a given movement option is possible. 
    
    let mut stats: MultiMap<String, Vec<String>> = MultiMap::new();

    for y in 0..10 {
        for x in 0..10 {
            stats.insert(
                String::from( format!("{}_{}", x.to_string(), y.to_string()) ),
                vec![String::from("emp"), String::from("single")],
            );
        }
    }
    // modified config; add before adding emp?

    println!("{:#?}", stats);

    

    print_board(&stats);
}

fn stats_config() {
    
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

    for reference"
    );

    // spaghetti-code printing algorithm:
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
