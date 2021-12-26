use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::VecDeque;

type Map = HashMap<String, Vec<String>>;
type Path = Vec<String>;


fn parse_input<I: BufRead>(reader: I) -> Map {
    let mut map = HashMap::new();

    for line_ in reader.lines() {
        let line = line_.unwrap();
        let mut parts = line.trim().split('-');
        let first = parts.next().unwrap().to_string();
        let second = parts.next().unwrap().to_string();

        match map.get_mut(&first) {
            None => {
                let _ = map.insert(String::from(&first), vec![String::from(&second)]);
            },
            Some(vec) => vec.push(String::from(&second)),
        }

        match map.get_mut(&second) {
            None => {
                let _ = map.insert(String::from(&second), vec![String::from(&first)]);
            },
            Some(vec) => vec.push(String::from(&first)),
        }


    }

    map
}

fn find_all_paths(map: &Map) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();
    let mut open_paths : VecDeque<Path> = VecDeque::new();


    let start: &Vec<String> = map.get(&String::from("start")).unwrap();
    start.iter().for_each(|n| {
        open_paths.push_back(vec!["start".to_string(), n.to_string()]);
    });

    while open_paths.len() > 0 {
        let current_path = open_paths.pop_front().unwrap();
        let last_node = &current_path[current_path.len()-1];

        match map.get(last_node) {
            Some(adjacent_nodes) => {
                for adjacent_node in adjacent_nodes {

                    let is_big_node = adjacent_node.chars().next().unwrap().is_uppercase();
                    if is_big_node || !current_path.contains(adjacent_node) {

                        let mut new_path = current_path.to_vec();
                        new_path.push(adjacent_node.to_string());

                        if "end" == adjacent_node {
                            //println!("Found path: {:?}", new_path);
                            paths.push(new_path);

                        } else {
                            open_paths.push_back(new_path);
                        }
                    }
                }
            },
            None => (),
        };


    }

    paths
}

fn main() {
    let input_file = "input12.txt";
    let f = fs::File::open(input_file).unwrap();
    let reader = io::BufReader::new(f);
    let map = parse_input(reader);
    let paths =  find_all_paths(&map);

    println!("Number of paths: {}", paths.len());
}


#[cfg(test)]
mod test {

    use super::*;

    const F1: &'static [u8] = b"start-A
start-b
A-c
A-b
b-d
A-end
b-end" as &[u8];

    const F2 : &'static [u8] = b"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc" as &[u8];



    #[test]
    fn test_find_all_paths() {
        let reader = io::BufReader::new(F2);
        let map = parse_input(reader);
        let all_paths = find_all_paths(&map);

        assert_eq!(19, all_paths.len());
    }


    #[test]
    fn test_find_all_paths_simple() {
        let reader = io::BufReader::new(F1);
        let map = parse_input(reader);
        let all_paths = find_all_paths(&map);

        assert_eq!(10, all_paths.len());
    }


    #[test]
    fn test_parse_input() {
        let reader = io::BufReader::new(F2);
        let map = parse_input(reader);

        let start = String::from("start");
        assert_eq!(true, map.contains_key(&start));
        let vec = map.get(&start).unwrap();
        assert_eq!(3, vec.len());
    }


    #[test]
    fn test_parse_input_simple() {
        let reader = io::BufReader::new(F1);
        let map = parse_input(reader);

        let start = String::from("start");
        assert_eq!(true, map.contains_key(&start));


        let a = String::from("A");
        assert_eq!(true, map.contains_key(&a));
        let vec =  map.get(&a).unwrap();

        assert_eq!(4, vec.len());
        let connects = ["start", "c", "b", "end"];
        for c in connects {
            assert_eq!( true, vec.contains( &c.to_string() ) );
            }
    }
}
