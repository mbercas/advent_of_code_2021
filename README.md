# advent_of_code_2021
Advent of code 2021 - learning rust the hard way


## day15

New concepts `BinaryHeap`, `cmp::Reverse`, `ndarray`

In this section I try to find the shortest path. First I will try to implement the Dijkstra algorithm to find the cost of the shortest path.
Dijksrta is an iterative algorithm where all the nodes of a graph are visitied, a temporal cost of the visited paths is stored and in each iteration the visited path with the less cost is retrived.
This means that we need an structure where we can retrive the lowest number easily. I will be using `BinaryHeap` from collections, as this structure sorts the elements from greater to lower.
We need the reverse order so the elements will be introduced in the structure using `cmp::Reverse`.

The grph I am going to transverse can me represented with a matrix where nodes are connected horizontally and vertically, we will use a function to calculate the neighbour nodes and a matrix to store the data.
I had first though of using a HashMap with te (i, j) as index, but this is overkill, instead I am going to use the ndarray crate and `Array2` for representing the 2D aray.
