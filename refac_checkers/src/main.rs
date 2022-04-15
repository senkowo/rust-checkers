// a beginner project in rust

use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    P1(Stack),
    P2(Stack),
    Emp,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Stack {
    Single,
    Double,
}

#[derive(Debug, PartialEq)]
enum PlayerTurn {
    Player1,
    Player2,
}

// note: change to InputOK?
#[derive(Debug, PartialEq)]
enum OkInput {
    Norm,
    End,
    Retype,
}

// scuffed attempt at making a custom macro (macros are hard umu).
// the macro "print!()" requires a "stdout().flush()" afterwards to work
// properly, so making a macro that does both ("printf!()") is nice.
macro_rules! printf {
    ($($str:expr),*) => {{
        print!($($str),*);
        ioflush()
    }};
}

type InputResult = Result<OkInput, Error>;

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
    // changes the given tile's state to another
    fn change_tile_state(stats: &mut HashMap<(u8, u8), Tile>, location: (u8, u8), variant: &Tile) {
        // the first expression returns a mutable reference to the value of the
        // given HashMap key, which is overwritten by Tile instance "variant"
        *stats.get_mut(&location).expect("OwO whats this?") = *variant;
    }
}

impl PlayerTurn {
    fn change(&mut self) {
        match self {
            PlayerTurn::Player1 => *self = PlayerTurn::Player2,
            PlayerTurn::Player2 => *self = PlayerTurn::Player1,
        }
    }
}

fn main() {
    // runs the init menu system
    introduction();

    /* HashMap "stats" stores a key of tuple (i8, i8), which represents the
     * x and y coordinates of a given tile on the checkerboard. For the value
     * of each respective key, there is an instance of enum "Tile", which
     * stores information about whether there is a piece on a given tile and
     * which player it belongs to (the variants are: P1(Single/Double),
     * P2(Single/Double), Emp). */
    let mut stats: HashMap<(u8, u8), Tile> = HashMap::new();
    Tile::init_fill_hashmap(&mut stats);

    // the loop below is the main process that repeats for the duration of
    // the game.
    let mut current_player = PlayerTurn::Player1;
    let mut is_another_turn = false;
    'outer: loop {
        clear();
        print_board(&stats);
        let mut vec_coords = Vec::new();

        while vec_coords.len() != 4 {
            // dialogue to play before input_coords
            if is_another_turn {
                printf!(
                    "\n{} goes again. Press \"enter\" without arguments to end turn.",
                    match current_player {
                        PlayerTurn::Player1 => "Player 1",
                        PlayerTurn::Player2 => "Player 2",
                    }
                );
            }
            printf!("\n{}: {}",
                match current_player {
                    PlayerTurn::Player1 => "Player 1",
                    PlayerTurn::Player2 => "Player 2",
                },
                match vec_coords.len() {
                    0 => "Enter move coordinates\n\t(e.g. [x, y] : \"23:enter:34\" or \"2334\"): ",
                    2 => "Input destination coordinates\n\t(e.g. \"34\") (Note: enter \"esc\" to cancel): ",
                    _ => panic!("OwO whats this? pls fix dis devewopew!"),
                }
            );

            let returned_option_enum = input_coords(&mut vec_coords);

            match returned_option_enum {
                Ok(OkInput::Norm) => (), // leave blank?
                Ok(OkInput::End) => {
                    if is_another_turn {
                        current_player.change();
                        is_another_turn = false;
                        continue 'outer;
                    } else {
                        // if no input and another_turn false, return err(NoInput)
                        error_code(Error::NoInput);
                        sleep(2);
                        continue 'outer;
                    }
                }
                Ok(OkInput::Retype) => {
                    continue 'outer;
                }
                Err(e) => {
                    error_code(e);
                    sleep(2);
                    continue 'outer;
                }
            }
        }

        // logic_check returns true if the movement could be performed.
        if logic_check(&stats, &vec_coords, &current_player) {
            // logic_move returns whether an enemy piece was captured and if the
            // player should play another turn.
            if logic_move(&mut stats, &vec_coords) {
                is_another_turn = true;
            } else {
                is_another_turn = false;
            }
        } else {
            error_code(Error::InvalidCoordinates);
            sleep(2);
            continue 'outer;
        }

        // checks if a piece needs to be promoted to king.
        check_if_promote_to_king(&mut stats);

        // checks if the game should end.
        if check_if_game_over(&stats) {
            clear();
            println!("Game Over!");
            break 'outer;
        }

        // if current player did not capture an enemy piece this turn, change.
        if !(is_another_turn) {
            current_player.change();
        }
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
            "\t\t\"23\" :enter: \"34\" :enter:\n",
            "\t\t\"2334\" :enter:\n\n",
            "\tBoth of the examples given moves a piece at coordinates (2, 3)\n",
            "\tto (3, 4).\n\n",
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
            "\n\n\n\nCommand not found: \"{}\"\nSee the list of available commands: \"c\" | \"commands\"",
            input
        ),
    }
    printf!("\n\n\n\nCommand: ");
}

fn input_coords(vec_coords: &mut Vec<u8>) -> InputResult {
    // requests user input, then filters out all chars that are not numbers
    let input: String = user_input().chars().filter(|c| c.is_digit(10)).collect();

    if input.is_empty() || input == "end" {
        return Ok(OkInput::End);
    }
    if input == "esc" {
        return Ok(OkInput::Retype);
    }

    /* creates a vec of chars from input. Then, it goes through all the chars
     * and converts them into int (u32). Then, it is converted to u8, so it
     * can be added to vec_coords. */
    let input_as_chars: Vec<char> = input.chars().collect();
    for v in input_as_chars.iter() {
        match v.to_digit(10) {
            Some(i) => vec_coords.push(match u32_to_u8(i) {
                Ok(as_u8) => as_u8,
                Err(conv_err) => return Err(conv_err),
            }),
            None => return Err(Error::InputContainsChar),
        };
    }

    // check if vec_coords.len() is 2 or 4. If not, return error (incorrect size)
    match vec_coords.len() {
        2 | 4 => (),
        _ => {
            return Err(Error::InputSize);
        }
    }

    //InputReturn::VecCoords(vec![1])
    Ok(OkInput::Norm)
}

#[derive(Debug, PartialEq)]
enum Error {
    NoInput,
    InputSize,
    InputContainsChar,
    InvalidNumber,
    InvalidCoordinates,
}

fn error_code(e: Error) {
    let error_to_print = match e {
        Error::NoInput => "|=> Error: No input provided".to_string(),
        Error::InputSize => "|=> Error: Too many or too little arguments".to_string(),
        Error::InputContainsChar => "|=> Error: Input contains non-numbers".to_string(),
        Error::InvalidNumber => "|=> Error: Invalid numbers".to_string(),
        Error::InvalidCoordinates => "|=> Error: Invalid Coordinates".to_string(),
    };

    for _ in 0..=error_to_print.len() {
        printf!("-");
    }
    printf!("\n{}\n", error_to_print);
    for _ in 0..=error_to_print.len() {
        printf!("-");
    }
}

fn logic_check(stats: &HashMap<(u8, u8), Tile>, coords: &[u8], turn: &PlayerTurn) -> bool {
    let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);

    // checks if input contains invalid coords "0" or "9"
    if coords.contains(&0) || coords.contains(&9) {
        return false;
    }

    // checks if the beginning tile contains the current player's piece
    match *turn {
        PlayerTurn::Player1 => match *stats.get(&(x1, y1)).unwrap() {
            Tile::P1(_) => (),
            _ => return false,
        },
        PlayerTurn::Player2 => match *stats.get(&(x1, y1)).unwrap() {
            Tile::P2(_) => (),
            _ => return false,
        },
    }

    // checks if the destination tile is empty
    match *stats.get(&(x2, y2)).unwrap() {
        Tile::Emp => (),
        _ => return false,
    }

    // checks if the movement in the y-direction is appropriate and if
    // move 2 spaces, set "need_to_check_for_capture" true.
    let mut need_to_check_for_capture = false;
    match *turn {
        PlayerTurn::Player1 => match stats.get(&(x1, y1)).unwrap() {
            Tile::P1(Stack::Single) => match signed(y2) - signed(y1) {
                1 => (),
                2 => need_to_check_for_capture = true,
                _ => return false,
            },
            Tile::P1(Stack::Double) => match signed(y2) - signed(y1) {
                1 | -1 => (),
                2 | -2 => need_to_check_for_capture = true,
                _ => return false,
            },
            _ => panic!("Error: this should not happen"),
        },
        PlayerTurn::Player2 => match stats.get(&(x1, y1)).unwrap() {
            Tile::P2(Stack::Single) => match signed(y2) - signed(y1) {
                -1 => (),
                -2 => need_to_check_for_capture = true,
                _ => return false,
            },
            Tile::P2(Stack::Double) => match signed(y2) - signed(y1) {
                1 | -1 => (),
                2 | -2 => need_to_check_for_capture = true,
                _ => return false,
            },
            _ => panic!("Error: this should not happen"),
        },
    }

    // check if the movement in the x-direction is appropriate relative
    // to the y.
    match signed(y2) - signed(y1) {
        1 | -1 => match signed(x2) - signed(x1) {
            1 | -1 => (),
            _ => return false,
        },
        2 | -2 => match signed(x2) - signed(x1) {
            2 | -2 => (),
            _ => return false,
        },
        _ => return false,
    }

    /* if "need_to_check_for_capture" is true, then the piece must move two
     * spaces. Because this is only possible if it jumps over a piece, this
     * checks whether there is an enemy piece in between the initial and
     * destination tiles. */
    if need_to_check_for_capture {
        match *stats.get(&((x1 + x2) / 2, (y1 + y2) / 2)).unwrap() {
            Tile::Emp => return false,
            Tile::P1(_) => {
                if *turn == PlayerTurn::Player1 {
                    return false;
                }
            }
            Tile::P2(_) => {
                if *turn == PlayerTurn::Player2 {
                    return false;
                }
            }
        }
    }
    true // the default output if it survives all the checks
}

// if fn logic_check is true, execute/implement the action with logic_move.
fn logic_move(stats: &mut HashMap<(u8, u8), Tile>, coords: &[u8]) -> bool {
    let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);

    // first, create a new piece of the same type as the initial in the
    // destination coordinate.
    Tile::change_tile_state(stats, (x2, y2), &stats.get(&(x1, y1)).unwrap().clone());

    // then, make the initial piece an empty tile.
    Tile::change_tile_state(stats, (x1, y1), &Tile::Emp);

    // if it jumps over a piece, then make it an empty tile.
    match signed(y2) - signed(y1) {
        2 | -2 => {
            Tile::change_tile_state(stats, ((x1 + x2) / 2, (y1 + y2) / 2), &Tile::Emp);
            true // return true if jumps over piece
        }
        // return false if doesn't jump over a piece
        _ => false,
    }
}

fn check_if_promote_to_king(stats: &mut HashMap<(u8, u8), Tile>) {
    // goes through the furthest ends of the board and checks if an enemy piece
    // exists in both respective rows. If yes, promote it to a king.
    for x in 1..=X_LEN {
        if *stats.get(&(x, 8)).unwrap() == Tile::P1(Stack::Single) {
            Tile::change_tile_state(stats, (x, 8), &Tile::P1(Stack::Double));
        }
        if *stats.get(&(x, 1)).unwrap() == Tile::P2(Stack::Single) {
            Tile::change_tile_state(stats, (x, 1), &Tile::P2(Stack::Double));
        }
    }
}

fn check_if_game_over(stats: &HashMap<(u8, u8), Tile>) -> bool {
    // goes through every tile on the board and if there are no P1 or P2 pieces,
    // return true to end the game
    let mut player1s = true;
    let mut player2s = true;
    for y in 1..=Y_LEN {
        for x in 1..=X_LEN {
            match stats.get(&(x, y)).unwrap() {
                Tile::P1(_) => player1s = false,
                Tile::P2(_) => player2s = false,
                Tile::Emp => (),
            }
        }
    }
    // if either remains true, return yes: end game
    player1s || player2s
    // ^^^ minor but neat refactor
}

// prints the UI/board according to HashMap "stats"
fn print_board(stats: &HashMap<(u8, u8), Tile>) {
    // spaghetti-code UI printing algorithm:
    println!("    ,______ ______ ______ ______ ______ ______ ______ ______,");
    for y in (1..=Y_LEN).rev() {
        for s in 0..2 {
            printf!(
                "  {} |",
                if s == 1 {
                    y.to_string()
                } else {
                    String::from(" ")
                }
            );
            for x in 1..=X_LEN {
                for (key, value) in stats {
                    // "*k" or "&(x, y)"???
                    if *key == (x, y) {
                        match value {
                            Tile::P1(Stack::Single) => printf!(" OOOO |"),
                            Tile::P1(Stack::Double) => printf!(" 0KK0 |"),
                            Tile::P2(Stack::Single) => printf!(" //// |"),
                            Tile::P2(Stack::Double) => printf!(" \\KK\\ |"),
                            Tile::Emp => printf!("      |"),
                        }
                    }
                }
            }
            // only here to create newline
            println!();
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
    printf!("\x1B[2J\x1B[1;1H");
}
fn ioflush() {
    let _ = io::stdout().flush().expect("could not flush the thing");
}
fn u32_to_u8(x: u32) -> Result<u8, Error> {
    match x.try_into() {
        Ok(i) => Ok(i),
        Err(_) => Err(Error::InvalidNumber),
    }
}
fn signed(x: u8) -> i8 {
    x.try_into().unwrap()
}
fn user_input() -> String {
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("OwO what's this? Failed to read line");
    ret = ret.trim().parse().unwrap();

    if ret == "exit" {
        clear();
        panic!("exit command used, crashed program");
    }

    ret
}
//
//
//
// extra lines to reach 600 lines