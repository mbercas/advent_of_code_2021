# advent_of_code_2021
Advent of code 2021 - learning rust the hard way


## day15

New concepts `BinaryHeap`, `cmp::Reverse`, `ndarray` crate

In this section I try to find the shortest path. First I will try to implement the Dijkstra algorithm to find the cost of the shortest path.
Dijksrta is an iterative algorithm where all the nodes of a graph are visitied, a temporal cost of the visited paths is stored and in each iteration the visited path with the less cost is retrived.
This means that we need an structure where we can retrive the lowest number easily. I will be using `BinaryHeap` from collections, as this structure sorts the elements from greater to lower.
We need the reverse order so the elements will be introduced in the structure using `cmp::Reverse`.

The graph I am going to transverse can be represented with a matrix where nodes are connected horizontally and vertically, we will use a function to calculate the neighbour nodes and a matrix to store the data.
I had first though of using a HashMap with te (i, j) as index, but this is overkill, instead I am going to use the ndarray crate and `Array2` for representing the 2D aray.

## day16

New concepts `State machine`, `bitstream_io` crate.

In this section I will try to implement a state machine to decode the messages. Since the messages are comming in a byte stream but are composed of fields not necesarily aligned to the byte or the nibble (4 bits) I will use the crate `bitstream_io` to stream bits through a reader.

There is a good explanation on how to implement state machines nicely in Rust. [[https://hoverbear.org/blog/rust-state-machine-pattern/]]
But it is overkill for what we need to do, so finally I resorted to a simpler implementation with two decoding functions and using `enum variants` for the different types of payloads. I have also tried to use `Option` as return types in a more consisting way.

For both the first and second part I have used recursion to iterate over the nested packets.

## day 17

Not new concepts on this problem.

The trick for this problem is to find good boundaries for the initial velocities for trajectories that will fit into the target so we don't restrict the possible values for the iterations.
