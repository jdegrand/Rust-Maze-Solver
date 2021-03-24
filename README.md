# Maze Generator and Solver!

In this assignment, students will implement a program that can generate and even solve mazes.
Maze files are full of 1's and 0's, where every 0 represents a space and every 1 represents a wall.
Mazes are read from the file and stored inside of a Maze struct, which students define themselves.
The solution to the maze will be printed in color using an external crate, for readibility.
The program will help teach students basic IO, structs, and basic traits in Rust.

Author(s):

- Joey DeGrand

## Assignment Details
Main
---
- The main function where argument parsing should occur.
- Valid arguments are:
  * `generate x y` where x and y are numbers
  * `solve maze_file` where maze_file is a file containing a maze (described further down)
- If the argument requirments aren't met, print a usage message and exit the program
- From main, you will also call functions that generate and solve mazes
- When you call to solve for a maze, if there is no solution it should print no solution. If not, the mazes `color_print_maze` should be called which will pretty print the solution.
---
Maze
---
- There is a struct that is called Maze, in which you can choose to represesnt a maze however you like
- When you parse a maze, it will be turned into a main object
- There are two function that you need to implement for a maze
`color_print_maze`
- This function should be able to pretty print a maze, with colors, for easy readibility. This function will be called to print a solved maze
  * Walls should be black
  * Spaces should be white
  * The solution path can be any color that is not one of the previous two
- There is an external crate that you should use, called `colored`. It is already imported.
- Here is are some examples:
  * `print!("{}", "white background".on_white()); `will print the text with a white background
  * `print!("{}", "black background".on_background());` will print the text with a black background
- Please refer to the documentation for more usage
- It is probably best to just print spaces with a background color for your maze, as if you add letters it wiill be confusing
`solve`
- This function is what solves a maze.
- The only parameter it takes is a mutable self, so you must reference items from your struct in order to solve the maze.
- When you solve the maze, it should be stored somewhere in your struct, as when you call `color_print_maze`, it should print out the maze with the solution.
- This function returns a `Option`; `Some(n)` if it is solvable and (n can be the number of moves if you want to add that as a small challenge), or `None` if a maze is not solvable.
---
Generate
---
- When there program is run with `generate x y`, the program must generate a valid, solvable maze, with dimensions x by y.
- Rules on generating mazes are as follows:
  * Mazes must be square (x and y must be the same; provide a usage message and terminate if they aren't)
  * The maze should be a minimum of 4 by 4
  * The maze must be solvable
  * The start and exit points of the maze should be the top left and bottom right, respectivley. This means they should be 0's
  * Print a usage message if a number can't be parsed for dimensions
- When we say a maze is 5 by 5, we mean that it is 5 by 5 empty spaces. This does not include the walls between. An example of a 5 by 5 maze is as follows:<br><br>
  `000101010`<br>
  `011101010`<br>
  `000100000`<br>
  `011111110`<br>
  `000001010`<br>
  `010111010`<br>
  `010000000`<br>
  `110101011`<br>
  `000101000`<br><br>
- Another example that may be easier to understand is an unsolvable maze, where all possible walls are up<br><br>
  `010101010`<br>
  `111111111`<br>
  `010101010`<br>
  `111111111`<br>
  `010101010`<br>
  `111111111`<br>
  `010101010`<br>
  `111111111`<br>
  `010101010`<br><br>
---
Solve
---

# [Assignment Design](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#assignment_design) Activity

Design a novel programming assignment suitable for the first 1/3 of the course
(Rust Basics).  The assignment should be a bit larger than
[Programming#01](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog01),
but need not be quite as large as
[Programming#02](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog02),
[Programming#03](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog03),
or
[Programming#04](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog04),
although larger assignments are often more interesting.

Individual or Pair assignment.  Each individual student and one member of each
pair should complete the [myCourses
survey](https://mycourses.rit.edu/d2l/lms/survey/user/surveys_list.d2l?ou=888966)
by Tue. 03/09 to indicate how the assignment will be undertaken. A Git
repository on [`https://git.cs.rit.edu/psr2205`](https://git.cs.rit.edu/psr2205)
will be created for each individual/pair for the assignment.

The assignment should have the following structure:

- `README.md` or `README.adoc`: assignment writeup, in either
  [Markdown](https://git.cs.rit.edu/help/user/markdown.html) or
  [AsciiDoc](https://git.cs.rit.edu/help/user/asciidoc.html) format as supported
  by [`git.cs.rit.edu`](https://git.cs.rit.edu) (GitLab)
- `soln`: reference solution
  * `Crate.toml` and `Crate.lock`: `[workspace]` definition, if multi-crate
    assignment; _optional_
  * `assets`: directory of shared assets, if multi-crate assignment; _optional_
  * _`crate1`_: first (and possibly only) assignment crate
    + `Crate.toml` and `Crate.lock`: crate manifest
    + `src`: directory of source files, must include `lib.rs` or `main.rs` and
      may include additional files
    + `assets`: directory of crate-level assets; _optional_
  * _`crate2`_, ..., _`craten`_: second and additional crates, with directory
    structure similar to _`crate1`_; _optional
- `starter`: student starter code; should be a copy of the `soln` directory
  except that any code to be written for the assignment has been removed and
  replaced by `unimplemented!()` or equivalent

# Limitations and Shortcomings
- There seems to be too much specification and rules for parsing a maze. This isn't the focus of the assignment, and is just tedious.
- My solution implemented a very easy maze generation algorithm. My algorithm bascially just creates a minimum spanning tree accross all
  spaces. This not only outputs a weird looking maze, but also only guarentees 1 solution. It would probably be more beneficial for students
  if we told them to implement a more complicated algorithm that they may not have seen before, such as Kruskal's, or other algorithms that
  have more than one path.
