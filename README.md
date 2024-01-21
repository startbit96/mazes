# mazes
Maze-generation and maze-solving written in Rust and visualized in the terminal.  

## Overview.

### Implemented maze-generation algorithms.
* Kruskal
* Wilson
* Recursive backtracking

### Implemented maze-solving algorithms.
* Breadth-first search (BFS)
* Depth-first search (DFS)
* Wall-follower

## Key bindings.

| Key | Functionality |  
| :---:   | :---: |  
| `r` | re-create the maze |  
| `k` or `↑` | increase size of the maze |  
| `j` or `↓` | decrease size of the maze |  
| `1` - `5` | set the number of mazes |  
| `n` | set the start and end position at random positions |  
| `m` | reset the start and end position to the top left and bottom right corner |  
| `s` | solve the maze |  
| `h` | switch to the next generation algorithm |  
| `l` | switch to the next solving algorithm |  
| `g` | toggle graph visualization on / off |  
| `a` | toggle animation on / off |  
| `b` | toggle binary representation on (with / without background) / off |  
| `q` | quit the application |  

## Notes.

In order to make the visualization of the mazes more appealing, change the size of the cells in your terminal so they are square. In kitty you can change this using the [modify_font setting](https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.modify_font).

For understanding and implementing the algorithms, the Python API [mazelib](https://github.com/john-science/mazelib/tree/main) was very helpful. If you are interested how the algorithms work, see the linked github-repository for the provided documentation.
