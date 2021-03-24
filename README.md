# Maze Generator and Solver!

In this assignment, students will implement a program that can generate and even solve mazes.
Maze files are full of 1's and 0's, where every 0 represents a space and every 1 represents a wall.
Mazes are read from the file and stored inside of a Maze struct, which students define themselves.
The solution to the maze will be printed in color using an external crate, for readibility.
The entry point of the maze is the top left of the maze file and the exit is the bottom right.
The program will help teach students basic IO, structs, and basic traits in Rust.

This project assignment might not seem big, but it definitley takes more time than Prog 01 and less time than Prog 02 and up, which is perfect for an introductoin assignment. I wanted to implement an easier Cow Snatchers puzzle project, so it has similar ideas but is much less complicated and takes less time.

I chose to have little to no starter code, as for a Rust beginner it is difficult to jump into someone elses code and understand what is going on. That is better for a later assignment. I feel like for the introduction assigments, it is better for students to experiment with Rust on their own and get things wrong and learn.

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

`fmt`
- This method should print a maze object in the format of a maze file, so you can easily call `println` and get the string representation of the maze.
- The maze file is described in the next section.
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
- This means when we generate a maze that is x by y, it is really `(x + (x - 1))*(y + (y - 1))`
---
Parse Maze
---
- `parse_maze` should be called by main. It is given an `&str`, which should be the string that is read from the file
- This function is what turns a String into an initial Maze object, hence the return type is a `Maze` instance
- This is where you should make sure that there are no invalid characters, and that all rows and columns are the same size

# Limitations and Shortcomings
- There seems to be too much specification and rules for parsing a maze. This isn't the focus of the assignment, and is just tedious.
- The whole walls and spaces thing is also a little confusing; I tried my best to explain it above
- My solution implemented a very easy maze generation algorithm. My algorithm bascially just creates a minimum spanning tree accross all
  spaces. This not only outputs a weird looking maze, but also only guarentees 1 solution. It would probably be more beneficial for students
  if we told them to implement a more complicated algorithm that they may not have seen before, such as Kruskal's, or other algorithms that
  have more than one path.
- I don't think my program utilizes Rust in ways it definitley should. For example, there is a lot of copying and cloning going on behind the scenes.
