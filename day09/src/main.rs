use std::fs;
use std::io::{self, BufRead};
use std::collections::VecDeque;

type Data = Vec<Vec<u8>>;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
    i: usize,
    j: usize,
    val: u8,
}

fn parse_input<I: BufRead>(reader: I) -> Data {
    let mut data: Data = vec![];

    for line_ in reader.lines() {
        let line = line_.unwrap();
        let mut tmp = Vec::<u8>::with_capacity(line.trim().len());
        for c in line.chars() {
            tmp.push(c.to_digit(10).unwrap() as u8);
        }
        data.push(tmp)
    }

    data
}

fn find_low_points(data: &Data) -> Vec<Point> {
    let mut low_points: Vec<Point> = vec![];
    let last_row = data.len() - 1;
    let last_col = data[0].len() - 1;

    for (i, row) in data.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let low_point = if i == 0 {
                if j == 0 {
                    (*val < row[1]) && (*val < data[1][0])
                } else if j == last_col {
                    (*val < row[j - 1]) && (*val < data[1][j])
                } else {
                    (*val < row[j - 1]) && (*val < row[j + 1]) && (*val < data[1][j])
                }
            } else if i == last_row {
                if j == 0 {
                    (*val < row[1]) && (*val < data[i - 1][0])
                } else if j == last_col {
                    (*val < row[j - 1]) && (*val < data[last_row - 1][j])
                } else {
                    (*val < row[j - 1]) && (*val < row[j + 1]) && (*val < data[last_row - 1][j])
                }
            } else if j == 0 {
                (*val < row[j + 1]) && (*val < data[i - 1][j]) && (*val < data[i + 1][j])
            } else if j == last_col {
                (*val < row[j - 1]) && (*val < data[i - 1][j]) && (*val < data[i + 1][j])
            } else {
                (*val < row[j - 1])
                    && (*val < row[j + 1])
                    && (*val < data[i - 1][j])
                    && (*val < data[i + 1][j])
            };
            if low_point {
                low_points.push(Point {
                    i: i,
                    j: j,
                    val: *val,
                });
            }
        }
    }
    low_points
}

fn sum_risk_level(data: &Data, low_points: &Vec<Point>) -> usize {
    let mut risk_level = 0;
    low_points.into_iter().for_each(|p| {
        risk_level += 1 + p.val as usize;
    });
    risk_level
}

fn find_neighbours(data: &Data, point: Point, max_i: usize, max_j: usize) -> Vec<Point> {
    let mut neighbours = vec![];

    let upper_bound: usize = if point.i == 0 { 0 } else { point.i - 1 };
    let lower_bound = if point.i == max_i { max_i } else { point.i + 1 };
    let left_bound: usize = if point.j == 0 { 0 } else { point.j - 1 };
    let right_bound: usize = if point.j == max_j { max_j } else { point.j + 1 };

    for i in upper_bound..=lower_bound {
        if i != point.i {
            neighbours.push(Point{i: i, j: point.j, val: data[i][point.j]});
        }
    }
    for j in left_bound..=right_bound {
        if j != point.j {
            neighbours.push(Point{i: point.i, j: j, val: data[point.i][j]});
        }
    }
    neighbours
}

fn find_basin(data: &Data, point: Point) -> Vec<Point> {
    let mut points = vec![];
    let mut unvisited: VecDeque<Point> = VecDeque::new();
    unvisited.push_back(point);


    while unvisited.len() >  0 {
        let p = unvisited.pop_front().unwrap();
        if !points.contains(&p) {
            points.push(p);

            for n in find_neighbours(data, p, data.len()-1, data[0].len()-1) {
                if n.val != 9 { //&& (n.val > p.val) {
                    unvisited.push_back(n)
                }
            }
        }
    }
    points
}

fn main() {
    let file_name = "input09.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);

    let data = parse_input(reader);
    let low_points = find_low_points(&data);

    println!(
        "Sum of low level points is {} - expected 439",
        sum_risk_level(&data, &low_points)
    );

    let mut basin_vec = vec![];
    for p in low_points {
        basin_vec.push(find_basin(&data, p))
    }
    let mut i = 0;
    let mut prod = 1;
    basin_vec.sort_by(|a, b| a.len().cmp(&b.len()) );
    for v in basin_vec.iter().rev() {
        prod *= v.len();
        i = i +1;
        if i == 3 {break;}
    }
    println!("product of len of 3 biggest basins {}", prod);
}

#[cfg(test)]
mod test {
    use super::*;

    const F: &'static [u8] =
        b"2199943210\n3987894921\n9856789892\n8767896789\n9899965678" as &[u8];


    #[test]
    fn test_find_basin() {
        let reader = io::BufReader::new(F);
        let data = parse_input(reader);

        let basin1 = find_basin(&data, Point{i: 0, j: 0, val: 2});
        assert_eq!(3, basin1.len());

        let basin2 = find_basin(&data, Point{i: 0, j: 9, val: 0});
        assert_eq!(9, basin2.len());

        let basin3 = find_basin(&data, Point{i: 2, j: 2, val: 5});
        assert_eq!(14, basin3.len());

        let basin4 = find_basin(&data, Point{i: 4, j: 6, val: 5});
        assert_eq!(9, basin4.len());

        let mut vec = vec![basin1, basin2, basin3, basin4];
        vec.sort_by(|a, b| a.len().cmp(&b.len()) );

        let mut i = 0;
        let mut prod = 1;
        for v in vec.iter().rev() {
            prod *= v.len();
            i += 1;
            if i == 3 { break; }
        }
        assert_eq!(1134, prod);

    }

    #[test]
    fn test_find_neighbours() {
        let reader = io::BufReader::new(F);
        let data = parse_input(reader);

        let neighbours = find_neighbours(&data,
                                         Point{i: 0, j: 0, val: 2},
                                         data.len()-1,
                                         data[0].len()-1);
        assert_eq!(2, neighbours.len());
        assert_eq!((1, 0, 3), (neighbours[0].i, neighbours[0].j, neighbours[0].val));
        assert_eq!((0, 1, 1), (neighbours[1].i, neighbours[1].j, neighbours[1].val));

        let neighbours = find_neighbours(&data,
                                         Point{i: 0, j: 9, val: 0},
                                         data.len()-1,
                                         data[0].len()-1);

        assert_eq!(2, neighbours.len());
        assert_eq!((1, 9, 1), (neighbours[0].i, neighbours[0].j, neighbours[0].val));
        assert_eq!((0, 8, 1), (neighbours[1].i, neighbours[1].j, neighbours[1].val));


        let neighbours = find_neighbours(&data,
                                         Point{i: 1, j: 0, val: 3},
                                         data.len()-1,
                                         data[0].len()-1);

        assert_eq!(3, neighbours.len());
        assert_eq!((0, 0, 2), (neighbours[0].i, neighbours[0].j, neighbours[0].val));
        assert_eq!((2, 0, 9), (neighbours[1].i, neighbours[1].j, neighbours[1].val));
        assert_eq!((1, 1, 9), (neighbours[2].i, neighbours[2].j, neighbours[2].val));


        let neighbours = find_neighbours(&data,
                                         Point{i: 1, j: 9, val: 1},
                                         data.len()-1,
                                         data[0].len()-1);

        assert_eq!(3, neighbours.len());
        assert_eq!((0, 9, 0), (neighbours[0].i, neighbours[0].j, neighbours[0].val));
        assert_eq!((2, 9, 2), (neighbours[1].i, neighbours[1].j, neighbours[1].val));
        assert_eq!((1, 8, 2), (neighbours[2].i, neighbours[2].j, neighbours[2].val));



        let neighbours = find_neighbours(&data,
                                         Point{i: 1, j: 1, val: 9},
                                         data.len()-1,
                                         data[0].len()-1);

        assert_eq!(4, neighbours.len());
        assert_eq!((0, 1, 1), (neighbours[0].i, neighbours[0].j, neighbours[0].val));
        assert_eq!((2, 1, 8), (neighbours[1].i, neighbours[1].j, neighbours[1].val));
        assert_eq!((1, 0, 3), (neighbours[2].i, neighbours[2].j, neighbours[2].val));
        assert_eq!((1, 2, 8), (neighbours[3].i, neighbours[3].j, neighbours[3].val));


        let neighbours = find_neighbours(&data,
                                         Point{i: 4, j: 0, val: 9},
                                         data.len()-1,
                                         data[0].len()-1);

        assert_eq!(2, neighbours.len());
        assert_eq!((3, 0, 8), (neighbours[0].i, neighbours[0].j, neighbours[0].val));
        assert_eq!((4, 1, 8), (neighbours[1].i, neighbours[1].j, neighbours[1].val));


        let neighbours = find_neighbours(&data,
                                         Point{i: 4, j: 9, val: 9},
                                         data.len()-1,
                                         data[0].len()-1);

        assert_eq!(2, neighbours.len());
        assert_eq!((3, 9, 9), (neighbours[0].i, neighbours[0].j, neighbours[0].val));
        assert_eq!((4, 8, 7), (neighbours[1].i, neighbours[1].j, neighbours[1].val));
    }

    #[test]
    fn test_sum_low_points() {
        let reader = io::BufReader::new(F);
        let data = parse_input(reader);
        let low_points = find_low_points(&data);

        assert_eq!(15, sum_risk_level(&data, &low_points));
    }

    #[test]
    fn test_find_low_points() {
        let reader = io::BufReader::new(F);
        let data = parse_input(reader);
        let low_points = find_low_points(&data);

        assert_eq!(4, low_points.len());
        assert_eq!(
            (0, 1, 1),
            (low_points[0].i, low_points[0].j, low_points[0].val)
        );
        assert_eq!(
            (0, 9, 0),
            (low_points[1].i, low_points[1].j, low_points[1].val)
        );
        assert_eq!(
            (2, 2, 5),
            (low_points[2].i, low_points[2].j, low_points[2].val)
        );
        assert_eq!(
            (4, 6, 5),
            (low_points[3].i, low_points[3].j, low_points[3].val)
        );
    }

    #[test]
    fn test_parse_input() {
        let reader = io::BufReader::new(F);
        let data = parse_input(reader);

        assert_eq!(5, data.len());
        assert_eq!(10, data[0].len());
    }
}
