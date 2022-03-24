use std::collections::HashMap;

fn main() {
    println!("\n\nHewwo~");
    println!("Initiawizing... pls waitt :3");

    println!(
        "\n\n\tcli-checkers UwU\n
            \tPress any key to begin"
    );
    //* wait for key press...

    //* Maybe put everything below in a separate function that sets up the game.

    // The following creates a list of coordinate names and puts them in a vector
    // e.g. 0_0, 3_2, 9_9; where "x-coords_y-coords".
    let mut board_coords: Vec<String> = Vec::new();
    for x in 0..10 {
        for y in 0..10 {
            board_coords.push(format!("{}_{}", x.to_string(), y.to_string()));
        }
    }
    println!("{:#?}", board_coords); //debug

    // The following creates the default board tile state ("emp" as in empty)
    let mut default_states = Vec::new();
    for _ in 0..board_coords.len() {
        default_states.push(String::from("emp"));
    }
    println!("{}", default_states.len()); //debug

    // The following creates a Hash Map from combining the two previous vectors.
    // The hash map stores the following: HashMap<"x_y coordinate", "tile occupancy">
    // This will later be used to keep track of the board and its pieces, as well as
    //   in checking if a given movement option is possible.
    let mut stats: HashMap<_, _> = board_coords.iter().zip(default_states.iter()).collect();
    println!("{:#?}", stats); //debug

    print_board();
}

fn stats_config() {}

fn print_board() {
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
}
