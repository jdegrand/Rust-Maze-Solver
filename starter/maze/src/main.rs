// @author: Joey DeGrand, jpd3263@rit.edu
// Date:    03/23/2021

use std::fmt::Display;
extern crate colored;
use colored::*;

// Should parse arguments and then call the generate_maze / parse_maze function, depending on if you want to
// generate a new maze or read from a file and solve a maze.
fn main() {
    unimplemented!()
}

// _bogus is only there for compliation reasons
// You can choose how to define your struct for maze
struct Maze {
    _bogus: u8
}

// Methods needed for your maze object
impl Maze {
    // Will use the colored library and pretty print the maze with color to the terminal
    // Walls of a maze should have a black background
    // Spaces of a maze should have a white background
    // The soluttion path can be any color
    // Remember that "black background".on_black()" will the text with a black background
    fn color_print_maze(&self) {
        unimplemented!()
    }

    // Solves the maze represented in the maze object.
    // Make sure you save it because we need to print it later
    fn solve(&mut self) -> Option<u8> {
        unimplemented!()
    }
}

// This is how we can call println with a maze object and it will print the string representation of it.
// Check the examples folder for how a proper maze file should look.
impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        unimplemented!()
    }
}


// Generates a valid solvable maze given dimensions.
// Returns it as a maze object.
fn generate_maze(width: usize, height: usize) -> Maze {
    unimplemented!()
}


// Takes in source string from a file, parses it, and returns it as a new Maze object.
// Make sure to check that all rows and columns are the same size and that and that all
// characters are valid
fn parse_maze(src: &str) -> Maze {
    unimplemented!()
}