#![feature(hash_drain_filter)]
use std::io::{self, BufRead};
use std::fs;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Axis {
    HORIZONTAL,
    VERTICAL,
}

struct Instruction {
    line: usize,
    axis: Axis,
}

impl Instruction {
    fn new(line: usize, axis: Axis ) -> Instruction {
        Instruction{line: line, axis: axis}
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    i: usize,
    j: usize,
    val: u32,
}

impl Point {
    fn new(i: usize, j:usize, val: u32) -> Point {
        Point{i: i, j:j, val: val}
    }
}

type Points = HashSet<(usize, usize)>;

fn parse_input<I: BufRead>(reader: I) -> (Points, Vec<Instruction>) {
    let mut points = Points::new();
    let mut instructions: Vec<Instruction> = vec![];

    for line_ in reader.lines() {
        let line = line_.unwrap();
        if line.contains(",") {
            let mut parts =  line.trim().split(',');
            let i = parts.next().unwrap().trim().parse::<usize>().unwrap();
            let j = parts.next().unwrap().trim().parse::<usize>().unwrap();
            points.insert((i, j));
        } else {
            if line.contains("=") {
                let mut parts = line.split('=');
                let axis = if parts.next().unwrap().contains("x") {
                    Axis::VERTICAL
                } else {
                    Axis::HORIZONTAL
                };
                let val = parts.next().unwrap().parse::<usize>().unwrap();
                instructions.push(Instruction::new(val, axis));
            }
        }
    }

    (points, instructions)
}

fn find_max_coordinates(points: &Points) -> (usize, usize) {
    let mut max_i = 0;
    let mut max_j = 0;

    for (i, j) in points {
        if *i > max_i { max_i = *i };
        if *j > max_j { max_j = *j };
    }
    (max_i, max_j)
}

fn fold_up(points: &mut Points, fold_pos: usize)  {

    let drained: Vec<(usize, usize)> = points
        .drain_filter(|(_i, j)| *j > fold_pos)
        .collect();

    for (i, j) in drained {
        points.insert((i, 2*fold_pos - j ));
    }
}

fn fold_left(points: &mut Points, fold_pos: usize)  {
let drained: Vec<(usize, usize)> = points
        .drain_filter(|(i, _j)| *i > fold_pos)
        .collect();

    println!("Size of points {}", points.len());
    for (i, j) in drained {
        points.insert((2*fold_pos - i, j ));
    }
}

fn execute_instruction(points: &mut Points, instruction: &Instruction) {
    if instruction.axis == Axis::HORIZONTAL {
        fold_up(points, instruction.line);
    } else {
        fold_left(points, instruction.line);
    }
}

fn display(points: &mut Points) {
    let (max_i, max_j) = find_max_coordinates(points);
    let mut text: Vec<Vec<char>>= Vec::with_capacity(max_j);
    for _ in 0..=max_j {
        text.push(vec![' ';max_i+1]);
    }
    for (i, j) in points.iter() {
        text[*j][*i] = '#';
    }

    for line in text {
        println!("{}", line.into_iter().collect::<String>())
    }

}

fn main() {
    let input_file ="input13.txt";
    let f = fs::File::open(input_file).unwrap();
    let reader = io::BufReader::new(f);
    let (mut points, folds) = parse_input(reader);

    let mut fold_iter = folds.iter();
    execute_instruction(&mut points, fold_iter.next().unwrap());
    println!("Points after first fold: {}", points.len());


    for fold in fold_iter {
        execute_instruction(&mut points, fold)
    }
    display(&mut points);
}

#[cfg(test)]
mod test {
    use super::*;

    const F: &'static [u8] = b"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5" as &[u8];




    #[test]
    fn test_execute_instruction() {

        let reader = io::BufReader::new(F);
        let (mut points, folds) = parse_input(reader);

        assert_eq!(18, points.len());

        let mut instruction_iter = folds.iter();

        execute_instruction(&mut points, instruction_iter.next().unwrap());
        assert_eq!(17, points.len());

        execute_instruction(&mut points, instruction_iter.next().unwrap());
        assert_eq!(16, points.len());

    }


    #[test]
    fn test_fold_up_left() {

        let reader = io::BufReader::new(F);
        let (mut points, _folds) = parse_input(reader);

        assert_eq!(18, points.len());

        fold_up(&mut points, 7);
        assert_eq!(17, points.len());

        fold_left(&mut points, 5);
        assert_eq!(16, points.len());

    }

    #[test]
    fn test_find_max_coordinates() {
        let reader = io::BufReader::new(F);
        let (points, _folds) = parse_input(reader);

        let (mx, my) = find_max_coordinates(&points);
        assert_eq!(10, mx);
        assert_eq!(14, my);
    }

    #[test]
    fn test_parse_input() {
        let reader = io::BufReader::new(F);
        let (points, folds) = parse_input(reader);

        assert_eq!(18, points.len());
        assert_eq!(2, folds.len());

        let epoints: [(usize, usize); 18] = [(6,10), (0,14), (9,10), (0,3), (10,4), (4,11), (6,0),
                       (6,12), (4,1), (0,13), (10,12), (3,4), (3,0), (8,4), (1,10),
                       (2,14), (8,10), (9,0)];

        for ep in epoints {
            assert_eq!(Some(&ep), points.get(&ep));
        }

        let mut fold_iter = folds.iter();

        let fold = fold_iter.next().unwrap();
        assert_eq!(Axis::HORIZONTAL, fold.axis);
        assert_eq!(7, fold.line);

        let fold = fold_iter.next().unwrap();
        assert_eq!(Axis::VERTICAL, fold.axis);
        assert_eq!(5, fold.line);

    }
}
