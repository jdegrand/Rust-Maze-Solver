// @author: Joey DeGrand, jpd3263@rit.edu
// Date:    03/23/2021

use std::fmt::Write;
use std::fs;
use std::env;
use std::process;
use std::fmt::Display;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use rand::Rng;

extern crate colored;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_, action, file] => {
            if action == "solve" {
                let src: String = fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot locate or read file: {}", file));
                let mut maze = parse_maze(&src);
                let res = maze.solve();
                match res {
                    Some(_) => {
                        maze.color_print_maze();
                    }, None => {
                        println!("No result!");
                    }
                }

            } else {
                println!("Invalid arguments");
                process::exit(1);
            }
        },
        [_, action, width, height] => {
            if action == "generate" {
                let w: usize = match width.parse() {
                    Ok(u) => {
                        if u < 4 {
                            println!("Width dimension should be at least 4");
                            process::exit(1);
                        }
                        u
                    },
                    Err(_e) => {
                        println!("Invalid width argument");
                        process::exit(1);
                    }
                };
                let h: usize = match height.parse() {
                    Ok(u) => {
                        if u < 4 {
                            println!("Height dimension should be at least 4");
                            process::exit(1);
                        }
                        u
                    },
                    Err(_e) => {
                        println!("Invalid height argument");
                        process::exit(1);
                    }
                };
                if w != h {
                    println!("Height and width must be the same");
                    process::exit(1);
                }
                let maze = generate_maze(w, h);
                println!("{}", maze);
            } else {
                println!("Invalid arguments");
                process::exit(1);
            }
        },
        _ => {
            println!("Invalid arguments");
            process::exit(1);
        }
    };
}

fn generate_maze(width: usize, height: usize) -> Maze {
    let mut rng = rand::thread_rng();
    let mut blank: Vec<char> = vec!['0'; (width * 2 - 1) * (height * 2 - 1)];

    for r in 0..height * 2 - 1 {
        for c in 0..width * 2 - 1 {
            if c % 2 != 0 || r % 2 != 0 {
                blank[(width * 2 - 1) * r + c] = '1'; 
            }
        }
    }

    let mut stk: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let rr: usize = rng.gen_range(0..height);
    let rc: usize = rng.gen_range(0..width);

    visited.insert((rr, rc));
    stk.push((rr, rc));

    while stk.len() != 0 {
        let curr = stk.pop().unwrap();
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if curr.0 > 0 && !visited.contains(&(curr.0 - 1, curr.1)) {
            neighbors.push((curr.0 - 1, curr.1))
        }
        if curr.0 < height - 1 && !visited.contains(&(curr.0 + 1, curr.1)) {
            neighbors.push((curr.0 + 1, curr.1))
        }
        if curr.1 > 0 && !visited.contains(&(curr.0, curr.1 - 1)) {
            neighbors.push((curr.0, curr.1 - 1))
        }
        if curr.1 < width - 1 && !visited.contains(&(curr.0, curr.1 + 1)) {
            neighbors.push((curr.0, curr.1 + 1))
        }

        if neighbors.len() != 0 {
            let rand_neighbor = rng.gen_range(0..neighbors.len());
            for i in 0..neighbors.len() {
                let chosen_neighbor = neighbors.get(i).unwrap();
                visited.insert(*chosen_neighbor);
                if chosen_neighbor.0 < curr.0 {
                    blank[(width * 2 - 1) * (chosen_neighbor.0 * 2 + 1) + (chosen_neighbor.1 * 2)] = '0';
                } else if chosen_neighbor.0 > curr.0 {
                    blank[(width * 2 - 1) * (curr.0 * 2 + 1) + (chosen_neighbor.1 * 2)] = '0';
                } else if chosen_neighbor.1 < curr.1 {
                    blank[(width * 2 - 1) * (chosen_neighbor.0 * 2) + (chosen_neighbor.1 * 2 + 1)] = '0';
                } else if chosen_neighbor.1 > curr.1 {
                    blank[(width * 2 - 1) * (chosen_neighbor.0 * 2) + (curr.1 * 2 + 1)] = '0';
                }
                if i != rand_neighbor {
                    stk.push(*neighbors.get(i).unwrap());
                }
            }
            stk.push(*neighbors.get(rand_neighbor).unwrap());
        }
    }

    Maze { rows: height * 2 - 1, cols: width * 2 - 1, cells: blank }

}

fn parse_maze(src: &str) -> Maze {
    let mut rows = 0;
    let mut cols = 0;
    let mut col_count = 0;

    for c in src.chars() {
        if cols == 0 && c == '\n' {
            cols = col_count;
            col_count = 0;
            rows = rows + 1;
        } else if c == '\n' && col_count != cols {
            println!("All columns must be the same size");
            process::exit(1);
        } else if c == '\n' {
            col_count = 0;
            rows += 1;
        } else if c != '0' && c!= '1' && c != '\n' {
            println!("Invalid character: {}", c);
            process::exit(1);
        } else {
            col_count += 1;
        }
    }

    if rows < 4 || cols < 4 {
        println!("Maze dimensions must be at least 4x4");
        process::exit(1);
    }

    let mut cells = vec!['0'; cols * rows];
    let mut col_index = 0;
    let mut row_index = 0;

    for c in src.chars() {
        if c == '\n' {
            col_index = 0;
            row_index += 1;
        } else {
            cells[cols * row_index + col_index] = c;
            col_index += 1;
        }
    }

    if cells[0] != '0' || cells[cells.len() - 1] != '0' {
        println!("Top left and bottom right should both be '0', as the are entry and exit points");
        process::exit(1);
    }

    Maze { rows, cols, cells }
}

struct Maze {
    rows: usize,
    cols: usize,
    cells: Vec<char>,
}

impl Maze {
    fn color_print_maze(&self) {
        let mut index = 0;
        for c in self.cells.iter() {
            if index == self.rows {
                println!();
                index = 0;
            }
            if *c == '0' {
                print!("{}", "  ".on_white());
            } else if *c == '2' {
                print!("{}", "  ".on_cyan());
            } else {
                print!("{}", "  ".on_black());
            }
            index += 1;
        }
        println!();
    }

    fn solve(&mut self) -> Option<u8> {
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        q.push_back((0, 0));
        visited.insert((0, 0));
        previous.insert((0, 0), (self.cols, self.rows));
        while q.len() > 0 {
            let curr = q.pop_front().unwrap();
            if curr == (self.cols - 1, self.rows - 1) {
                self.cells[self.cols * (self.rows - 1) + (self.cols - 1)] = '2';
                let mut temp = (self.rows - 1, self.cols - 1,);
                let mut back = previous.get(&(self.rows - 1, self.cols - 1)).unwrap();
                while back != &(self.rows, self.cols) {
                    self.cells[self.cols * temp.0 + temp.1] = '2';
                    if back.0 < temp.0 {
                        self.cells[self.cols * (temp.0 - 1) + temp.1] = '2';
                    } else if back.0 > temp.0 {
                        self.cells[self.cols * (back.0 - 1) + temp.1] = '2';
                    } else if back.1 < temp.1 {
                        self.cells[self.cols * temp.0 + (temp.1 - 1)] = '2';
                    } else if back.1 > temp.1 {
                        self.cells[self.cols * temp.0 + (back.1 - 1)] = '2';
                    }
                    temp = *back;
                    back = previous.get(back).unwrap();
                }
                self.cells[self.cols * temp.0 + temp.1] = '2';
                return Some(0);
            }
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            if curr.0 > 1 && !visited.contains(&(curr.0 - 2, curr.1)) && self.cells[self.cols * (curr.0 - 1) + curr.1] != '1' {
                neighbors.push((curr.0 - 2, curr.1));
            }
            if curr.0 < self.rows - 2 && !visited.contains(&(curr.0 + 2, curr.1)) && self.cells[self.cols * (curr.0 + 1) + curr.1] != '1' {
                neighbors.push((curr.0 + 2, curr.1));
            }
            if curr.1 > 1 && !visited.contains(&(curr.0, curr.1 - 2)) && self.cells[self.cols * curr.0 + (curr.1 - 1)] != '1' {
                neighbors.push((curr.0, curr.1 - 2));
            }
            if curr.1 < self.cols - 2 && !visited.contains(&(curr.0, curr.1 + 2)) && self.cells[self.cols * curr.0 + (curr.1 + 1)] != '1' {
                neighbors.push((curr.0, curr.1 + 2));
            }
            for n in neighbors.iter() {
                q.push_back(*n);
                visited.insert(*n);
                previous.insert(*n, curr);
            }
        }
        None
    }
}

impl Display for Maze {
    
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    let mut index = 0;    
    for c in self.cells.iter() {
            if index == self.rows {
                f.write_char('\n')?;
                index = 0;
            }
            f.write_char(*c)?;
            index += 1;
        }
        Ok(())
    }
}