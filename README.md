# AoC2024-Rust
My solutions to the Advent of Code 2024 puzzles

### Thoughts
The solutions are run all together in the [main](src/main.rs) file,
compiled down to a single optimized executable.
<br>
The answers are obviously specific to the inputs, which is why I made it so that anyone
can build and run the project with their own inputs (although I only tested the program on Linux).

This was my second time participating in the [**Advent of Code**](https://adventofcode.com/2024)
event, and I loved every moment of it.
<br>
Despite not finishing before Christmas again, I was able to solve most of the puzzles completely on my own and within just a couple days. Thanks to some help from **Reddit** [here](https://www.reddit.com/r/adventofcode/), I was even able to keep the cumulative execution time under 40-50ms, which is truly insane to me (Rust is a great language).

The best part was learning a lot of new things about Rust and the topics [Eric](https://github.com/topaz) hides in his wonderful puzzles; focusing on optimized parsing and fast solutions was interesting and a lot of fun.

### Days
Same as [2023](https://github.com/massimopavoni/AoC2023-Haskell), no write-ups, just listing days with concepts reviewed, studied and learned.

1. [Historian Hysteria](src/historian_hysteria.rs) ->
   [*itertools*](https://crates.io/crates/itertools),
   [Traits](https://doc.rust-lang.org/stable/reference/items/traits.html)

2. [Red-Nosed Reports](src/red_nosed_reports.rs) ->
   more *itertools*

3. [Mull It Over](src/mull_it_over.rs) ->
   [*atoi*](https://crates.io/crates/atoi),
   [*regex*](https://crates.io/crates/regex),
   [Pattern matching](https://doc.rust-lang.org/stable/reference/patterns.html)

4. [Ceres Search](src/ceres_search.rs) ->
   [*grid*](https://crates.io/crates/grid),
   [Array type](https://doc.rust-lang.org/stable/reference/types/array.html)
   const generics

5. [Print Queue](src/print_queue.rs) ->
   [*rustc_hash*](https://crates.io/crates/rustc-hash),
   sorting rules

6. [Guard Gallivant](src/guard_gallivant.rs) ->
   [Binary search](https://en.wikipedia.org/wiki/Binary_search),
   binary insert,
   position and direction representation,
   custom impls and traits,
   [Derive](https://doc.rust-lang.org/stable/reference/attributes/derive.html) macro

7. [Bridge Repair](src/bridge_repair.rs) ->
   expressions evaluation,
   recursive backwards solving

8. [Resonant Collinearity](src/resonant_collinearity.rs) ->
   simple lines and segments geometry

9. [Disk Fragmenter](src/disk_fragmenter.rs) ->
   silly [Defragmentation](https://en.wikipedia.org/wiki/Defragmentation),
   simple [Checksum](https://en.wikipedia.org/wiki/Checksum),
   [Triangular numbers](https://en.wikipedia.org/wiki/Triangular_number),
   [BinaryHeap](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html) (min-heap)

10. [Hoof It](src/hoof_it.rs) ->
    [BFS](https://en.wikipedia.org/wiki/Breadth-first_search)

11. [Plutonian Pebbles](src/plutonian_pebbles.rs) ->
    numbers expansion using hash map

12. [Garden Groups](src/garden_groups.rs) ->
    perimeter and sides geometric analysis,
    cost functions

13. [Claw Contraption](src/claw_contraption.rs) ->
    fast numbers parsing,
    [Inverse matrix solution](https://en.wikipedia.org/wiki/System_of_linear_equations#Matrix_solution) for simple systems of linear equations 

14. [Restroom Redoubt](src/restroom_redoubt.rs) ->
    [*num_modular*](https://crates.io/crates/num-modular),
    [Variance](https://en.wikipedia.org/wiki/Variance),
    [Chinese remainder theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem),
    [Modular multiplicative inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)

15. [Warehouse Woes](src/warehouse_woes.rs) ->
    grid movement simulation

16. [Reindeer Maze](src/reindeer_maze.rs) ->
    [*pathfinding*](https://crates.io/crates/pathfinding),
    [A*](https://en.wikipedia.org/wiki/A*_search_algorithm),
    boxed dyn traits

17. [Chronospatial Computer](src/chronospatial_computer.rs) ->
    program simulation,
    [Bitwise operations](https://en.wikipedia.org/wiki/Bitwise_operation),
    [Octal](https://en.wikipedia.org/wiki/Octal),
    [DFS](https://en.wikipedia.org/wiki/Depth-first_search),
    [Quines](https://en.wikipedia.org/wiki/Quine_(computing))

18. [RAM Run](src/ram_run.rs) ->
    more *pathfinding*,
    path obstacle cutoff

19. [Linen Layout](src/linen_layout.rs) ->
    paths counting,
    custom string pattern matching,
    [*rayon*](https://crates.io/crates/rayon)

20. [Race Condition](src/race_condition.rs) ->
    pathfinding shortcuts cost,
    more *rayon*

21. [Keypad Conundrum](src/keypad_conundrum.rs) ->
    problem depth path cost optimization,
    [Memoization](https://en.wikipedia.org/wiki/Memoization)

22. [Monkey Market](src/monkey_market.rs) ->
    silly [PNRGs](https://en.wikipedia.org/wiki/Pseudorandom_number_generator),
    [SIMD](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data),
    bitwise operations as [Transformation matrices](https://en.wikipedia.org/wiki/Transformation_matrix),
    [*nalgebra*](https://crates.io/crates/nalgebra),
    information compression by bit-packing

23. [LAN Party](src/lan_party.rs) ->
    [Cliques](https://en.wikipedia.org/wiki/Clique_(graph_theory)) and triangles in graphs,
    Adjacency [list](https://en.wikipedia.org/wiki/Adjacency_list) and [matrix](https://en.wikipedia.org/wiki/Adjacency_matrix) representation,
    [Clique problem](https://en.wikipedia.org/wiki/Clique_problem),
    maximal cliques enumeration with the [Bron–Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm),
    greedy approach to [single maximal clique](https://en.wikipedia.org/wiki/Clique_problem#Finding_a_single_maximal_clique)

24. [Crossed Wires](src/crossed_wires.rs) ->
    [Digital electronics](https://en.wikipedia.org/wiki/Digital_electronics),
    combinational circuit simulation,
    heuristic rules for [Ripple carry adder](https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder) errors detection

25. [Code Chronicle](src/code_chronicle.rs) ->
    simple exhaustive enumeration of lock-key pairs

Some additional things I learned during **AoC2024** are: [Cargo](https://doc.rust-lang.org/cargo/) project management, [The Performance Book](https://nnethercote.github.io/perf-book/), Rust program profiling with [flamegraph](https://github.com/flamegraph-rs/flamegraph), [clippy](https://doc.rust-lang.org/clippy/) linting and allow attributes, Rust syntax in general (common structs/enums, iterators, traits, macros, lifetimes), better understanding of ownership and the borrow checker, file handling and resources embedding.
