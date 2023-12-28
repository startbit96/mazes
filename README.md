# mazes
Maze-generation and maze-solving written in Rust.  

## Overview.

### Implemented maze-generation algorithms.
* Kruskal

### Implemented maze-solving algorithms.

## Key bindings.

| Key | Functionality |  
| :---:   | :---: |  
| `r` | re-create the maze |  
| `k` or `↑` | increase size of the maze |  
| `j` or `↓` | decrease size of the maze |  
| `g` | toggle graph visualization on / off |  
| `a` | toggle animation on / off |  

## Notes.

In order to make the visualization of the mazes more appealing, change the size of the cells in your terminal so they are square. In kitty you can change this using the [modify_font setting](https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.modify_font).

For understanding and implementing the algorithms, the Python API [mazelib](https://github.com/john-science/mazelib/tree/main) was very helpful. If you are interested how the algorithms work, see the linked github-repository for the provided documentation.
