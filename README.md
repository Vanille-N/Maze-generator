# Maze Generator

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Output is rendered as ASCII art with Unicode box drawing characters.

Installation:
`$ cargo build`

Usage:

For help: `$ cargo run help`

To build a maze: `$ cargo run ALGORITHM HEIGHT WIDTH`

Algorithms available:
- Prim (`prim`)
- Kruskal (`kruskal`)
- DFS backtracker (`dfs`)
- Ellen (`ellen`)


For more information see [](https://en.wikipedia.org/wiki/Maze_generation_algorithm)

Sample run:

```
$ cargo run -- kruskal 20 30
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/maze kruskal 20 30`
   ↓
  ╷ ╶─────────┬─────────┬───────┬─────┬───┬───┬─┬───┬───┬─┬─┬─┐
  ├───┐ ╶───┐ │ ╶─┬─────┘ ┌─╴ ╶─┘ ╷ ┌─┘ ┌─┴─╴ ╵ ├─┐ ╵ ┌─┤ ╵ │ │
  ├─┐ └─┬───┘ ╵ ┌─┼─╴ ╷ ╶─┼─┬─┐ ╶─┤ ╵ ╶─┤ ╶─┬───┘ │ ╷ ╵ ╵ ╷ │ │
  │ ╵ ╶─┴───┐ ╶─┘ ╵ ╶─┤ ╷ │ ╵ ├─╴ │ ┌───┴─╴ │ ╶─┬─┘ └─┐ ┌─┤ │ │
  │ ┌─────╴ └─┐ ╷ ╷ ╷ │ ├─┤ ╷ │ ┌─┘ └─╴ ╶───┘ ╶─┤ ╷ ╶─┤ ╵ └─┘ │
  ├─┘ ╷ ╶─────┤ │ │ └─┴─┤ ╵ ├─┼─┘ ┌───╴ ┌─┐ ╷ ╷ └─┼───┘ ┌─────┤
  ├─╴ │ ┌─╴ ┌─┘ │ ├─────┤ ╷ │ └───┘ ┌─┐ │ │ ├─┴─┐ ├─┬─╴ ├─┬─┐ │
  │ ╷ ├─┤ ╷ ╵ ┌─┼─┘ ╷ ╷ ╵ ├─┘ ╷ ┌─╴ ╵ └─┘ └─┼─┐ ╵ │ │ ╶─┘ ╵ ╵ │
  │ ├─┘ ├─┴─┐ ╵ ╵ ╷ │ │ ╶─┴───┤ └─┬───╴ ┌───┘ ├─┐ │ ╵ ┌─┬─╴ ╶─┤
  ├─┼─╴ ├─╴ └─╴ ╷ ├─┤ └─┐ ┌───┘ ╶─┼─╴ ┌─┤ ╶───┘ │ └───┘ ├─╴ ╶─┤
  │ ╵ ╷ ├─┬─────┤ ╵ │ ╶─┴─┤ ╷ ┌─╴ └───┘ ╵ ╶─┐ ┌─┘ ┌─╴ ╶─┘ ╷ ┌─┤
  ├───┘ │ ╵ ╷ ╶─┤ ╷ │ ╷ ┌─┴─┼─┘ ╷ ╶─┬───┬─╴ │ ├───┼─┬─────┘ ╵ │
  ├───╴ ╵ ╶─┤ ╷ ├─┴─┼─┴─┤ ╶─┴───┼─┬─┴─╴ ╵ ╷ └─┘ ╶─┤ ├───╴ ┌─╴ │
  ├───┐ ╶─┬─┴─┤ ╵ ┌─┘ ╷ ╵ ┌─╴ ╶─┘ └─╴ ╷ ╶─┴─┬─╴ ┌─┤ └─╴ ┌─┴───┤
  │ ╷ ╵ ┌─┴─┐ ╵ ┌─┘ ┌─┴─╴ ├───╴ ╷ ┌─┐ │ ┌───┼─╴ ╵ └───┐ ├─┬─┐ │
  ├─┴─╴ ╵ ╶─┤ ╷ ╵ ╶─┴─┬─┐ └─┬─┐ └─┤ ├─┴─┘ ┌─┼─┬───┐ ╷ └─┤ │ ╵ │
  │ ╷ ┌───┐ ├─┘ ╷ ╶───┘ └───┤ │ ╷ │ ├─╴ ╶─┤ ╵ │ ╷ ╵ └───┤ ╵ ╷ │
  ├─┴─┘ ╶─┤ ├─┬─┤ ╶─┐ ╶─┬───┤ ╵ └─┤ ├─╴ ╶─┤ ╷ ╵ ├───╴ ┌─┘ ╷ └─┤
  │ ╷ ┌─╴ │ ╵ │ ╵ ┌─┼─┐ └─┐ └─╴ ╶─┘ │ ┌─┐ │ ├─╴ │ ┌─╴ ╵ ╷ │ ┌─┤
  │ ├─┘ ╷ └─╴ │ ╷ ╵ ╵ ├─╴ │ ╶─┬─╴ ╶─┘ │ ╵ └─┤ ╷ ├─┴─╴ ╷ └─┤ ╵ │
  └─┴───┴─────┴─┴─────┴───┴───┴───────┴─────┴─┴─┴─────┴───┴─╴ ╵
                                                             ↓

$ cargo run -- dfs 20 30
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
      Running `target/debug/maze dfs 20 30`
   ↓
  ╷ ╶───────┬───┬─────────────┬─────────────┬─────┬───────┬───┐
  ├─┬─────┐ ╵ ╷ │ ┌─────────┐ ╵ ╷ ┌───────╴ │ ┌─╴ │ ╷ ╶─┐ ╵ ╷ │
  │ ╵ ┌─┐ └───┤ │ └─┐ ┌───┐ ├───┘ │ ╶───┬───┤ └───┤ └─┐ └─┬─┘ │
  │ ┌─┘ │ ┌───┤ └─┐ └─┤ ╷ ╵ │ ╶───┼───┐ │ ┌─┘ ┌─┐ └─┐ ├─╴ │ ┌─┤
  │ ├─╴ │ ╵ ╷ └─┐ └─┐ ╵ │ ┌─┴───╴ │ ╷ │ │ ╵ ╶─┤ └─┐ ╵ │ ┌─┘ ╵ │
  │ ╵ ╷ ├───┴─┐ └─╴ ├───┤ │ ╶───┬─┘ │ │ └─┬─┐ ╵ ╷ ├───┤ └─────┤
  ├───┤ ╵ ┌─┐ ├─────┘ ╶─┤ └─┬─┐ ╵ ╶─┤ ├─┐ │ └─╴ ├─┘ ╷ │ ┌───┐ │
  │ ╷ └───┘ │ │ ╶─┬───┐ └─┐ ╵ ├─────┘ │ │ └─────┘ ┌─┘ │ ╵ ╷ │ │
  │ └─┬───╴ │ │ ┌─┘ ╷ ╵ ╷ └─┐ │ ╶─┬───┘ └─────────┤ ╶─┴─┬─┴─┤ │
  ├─╴ │ ╷ ┌─┘ │ │ ╶─┴───┼─╴ │ ├─┐ │ ┌───────────┐ └───┐ │ ╷ │ │
  ├───┤ │ │ ╶─┤ ├─────┐ └───┘ ╵ │ │ ├─────┐ ┌───┼───╴ │ │ │ ╵ │
  │ ╷ ╵ │ └─┐ └─┤ ┌─┐ └─┐ ╶─┬───┤ │ ╵ ┌─┐ │ │ ╷ ╵ ┌───┘ │ └─┐ │
  │ └─┐ ├───┴─┐ ╵ │ └─┐ │ ┌─┘ ╷ ╵ │ ╶─┤ │ │ ╵ └─┬─┘ ┌───┤ ╷ └─┤
  │ ╷ └─┘ ┌─╴ └─┬─┘ ╷ │ └─┘ ┌─┴───┼─╴ ╵ │ │ ┌───┘ ┌─┤ ╶─┘ ├─┐ │
  │ └───┬─┘ ┌─┐ │ ╶─┴─┼─────┘ ┌───┤ ╶─┬─┘ ├─┘ ┌───┘ ╵ ╶─┬─┘ │ │
  ├───┐ │ ┌─┤ ╵ └───╴ │ ┌───╴ │ ╷ └─╴ │ ┌─┘ ┌─┴─┬─────┐ ╵ ┌─┘ │
  │ ╷ └─┘ │ ╵ ┌───────┘ │ ╶─┐ │ └───┬─┘ │ ╶─┘ ╷ │ ┌─╴ └───┘ ╷ │
  │ └─┬───┘ ┌─┘ ┌───────┴─┐ └─┴───┐ │ ╶─┴─┬───┘ │ └─┬─┬─────┘ │
  │ ╷ └─┐ ╶─┤ ┌─┘ ┌─╴ ┌─┐ ├───┬─╴ │ └─┬─╴ │ ┌───┼─╴ │ │ ┌───┬─┤
  │ └─┐ └───┘ └───┘ ┌─┘ ╵ ╵ ╷ │ ╶─┴─╴ │ ╶─┘ ╵ ╷ ╵ ┌─┘ ╵ ╵ ╷ ╵ │
  └───┴─────────────┴───────┴─┴───────┴───────┴───┴───────┴─╴ ╵
                                                             ↓

$ cargo run -- prim 20 30
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/maze prim 20 30`
   ↓
  ╷ ╶───┬─┬─────┬─────┬─┬─┬─────┬───────────┬───┬───────┬─┬─┬─┐
  ├─╴ ╷ ╵ └─┬─╴ └─╴ ╷ ╵ ╵ ├─╴ ╷ └─╴ ╶─┬─┐ ┌─┴─╴ ╵ ┌─────┘ ╵ ╵ │
  │ ┌─┘ ╷ ┌─┴─╴ ┌─┐ ├─┐ ╷ ├───┤ ┌─╴ ┌─┤ └─┼───╴ ┌─┘ ┌───┬─┬───┤
  │ └─┐ ├─┘ ╷ ┌─┘ │ ╵ │ └─┘ ┌─┴─┼─╴ │ └─╴ └─┐ ╷ └─┐ │ ╷ │ ╵ ╷ │
  │ ┌─┤ │ ╷ └─┼─╴ ├───┼───┬─┘ ╷ │ ╷ │ ┌─╴ ╷ └─┼─╴ ╵ └─┤ ├─┬─┘ │
  ├─┘ ╵ │ └─┬─┘ ╷ ├─╴ ├─╴ ╵ ╶─┴─┘ └─┼─┼─┐ └───┘ ┌───╴ │ │ ╵ ╶─┤
  ├─╴ ╶─┼─╴ ├─┐ │ └─╴ ├─╴ ╶─┬─╴ ╶───┤ ╵ ╵ ┌─╴ ╶─┼─╴ ┌─┘ ╵ ╷ ╶─┤
  ├───╴ ├─╴ │ ╵ ├─┬───┤ ╷ ┌─┼─╴ ╷ ╷ └───┬─┤ ╷ ┌─┘ ╷ ├─╴ ╶─┴─┐ │
  │ ╷ ╷ ╵ ╷ ├─┐ ╵ ├─╴ ├─┴─┘ │ ╷ │ │ ╷ ┌─┘ ╵ └─┤ ╶─┤ ╵ ╶─────┼─┤
  ├─┤ ├───┘ ╵ ╵ ╷ │ ╷ └───┐ └─┘ │ │ └─┘ ┌─┐ ┌─┤ ╷ ├───────┐ ╵ │
  │ └─┴─┐ ╷ ╷ ╷ └─┤ └─┬─┐ ╵ ╶─┐ ├─┴─┐ ┌─┤ └─┤ ├─┴─┼───┐ ╶─┴─╴ │
  │ ╶─┐ │ └─┼─┼─┐ └─╴ ╵ └─┬─┐ └─┘ ╷ │ │ │ ╷ ╵ ╵ ╷ └─╴ │ ╷ ╶─┬─┤
  │ ╷ └─┴─┬─┘ │ ╵ ╶─┬─────┘ ╵ ┌───┤ │ │ └─┼─╴ ┌─┴─┐ ╶─┘ └─┬─┤ │
  ├─┤ ╷ ╷ ╵ ╶─┴─┐ ╷ ├─╴ ╶─┐ ╶─┴─┐ ├─┼─┤ ╶─┴─╴ ├─╴ ├─┬───┐ ╵ ╵ │
  │ ├─┴─┤ ╶───┬─┴─┼─┼───╴ └─┐ ┌─┘ │ ╵ ╵ ╶─┬─┐ ╵ ╷ │ └─╴ └─╴ ╶─┤
  │ │ ╶─┴─╴ ╶─┼─╴ ╵ ├─╴ ╶───┼─┘ ╷ ├───╴ ╷ │ ╵ ╷ └─┼─┬───╴ ┌─┐ │
  │ ╵ ╷ ╷ ╶───┴─┐ ╶─┘ ┌─╴ ╶─┴─┐ ├─┴─╴ ┌─┴─┴─┐ └───┘ ├─────┤ ╵ │
  ├─┐ ├─┘ ╷ ┌─╴ ╵ ╷ ╶─┤ ╷ ╷ ╶─┘ └─┬───┴─┐ ┌─┴───┐ ╷ └─┐ ╶─┴─╴ │
  │ ├─┘ ╷ ├─┘ ╶─┬─┴─╴ ├─┤ └───┐ ┌─┤ ╶─┐ ╵ ╵ ╶─┐ ╵ │ ╶─┼─╴ ╶───┤
  │ ╵ ╶─┤ │ ╷ ╷ └─┐ ╷ │ ╵ ┌─╴ └─┤ ╵ ╶─┴───┬─╴ ├───┘ ╷ └─┬─╴ ╶─┤
  └─────┴─┴─┴─┴───┴─┴─┴───┴─────┴─────────┴───┴─────┴───┴───╴ ╵
                                                             ↓

$ cargo run -- ellen 20 30
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/maze ellen 20 30`
   ↓
  ╷ ┌─────┬─┬─┬───┬─┬─┬───────────┬─┬─┬───┬─┬─┬─────────┬─────┐
  │ │ ╶───┘ ╵ ├─┐ │ │ ╵ ┌─┬─╴ ╶───┤ │ │ ╷ ╵ ╵ └─┐ ╷ ┌───┴───┐ │
  │ ├─────╴ ┌─┘ ╵ ╵ └─┬─┤ ╵ ╶─┐ ╶─┤ │ └─┴─┬───┐ ╵ │ │ ┌─╴ ╶─┘ │
  │ │ ╷ ┌─┐ └─┐ ╶─┬─╴ │ ├─┐ ┌─┼───┤ │ ┌─┬─┼─╴ └─╴ └─┴─┤ ╶─┬─┐ │
  │ ╵ └─┤ ├─┬─┴───┘ ┌─┘ │ ╵ │ └─┐ │ │ ╵ │ └─╴ ╶─┬───┬─┤ ╷ │ ╵ │
  ├─╴ ╷ ╵ │ ├─╴ ╷ ╷ │ ┌─┴─┐ │ ╷ │ │ │ ┌─┤ ╷ ╷ ┌─┤ ╶─┤ └─┤ ├───┤
  ├───┘ ╶─┤ └─┬─┼─┤ │ ├─┐ ╵ ├─┤ │ │ │ │ └─┘ ├─┘ ╵ ╶─┤ ╶─┤ │ ╷ │
  ├─┬─┬─╴ ╵ ╷ │ │ │ ╵ │ ├─╴ ╵ │ │ │ ╵ └───┐ └─┐ ┌─┬─┤ ┌─┘ └─┘ │
  │ │ ╵ ┌─╴ └─┘ ╵ ╵ ╶─┤ └─╴ ┌─┘ ╵ ├───╴ ┌─┼─╴ ╵ │ ╵ │ ├───┬─┐ │
  │ └───┤ ╷ ╶─┬─┐ ╷ ╶─┼─╴ ╷ ├───╴ │ ╶─┬─┘ ╵ ╶─┐ │ ┌─┘ │ ╷ │ │ │
  │ ┌───┘ └─┬─┤ ├─┴───┴─┬─┤ └─╴ ╷ ╵ ┌─┤ ┌─┬─╴ └─┤ ├─╴ └─┤ ╵ │ │
  │ └─┬─┐ ╷ ╵ │ ╵ ╶─┐ ╷ │ ╵ ╷ ╷ │ ╶─┤ └─┘ ├─┐ ╷ ╵ └─┬─╴ │ ╷ │ │
  │ ╶─┘ └─┴─┐ ├─╴ ╶─┴─┤ ├───┤ ├─┤ ┌─┼─┐ ╶─┤ └─┼─┐ ╷ └─┐ ╵ │ │ │
  ├───╴ ╷ ╷ │ ╵ ╷ ╶───┤ ├─┐ └─┤ │ │ │ ├─┐ ╵ ╷ ╵ ├─┤ ╷ ╵ ╶─┴─┘ │
  ├─╴ ╶─┤ └─┘ ╶─┤ ┌─┬─┘ ╵ │ ╷ ╵ ╵ ╵ │ ╵ │ ╷ └───┤ │ ├───┬─┬─┬─┤
  ├─┬─┬─┴─┬─┬───┼─┘ │ ┌─┬─┘ └─┬─┐ ┌─┘ ╶─┴─┴─╴ ╶─┤ ╵ ╵ ╷ │ │ │ │
  │ ╵ │ ┌─┤ ├─╴ ╵ ┌─┤ ╵ └───╴ │ ├─┼─────────╴ ┌─┼─┬─┐ │ ╵ ╵ ╵ │
  │ ┌─┤ │ │ │ ╶─┐ ╵ │ ╶───┬───┤ │ ├─╴ ╷ ╶───┬─┤ │ │ └─┴─┬─┐ ┌─┤
  │ │ │ ╵ ╵ │ ╶─┤ ╷ ├───┐ ╵ ┌─┘ │ │ ╷ └─────┘ │ ╵ ├─┬─┐ ╵ │ ╵ │
  │ ╵ └─╴ ╷ └─╴ └─┴─┘ ╷ ╵ ╶─┘ ╶─┘ └─┘ ╷ ╶─────┘ ╶─┘ ╵ └─╴ ╵ ╷ │
  └───────┴───────────┴───────────────┴─────────────────────┘ ╵
                                                             ↓
```

Mazes generated are guaranteed to be perfect (assuming no mistakes were made):

> A so called 'perfect' maze has every path connected to every other path, so there are no unreachable areas. Also, there are no path loops or isolated walls. There is always one unique path between any two points in the maze.
