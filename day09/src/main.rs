use std::fs;
use std::io::{self, BufRead};

type Data = Vec<Vec<u8>>;

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
                low_points.push(Point{i:i, j:j, val: *val});
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
}

#[cfg(test)]
mod test {
    use super::*;

    const F: &'static [u8] =
        b"2199943210\n3987894921\n9856789892\n8767896789\n9899965678" as &[u8];

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
        assert_eq!((0, 1, 1), (low_points[0].i, low_points[0].j, low_points[0].val));
        assert_eq!((0, 9, 0), (low_points[1].i, low_points[1].j, low_points[1].val));
        assert_eq!((2, 2, 5), (low_points[2].i, low_points[2].j, low_points[2].val));
        assert_eq!((4, 6, 5), (low_points[3].i, low_points[3].j, low_points[3].val));
    }

    #[test]
    fn test_parse_input() {
        let reader = io::BufReader::new(F);
        let data = parse_input(reader);

        assert_eq!(5, data.len());
        assert_eq!(10, data[0].len());
    }
}
