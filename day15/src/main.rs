#![feature(drain_filter)]

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use ndarray::{Array, Array2};


type Map = Array2<u32>;

/// Returns the values of the map and the max values for i,j coordinates
fn parse_input(reader: &str) -> Map {

    let lines: Vec<_> = reader.lines().collect();
    let dim_i = lines.len();
    let dim_j = lines[0].len();
    let map = Array::from_shape_vec(
        [dim_i, dim_j],
        reader.chars().filter_map(|c| c.to_digit(10)).collect(),
    ).unwrap();



    map
}


fn get_neighbours(idx: &[usize; 2], max_i: usize, max_j: usize) -> Vec<([usize; 2])> {

    let mut n: Vec<[usize; 2]> = Vec::with_capacity(4);
    let i = idx[0];
    let j = idx[1];

    if i != 0     { n.push([i-1, j  ]); }
    if i != max_i { n.push([i+1, j  ]); }
    if j != 0     { n.push([i,   j-1]); }
    if j != max_j { n.push([i,   j+1]); }

    n
}

fn get_shortest_path_value(map: &Map) -> u32 {
    //let mut shortest_path = Path::new();

    let mut unvisited_nodes = Array::from_elem(map.shape(), u32::MAX);
    let first_idx = [0, 0];
    let current_node_best_known_cost = unvisited_nodes.get_mut(first_idx).unwrap();
    *current_node_best_known_cost = 0;

    let dim = map.shape();
    let max_i = dim[0] -1;
    let max_j = dim[1] -1;

    let mut visited_nodes: BinaryHeap<(Reverse<u32>, [usize;2])> = BinaryHeap::with_capacity(dim[0]*dim[1]);
    visited_nodes.push((Reverse(0), first_idx));

    while let Some((Reverse(current_node_cost), idx)) = visited_nodes.pop() {

        for nidx in get_neighbours(&idx, max_i, max_j) {

            let new_neighbour_cost = current_node_cost + map[nidx];
            let tmp_neighbour_cost = unvisited_nodes.get_mut(nidx).unwrap();
            if new_neighbour_cost < *tmp_neighbour_cost {
                *tmp_neighbour_cost = new_neighbour_cost;

                visited_nodes.push( (Reverse(new_neighbour_cost),  nidx) );
            }
        }

        // if we have visited the last node we can break
        //if [max_i, max_j] == idx {break;}
    }

    *unvisited_nodes.last().unwrap() //- map.get(&(0, 0)).unwrap()
}


fn main() {

    let reader = include_str!("../input15.txt");
    let map = parse_input(reader);

    let spc = get_shortest_path_value(&map);

    println!("Shortest path cost: {}", spc);
}

#[cfg(test)]
mod test {
    use super::*;

    const F: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";



    #[test]
    fn test_get_shortest_path_value() {
        let map = parse_input(F);

        let sp = get_shortest_path_value(&map);
        assert_eq!(40, sp);
    }

    #[test]
    fn test_get_node_with_less_cost () {

        let max_i = 10_usize;
        let max_j = 10_usize;

        let mut bheap = BinaryHeap::with_capacity((max_i + 1) * (max_j + 1));
        let idx = [0, 0];
        bheap.push((Reverse(5), idx));
        bheap.push((Reverse(8), idx));
        bheap.push((Reverse(2), idx));
        bheap.push((Reverse(9), idx));

        assert_eq!(Some((Reverse(2), idx)), bheap.pop());
        assert_eq!(Some((Reverse(5), idx)), bheap.pop());
        {
            let mut p = match bheap.peek_mut() {
                Some(it) => it,
                _ => unreachable!(),
            };
            *p = (Reverse(15), idx);
        }

        //assert_eq!(8, *modify);
        assert_eq!(Some((Reverse(9), idx)), bheap.pop());
        assert_eq!(Some((Reverse(15), idx)), bheap.pop());
        assert_eq!(None, bheap.pop());



    }

    #[test]
    fn test_get_neighbours() {

        let n = get_neighbours(&[0, 0], 3, 3);
        assert_eq!(2, n.len());
        assert!( n.contains(&[0,1]) );
        assert!( n.contains(&[1,0]) );

        let n = get_neighbours(&[3, 3], 3, 3);
        assert_eq!(2, n.len());
        assert!( n.contains(&[2,3]) );
        assert!( n.contains(&[3,2]) );


        let n = get_neighbours(&[1, 1], 3, 3);
        assert_eq!(4, n.len());
        assert!( n.contains(&[0,1]) );
        assert!( n.contains(&[1,0]) );
        assert!( n.contains(&[1,2]) );
        assert!( n.contains(&[2,1]) );


    }


    #[test]
    fn test_parse_input() {
        //let reader = io::BufReader::new(F);
        //let reader = include_str!(input_file);
        let map = parse_input(F);

        assert_eq!(100, map.len());
    }
}
