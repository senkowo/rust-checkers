// a beginner project in rust

use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Tile {
    P1(Stack),
    P2(Stack),
    Emp,
}

#[derive(Debug, PartialEq)]
enum Stack {
    Single,
    Double,
}

#[derive(Debug, PartialEq)]
enum PlayerTurn {
    P1,
    P2,
}

#[derive(Debug, PartialEq)]
enum InputReturn {
    VecCoords(Vec<u8>),
    End,
    Retype,
}

const X_LEN: u8 = 8;
const Y_LEN: u8 = 8;

impl Tile {
    // sets up the checkerboard by putting all the pieces in their default places
    fn init_fill_hashmap(stats: &mut HashMap<(u8, u8), Tile>) {
        for y in 1..=Y_LEN {
            for x in 1..=X_LEN {
                stats.insert(
                    (x, y),
                    if ((x + (y % 2)) % 2) == 1 {
                        match y {
                            1..=3 => Tile::P1(Stack::Single),
                            6..=8 => Tile::P2(Stack::Single),
                            _ => Tile::Emp,
                        }
                    } else {
                        Tile::Emp
                    },
                );
            }
        }
    }
    // changes the given tile state to another
    fn change_tile_state(
        stats: &mut HashMap<(u8, u8), Tile>,
        location: (u8, u8),
        variant: Tile
    ) {
        // the first expression returns a mutable reference to the value of the
        // given HashMap key, which is overwritten by Tile instance "variant"
        *stats.get_mut(&location).expect("OwO whats this?") = variant;
    }
}

fn main() {
    // runs the init menu system
    introduction();

    // creates a hashmap that contains information for every tile on the
    // checkerboard respectively (e.g. player occupation and piece type)
    let mut stats: HashMap<(u8, u8), Tile> = HashMap::new();
    Tile::init_fill_hashmap(&mut stats);

    // debug: test
    Tile::change_tile_state(&mut stats, (2, 1), Tile::Emp);
    println!("{:#?}", stats);
    print_board(&stats);


    // Things to consider:
    // enter pressed in another turn
    // esc pressed in second coordinate input


    // set user-turn (enum)
    let mut current_player = PlayerTurn::P1;
    let mut is_another_turn = false;
    'outer: loop {
        let mut vec_coords = Vec::new();
        loop {
            ////maybe have little in this loop, most in outer?
            // print board
            // receive user input (if not full, two)
            //      (enter pressed in another turn?) (esc pressed in coord input?)
            // vvv For input_coords(), print the print statements out of the loop,
            //  so you don't need to push whos_turn and stuff (keep it simple)
            vec_coords = input_coords();
            // logic check
            // logic implement
            // check if promote to king
            // check if game over
            // (exit loop if player captured a piece?)
        }
        // change turn

    }
}

fn introduction() {
    clear();
    println!(
        "\n\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        // Random ascii art
        "⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⢠⡀⠀⠀⡠⠑⠁⠀⠀⠀⠀⠀⠀⠀⠙⠿⣿⣿⣽⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠈⠢⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        "⠀⠀⠀⠀⢸⠀⠀⠀⠀⠰⠚⠘⡠⠊⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠀⠀⠈⠛⠻⠿⠿⣿⣿⣿⣿⣿⣿⣤⣀⠀⠀⠐⡀⠀⠀⠀⠀⠀⠀⠀\n",
        "⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠁⠀⠙⠓⠒⠢⠼⠤⠤⠤⠤⠤⢀⣀\n",
        "⠀⠀⠀⠀⠀⡆⠀⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠌⢠⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
        "⠀⠀⠀⠀⠀⢡⠀⠀⡰⠁⠀⠀⠀⢀⠀⠀⠀⠀⢀⡌⢠⡎⠀⠀⠀⠀⠀⠀⠀⠀⡔⠁⣰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡄⠀⠀⠀⠀⠀\n",
        "⠀⠀⠀⠀⠀⢊⠂⠰⠁⠀⠀⠀⢀⠂⠀⠀⣀⠔⢱⢠⠃⡇⠀⠀⠀⠀⠀⠀⢀⠎⠀⠔⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢓⠞⠁⠀⠀⠀\n",
        "⠀⢀⠔⠂⠁⠙⣄⠃⢠⠃⠀⠀⠆⠀⠀⢀⠊⠑⣦⡇⠀⡇⠀⠀⠀⠀⠀⣤⠃⢀⠊⠀⢸⠀⢰⠀⠀⠀⠀⠀⠀⠀⠀⠠⠃⠀⠀⠀⠀⠀\n",
        "⢠⠚⠀⠀⠀⠀⡘⠀⢾⠀⠀⠸⠀⠀⢠⠃⠀⠀⡟⠀⠀⢰⠀⠀⠀⠀⡰⠏⡩⠉⠉⠉⢹⠀⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊\n",
        "⡜⠀⠀⠀⠀⠀⡇⠸⠣⠀⠀⡇⠀⢸⠓⠶⠶⣤⣁⡀⠀⠘⡄⠀⠀⡔⢙⠜⠀⠀⠀⠀⢸⢰⠀⠀⠀⠀⠀⢠⠀⠀⠀⠀⠀⠀⡠⠊⠀⠀\n",
        "⢸⠀⠀⠀⠀⢰⠀⠇⢀⠀⠀⣇⢀⡮⣂⡤⠴⢖⠛⠁⠀⠀⠱⡀⠜⠀⠁⠶⣿⡛⠛⠛⢻⠖⠂⠀⠀⠀⠀⡌⠀⠀⠀⢀⡠⢊⠀⠀⠀⠀\n",
        "⠈⢑⠀⠀⠀⢸⢸⠀⢸⠀⢠⡇⣸⠂⠀⠌⠈⠀⠀⠀⠀⠂⠀⠈⠀⠀⡀⠄⡊⠛⢷⣤⡌⡀⠀⠀⠀⠀⡐⠀⠀⢀⠔⡿⣗⢜⡄⠀⠀⠀\n",
        "⡤⠖⠛⠓⠒⢺⡖⠔⠊⡆⢸⡇⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⡂⠄⠨⡘⠈⠀⠀⠀⠀⡰⠁⣀⢤⡶⠷⠉⠁⠉⠑⠄⠤⡀\n",
        "⠀⠀⠀⠀⠀⠈⡇⠀⠀⠱⢸⠠⠘⡄⠀⠀⠀⠀⠀⠀⠀⣶⠀⠀⠀⠀⠀⠀⠀⢀⡔⠀⠀⠀⠀⠐⡰⣁⠝⠠⡋⠁⠀⠀⠀⠀⠀⠀⠀⠸\n",
        "⠀⠀⢀⠔⠒⢄⠃⠀⠀⠀⢹⠀⢰⡟⢦⡀⠀⠀⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⢠⠌⠀⠀⢀⠔⠀⡐⡙⠀⡜⢨⠀⠀⠀⠀⠀⠀⠀⠀⠀⢘\n",
        "⢀⠔⠁⠀⣀⣈⣂⡀⠀⠀⠘⡀⣶⡇⠀⡝⡦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠃⠀⢀⡴⡟⢀⠌⣰⠁⡰⡇⢨⡀⠀⠀⠀⠀⢠⠀⠀⠀⠔\n",
        "⡏⠀⢠⣾⣿⡦⠤⠸⡄⠀⠀⢻⢸⠱⠀⡇⠑⡈⠢⣤⣤⣀⣤⣤⣤⣤⡖⠀⢀⠔⣽⠏⠈⡠⢞⠆⡐⠁⡇⢠⠢⢄⠀⠀⠀⢸⣇⡠⠚⠀\n",
        "⢇⢰⣿⣿⣿⡇⠐⠈⠉⡆⠀⡏⢸⠀⠑⢼⡀⠈⣢⣀⠬⠟⠛⠉⠁⡘⠀⡰⠓⠰⠧⠶⢿⠌⠊⠊⠀⠀⠸⠆⠀⠀⠉⠓⠉⠉⣿⡀⠀⢀\n",
        "⢸⣿⣿⣿⣿⡇⢰⣂⡰⠁⢰⠀⢸⣀⡀⠈⠫⡁⠀⠀⠀⠀⠀⠀⢰⢁⢮⠃⠀⠀⢠⡊⢻⡀⠀⢀⣀⣀⣀⠃⠀⠀⠀⠀⠀⠀⠈⢻⣿⣿\n",
        "⢸⣿⣿⣿⣿⣧⠀⡆⠀⢀⠔⡆⠸⠀⢈⠕⠒⡮⡂⠀⠀⠀⠀⠀⡠⠃⡆⢀⡀⣀⡰⠗⠒⢲⠋⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⢻⣿\n",
        "⣿⣿⣿⣿⣿⣿⠞⠀⢠⠋⠀⠑⢀⡇⡌⠀⠀⠠⠒⠀⠀⠀⠒⢶⠁⢸⠀⠎⠉⠀⠀⠀⠀⡎⠀⠀⠀⠀⠀⠀⠈⣦⠀⠀⠀⠀⠀⠀⠀⠙\n",
        "⣿⣿⣿⣿⣿⡇⢠⢠⠃⠀⠀⠀⢀⠜⠀⣠⠐⠈⠀⠀⠀⠀⠀⠘⡀⡆⠀⠀⠀⠀⠀⠀⡸⠀⠀⠀⠀⠀⠀⠀⠀⢻⠀⠀⠀⠀⠀⠀⠀⠀\n",
        "⣿⣿⣿⣿⡿⠁⡜⡎⠀⠀⠀⣀⠜⠀⠀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠘⡧⡀⠀⠀⠀⣀⠀⠇⣆⡠⡀⠀⠀⠀⠀⠀⢸⡄⠀⠀⠀⠀⠀⠀⠀\n",
        "⣿⣿⣿⡟⠁⠀⡿⡤⠔⠂⠁⠀⠀⠀⢸⡀⠀⠀⠀⠀⠀⠀⣠⠴⡀⡖⠓⠀⠀⡜⠁⠛⠫⡀⠀⠘⡄⠀⠀⠀⠀⠈⡇⠀⠀⠀⠀⠀⠀⠀\n",
        "⣿⣿⠏⠀⢀⡼⠉⠀⠀⠀⠀⠀⠀⠀⠀⠃⠀⠀⠀⢀⠔⠋⠀⠀⠑⠁⠀⡔⠉⠑⢄⠀⣀⣼⡇⠀⠰⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀\n",
        "⠟⠁⠀⡰⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⠤⠄⠰⠁⠀⠀⠀⠀⠀⠀⡎⠳⠄⠀⠀⢸⣿⣿⣿⣦⣠⣷⣾⣿⣿⣷⣾⠀⠀⠀⠀⠀⠀⠀\n",
        "\n\t\tAwoo~!\n\t\t>Cute spash-screen<"
    );
    sleep(2);
    clear();
    intro_scripts("menu");
    loop {
        let uwu = user_input();
        clear();
        match &uwu[..] {
            "" | "s" | "start" => break,
            _ => intro_scripts(&uwu),
        }
    }
}

fn intro_scripts(input: &str) {
    clear();
    match input {
        "" | "s" | "start" => panic!("Error: it should never pass through here"),
        "m" | "menu" => println!(
            "{}{}{}{}{}{}{}{}{}{}",
            "\n\n\n\tcli-checkers\n\n\n",
            "\tCommands:\n",
            "\tEnter \"s\" or \"start\" or \"\" (no arguments) to start the game\n",
            "\tEnter \"m\" or \"menu\" to return to main menu\n",
            "\tEnter \"c\" or \"commands\" to list all commands\n",
            "\tEnter \"h\" or \"help\" for game instructions\n",
            "\tEnter \"i\" or \"info\" for information about the project\n",
            "\tEnter \"OwO\" or \"owo\" for secret\n",
            "\tEnter \"UwU\" or \"uwu\" for secret\n\n\n",
            "\tPress enter key to begin"
        ),
        "c" | "commands" => println!(
            "{}{}{}{}{}{}{}{}",
            "\n\n\tList of Commands Available:\n\n",
            "\t\"m\" | \"menu\"\n",
            "\t\"s\" | \"start\" | \"\" (no args)\n",
            "\t\"c\" | \"commands\"\n",
            "\t\"h\" | \"help\"\n",
            "\t\"i\" | \"info\"\n",
            "\t\"OwO\" | \"owo\"\n",
            "\t\"UwU\" | \"uwu\"",
        ),
        "h" | "help" => println!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "\n\n\tWelcome to cli-checkers!\n\n",
            "\tOwO what's this?\n",
            "\tThis is a two-player game where the objective is to capture\n",
            "\tall of the opponent's pieces.\n\n",
            "\tWhen performing a move, you first enter the coordinates of\n",
            "\tthe piece you wish to move; then, the destination coordinates.\n",
            "\tThere are several ways to enter coordinates:\n\n",
            "\t\t\"12\" :enter: \"23\" :enter:\n",
            "\t\t\"1223\" :enter:\n\n",
            "\tBoth of the examples given moves a piece at coordinates (1, 2)\n",
            "\tto (2, 3).\n\n",
            "\tSpaces and letters are not read when entering coordinates, so\n",
            "\tyou can even do \"e621 :3\" and this will be read as\n",
            "\t(6, 2) => (1, 3). These are not realistically possible\n",
            "\tmovements to perform, however.\n\n",
            "\tIf you entered the initial coordinate but wish to go back and\n",
            "\tchange it, you can enter \"esc\" to go back to the previous\n",
            "\tinput prompt.\n\n",
            "\tLike ordinary Checkers, your piece will gain the ability to\n",
            "\tmove backwards once you reach the other side of the board.\n\n",
            "\tOnce you capture an enemy piece, you will have the opportunity\n",
            "\tto perform another action. If you wish not to, simply hit\n",
            "\t:enter: with no arguments and your turn will end.\n\n",
            "\tEntering \"exit\" will terminate the program at any time."
        ),
        "i" | "info" => println!(
            "{}{}{}{}{}{}{}{}{}{}",
            "\n\n\n\tI apologize for the following:\n\n",
            "\thewwo~ dis is a simpwe pwoject i made in Wust (Rust).\n",
            "\ti am stiww a beginnew in Wust (Rust) so it's nyot the\n",
            "\tpwettiest piece of code, but i weawned a wot fwom dis!\n",
            "\tweawning Wust came at the cost of becoming a femboy fuwwy,\n",
            "\tbut I came to weawwy wuv Wust!!\n",
            "\tit's onwy a mattew of time untiw I get mysewf a paiw of\n",
            "\tPwogwamming Socks: a must nyeed for aww pwogwammews!\n",
            "\ti don't knyow why i am wwiting dis on my AP Pewfowmance\n",
            "\tTask, but I wondew what the AP Gwadews wiww think of dis!\n",
        ),
        "OwO" | "owo" | "Owo" => println!("\nUwU"),
        "UwU" | "uwu" | "Uwu" => println!("\nOwO"),
        _ => println!(
            "\n\n\n\nCommand not found: \"{}\"\n{}",
            input, "See the list of available commands: \"c\" | \"commands\""
        ),
    }
    print!("\n\n\n\nCommand: ");
    ioflush();
}

fn input_coords() -> Vec<u8> {
    vec![1, 2, 3]
}





// prints the UI/board according to HashMap "stats"
fn print_board(stats: &HashMap<(u8, u8), Tile>) {
    // spaghetti-code UI printing algorithm:
    println!("    ,______ ______ ______ ______ ______ ______ ______ ______,");
    for y in (1..=Y_LEN).rev() {
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
            for x in 1..=X_LEN {
                for (key, value) in stats {
                    // "*k" or "&(x, y)"???
                    if *key == (x, y) {
                        match value {
                            Tile::P1(Single) => print!(" OOOO |"),
                            Tile::P1(Double) => print!(" 0KK0 |"),
                            Tile::P2(Single) => print!(" //// |"),
                            Tile::P2(Double) => print!(" \\KK\\ |"),
                            Tile::Emp => print!("      |"),
                        }
                        ioflush();
                    }
                }
            }
            // only here to create newline
            println!("");
        }
        if y == 1 {
            continue;
        };
        println!("    |------ ------ ------ ------ ------ ------ ------ ------|");
    }
    println!("    '-------------------------------------------------------'");
    println!("        1      2      3      4      5      6      7      8   ");
}

fn sleep(s: u64) {
    std::thread::sleep(std::time::Duration::from_secs(s));
}
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    ioflush();
}
fn ioflush() {
    let _ = io::stdout().flush().expect("could not flush");
}
fn user_input() -> String {
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("OwO what's this? Failed to read line");
    ret.pop();

    if ret == "exit" {
        clear();
        panic!("exit command used, crashed program");
    }

    ret
}
