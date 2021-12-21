use regex::{self, Regex};
use std::cmp::{max, min};
use std::fmt;
use std::fs;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Errors {
    RegexParseError,
}

struct Point {
    x: u32,
    y: u32,
}

struct Line {
    points: Vec<Point>,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:3}, {:3}", self.x, self.y)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tmp_string = String::new();
        for (i, point) in self.points.iter().enumerate() {
            let sep = if i % 5 == 4 {
                String::from("\n")
            } else {
                String::from(" ")
            };
            tmp_string.push_str(format!("{}{}", point, sep.as_str()).as_str());
        }
        write!(f, "{}", tmp_string.to_string())
    }
}

impl Line {
    fn initialize_points_in_line(&mut self, start: &Point, end: &Point) {
        let startx = min(start.x, end.x);
        let starty = min(start.y, end.y);

        let endx = max(start.x, end.x);
        let endy = max(start.y, end.y);

        let number_of_points = max(endx - startx + 1, endy - starty + 1) as usize;

        let x_idx: Vec<u32> = if startx == endx {
            vec![startx; number_of_points]
        } else {
            if start.x < end.x {
                (start.x..=end.x).collect::<Vec<u32>>()
            } else {
                (end.x..=start.x).rev().collect::<Vec<u32>>()
            }
        };

        let y_idx: Vec<u32> = if starty == endy {
            vec![starty; number_of_points]
        } else {
            if start.y < end. y {
                (start.y..=end.y).collect::<Vec<u32>>()
            } else {
                (end.y..=start.y).rev().collect::<Vec<u32>>()
            }
        };

        for (x, y) in x_idx.iter().zip(y_idx.iter()) {
            self.points.push(Point { x: *x, y: *y });
        }
    }
}

fn parse_points_string(pair_str: &str) -> Result<(Point, Point), Errors> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    if re.is_match(pair_str) {
        let caps = re.captures(pair_str).unwrap();
        let x = caps.get(1).map_or("", |x| x.as_str());
        let y = caps.get(2).map_or("", |x| x.as_str());

        let start = Point {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
        };
        let x = caps.get(3).map_or("", |x| x.as_str());
        let y = caps.get(4).map_or("", |x| x.as_str());
        let end = Point {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
        };
        Ok((start, end))
    } else {
        Err(Errors::RegexParseError)
    }
}

/* Return max_x, max_y and a vector with all the lines */
fn parse_file<I: io::BufRead>(reader: I, exclude_diagonals: bool) -> (u32, u32, Vec<Line>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut lines = vec![];

    for line in reader.lines() {
        let mut tmp = Line { points: vec![] };
        let (start, end) = parse_points_string(&line.unwrap()).unwrap();
        if max(start.x, end.x) > max_x {
            max_x = max(start.x, end.x);
        }
        if max(start.y, end.y) > max_y {
            max_y = max(start.y, end.y);
        }

        if exclude_diagonals && (start.x != end.x) && (start.y != end.y) {
            continue;
        }
        tmp.initialize_points_in_line(&start, &end);
        lines.push(tmp);
    }

    return (max_x + 1, max_y + 1, lines);
}

fn calculate_overlaps(max_x: u32, max_y: u32, lines: &Vec<Line>) -> u32 {
    let mut space: Vec<u32> = vec![0u32; (max_x * max_y).try_into().unwrap()];

    let mut count_overlaps = 0;
    for line in lines {
        for point in &line.points {
            let pos: usize = (point.x + max_y * point.y).try_into().unwrap();
            space[pos] += 1;
            if space[pos] == 2 {
                count_overlaps += 1;
            }
        }
    }
    count_overlaps
}

fn main() {
    let file_name = "input05.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);

    /*
    // exclude diagonals
    let (max_x, max_y, lines) = parse_file(reader, true);
    println!("Mat is {} x {}", max_x, max_y);
    println!("Number of lines is {}", lines.len());
    println!(
        "Dangerous poins: {}",
        calculate_overlaps(max_x, max_y, &lines)
    );

    println!("---------");
    */
    // include diagonals
    let (max_x, max_y, lines) = parse_file(reader, false);
    println!("Mat is {} x {}", max_x, max_y);
    println!("Number of lines is {}", lines.len());
    println!(
        "Dangerous poins: {}",
        calculate_overlaps(max_x, max_y, &lines)
    );
}

#[cfg(test)]
mod test {

    use super::*;

    const F: &'static [u8] = b"0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2" as &[u8];

    #[test]
    fn integration_test_solution2() {
        let reader = io::BufReader::new(F);
        let (max_x, max_y, lines) = parse_file(reader, false);
        let overlaps = calculate_overlaps(max_x, max_y, &lines);
        assert_eq!(10, max_x);
        assert_eq!(10, max_y);
        assert_eq!(10, lines.len());

        let mut lines_iterator = lines.iter();

        let line = lines_iterator.next().unwrap();
        let mut i = 0;
        for point in &line.points {
            assert_eq!(point.y, 9);
            assert_eq!(i, point.x);
            i += 1;
        }

        assert_eq!(12, overlaps);
    }

    #[test]
    fn integration_test_solution1() {
        let reader = io::BufReader::new(F);
        let (max_x, max_y, lines) = parse_file(reader, true);
        let overlaps = calculate_overlaps(max_x, max_y, &lines);
        assert_eq!(10, max_x);
        assert_eq!(10, max_y);
        assert_eq!(6, lines.len());

        let mut lines_iterator = lines.iter();

        let line = lines_iterator.next().unwrap();
        let mut i = 0;
        for point in &line.points {
            assert_eq!(point.y, 9);
            assert_eq!(i, point.x);
            i += 1;
        }

        assert_eq!(5, overlaps);
    }

    #[test]
    fn test_initialize_points_in_line() {
        let mut line = Line { points: vec![] };

        line.initialize_points_in_line(&Point { x: 0, y: 0 }, &Point { x: 3, y: 0 });
        assert_eq!(4, line.points.len());
        for (i, point) in line.points.iter().enumerate() {
            assert_eq!(point.x, i as u32);
            assert_eq!(point.y, 0);
        }

        let mut line = Line { points: vec![] };
        line.initialize_points_in_line(&Point { x: 3, y: 0 }, &Point { x: 0, y: 0 });
        assert_eq!(4, line.points.len());
        for (i, point) in line.points.iter().enumerate() {
            assert_eq!(point.x, 3-i as u32);
            assert_eq!(point.y, 0);
        }

        let mut line = Line { points: vec![] };
        line.initialize_points_in_line(&Point { x: 0, y: 3 }, &Point { x: 0, y: 0 });
        assert_eq!(4, line.points.len());
        for (i, point) in line.points.iter().enumerate() {
            assert_eq!(point.x, 0);
            assert_eq!(point.y, 3-i as u32);
        }

        let mut line = Line { points: vec![] };
        line.initialize_points_in_line(&Point { x: 0, y: 0 }, &Point { x: 0, y: 3 });
        assert_eq!(4, line.points.len());
        for (i, point) in line.points.iter().enumerate() {
            assert_eq!(point.x, 0);
            assert_eq!(point.y, i as u32);
        }

        let mut line = Line { points: vec![] };
        line.initialize_points_in_line(&Point { x: 3, y: 3 }, &Point { x: 3, y: 3 });
        assert_eq!(1, line.points.len());
    }

    #[test]
    fn test_parse_points_string() {
        let test_str = "0,9 -> 5,9";

        let (start, end) = parse_points_string(test_str).unwrap();

        assert_eq!(start.x, 0);
        assert_eq!(start.y, 9);
        assert_eq!(end.x, 5);
        assert_eq!(end.y, 9);
    }
}
