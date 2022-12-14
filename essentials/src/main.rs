mod structs;
mod enums;

include!("hello_rust.rs");
include!("variables.rs");
include!("structs.rs");
include!("enums.rs");

fn main() {

    // HelloRust - a guessing game.
    guess_game();
    
    // Variables, loops, and functions.
    run_variablaes_demo();

    structs_demo();

    enums_demo();

}