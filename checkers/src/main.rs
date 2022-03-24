use std::collections::HashMap;
use std::io::{self, Write};
//use std::io::Write;

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
    io::stdin().read_line(&mut uwu).expect("Failed to read line");

    //* Maybe I could put the vectors below in an impl! 

    
    // The following creates a list of coordinate names and puts them in a vector
    // e.g. 0_0, 3_2, 9_9; where "x-coords_y-coords".
    let mut board_coords: Vec<String> = Vec::new();
    for x in 0..10 {
        for y in 0..10 {
            board_coords.push(format!("{}_{}", x.to_string(), y.to_string()));
        }
    }
    println!("{:#?}", board_coords); //debug

    // The following creates the default board tile state ("emp" as in empty) (alts: "p1" and "p2")
    let mut default_states = Vec::new();
    for _ in 0..board_coords.len() {
        default_states.push(String::from("emp"));
    }
    println!("{}", default_states.len()); //debug

    // The following creates a Hash Map from combining the two previous vectors.
    // The hash map stores the following: HashMap<"x_y coordinate", "tile occupancy">
    // This will later be used to keep track of the board and its pieces, as well as
    //   in checking if a given movement option is possible.
    let mut stats: HashMap<_, _> = board_coords.iter().to_owned().zip(default_states.iter().to_owned()).collect();
    println!("{:#?}", stats); //debug


    
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

fn print_board(stats: &HashMap<&String,&String>) {
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
    '------ ------ ------ ------ ------ ------ ------ ------ ------ ------'
        0      1      2      3      4      5      6      7      8      9

    foo"
    );

    // printing algorithm:
    println!(",______ ______ ______ ______ ______ ______ ______ ______ ______ ______,");
    
    println!("|"); 

    let mut val = "error";
    for (k, v) in stats {
        println!("{}", *k);
        if *k == ("0_0") {
            val = *v;
            println!("break");
            break; 
        }
    }
    println!("{}", val);
    
    if val == "emp" {
        println!("emp found!");
        print!("      ");
        let _ = io::stdout().flush();
    }
}
