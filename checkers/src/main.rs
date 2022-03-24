use std::collections::HashMap;
use std::io::{self, Write};

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


    // The following creates a Hash Map that stores the following: 
    //         HashMap<'x_y coords', 'tile occupancy'>
    // This will later be used to keep track of the board and its pieces, as well as
    // in checking if a given movement option is possible. 
    
    let mut stats = HashMap::new();

    for y in 0..10 {
        for x in 0..10 {
            stats.insert(
                String::from( format!("{}_{}", x.to_string(), y.to_string()) ),
                String::from("emp")
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

fn print_board(stats: &HashMap<String, String>) {
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
                let mut val = "error";
                for (k, v) in stats {
                    if k == &format!("{}_{}", x, y) {
                        val = v;
                        break;
                    }
                }
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
