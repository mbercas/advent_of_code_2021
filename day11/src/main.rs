use std::collections::VecDeque;
use std::fs;
use std::io::{self, BufRead};

type Data = Vec<Vec<u8>>;

const MAX_IDX: usize = 9;
const FLASH_VAL: u8 = 10;

fn get_neighbours(
    pos: (usize, usize),
    max_i_index: usize,
    max_j_index: usize,
) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(8);

    let i_min = if pos.0 == 0 { 0 } else { pos.0 - 1 };
    let i_max = if pos.0 == max_i_index {
        pos.0
    } else {
        pos.0 + 1
    };

    let j_min = if pos.1 == 0 { 0 } else { pos.1 - 1 };
    let j_max = if pos.1 == max_j_index {
        pos.1
    } else {
        pos.1 + 1
    };

    for i in i_min..=i_max {
        for j in j_min..=j_max {
            if !((i == pos.0) && (j == pos.1)) {
                neighbours.push((i, j));
            }
        }
    }

    neighbours
}

// Mofifies the input data and returns the number of flashes
fn step_data(data: &mut Data) -> u32 {
    let mut flash_list: VecDeque<(usize, usize)> = VecDeque::new();
    let mut flash_counter: u32 = 0;
    for (i, row) in data.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            *val = *val + 1;
            if *val == FLASH_VAL {
                flash_list.append(
                    &mut get_neighbours((i, j), MAX_IDX, MAX_IDX)
                        .into_iter()
                        .collect(),
                );
            }
        }
    }

    while !flash_list.is_empty() {
        let (i, j) = flash_list.pop_front().unwrap();
        let val = data[i][j] + 1;

        if val == FLASH_VAL {
            flash_list.append(
                &mut get_neighbours((i, j), MAX_IDX, MAX_IDX)
                    .into_iter()
                    .collect(),
            );
        }

        data[i][j] = val;
    }

    for row in data.iter_mut() {
        for val in row.iter_mut() {
            if *val >= FLASH_VAL {
                *val = 0;
                flash_counter += 1;
            }
        }
    }

    flash_counter
}

fn parse_input<I: BufRead>(reader: I) -> Data {
    let mut data: Data = Vec::with_capacity(MAX_IDX + 1);
    for line_ in reader.lines() {
        let mut vec: Vec<u8> = Vec::with_capacity(MAX_IDX + 1);
        let line = line_.unwrap();
        for c in line.trim().chars() {
            vec.push(c.to_digit(10).unwrap() as u8)
        }
        data.push(vec)
    }
    data
}

fn main() {
    let input_file = "input11.txt";
    let f = fs::File::open(input_file).unwrap();
    let reader = io::BufReader::new(f);

    let mut idata = parse_input(reader);
    let data = &mut idata;
    let mut counter = 0;

    let mut nb_flashes = step_data(data);
    for i in 0..100 {
        nb_flashes = step_data(data);
        counter += nb_flashes;
        if nb_flashes == 100 {
            println!("Flashed all in step {}", i+1);
        }
    }
    println!("Number of flashes is {}", counter);
    let mut i = 100;
    while nb_flashes != 100 {
        nb_flashes = step_data(data);
        i += 1;
    }
    println!("Flashed all in step {}", i+1);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
        let f = b"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526" as &[u8];

        let reader = io::BufReader::new(f);
        let mut odata: Data = parse_input(reader);
        let data = &mut odata;
        let mut cnt = 0;
        for _i in 0..10 {
            cnt += step_data(data);
        }
        assert_eq!(204, cnt);

        for _i in 10..100 {
            cnt += step_data(data);
        }
        assert_eq!(1656, cnt);
    }

    #[test]
    fn test_step_data() {
        let sz: usize = MAX_IDX;
        let mut data: Data = Vec::with_capacity(sz);
        for _i in 0..=sz {
            let mut vec = Vec::with_capacity(3);
            for _j in 0..=sz {
                vec.push(0);
            }
            data.push(vec);
        }

        for i in 1..FLASH_VAL {
            let flash_counter = step_data(&mut data);
            assert_eq!(0, flash_counter);
            data.iter()
                .for_each(|row| row.iter().for_each(|val| assert_eq!(i, *val)))
        }

        let flash_counter = step_data(&mut data);
        assert_eq!((MAX_IDX + 1) * (MAX_IDX + 1), flash_counter as usize);
        data.iter()
            .for_each(|row| row.iter().for_each(|val| assert_eq!(0, *val)))
    }

    #[test]
    fn test_get_neighbours() {
        let n = get_neighbours((0, 0), 9, 9);
        let exp = [(0, 1), (1, 0), (1, 1)];
        assert_eq!(3, n.len());
        for (na, ne) in n.iter().zip(exp) {
            assert_eq!(*na, ne);
        }

        let n = get_neighbours((9, 9), 9, 9);
        let exp = [(8, 8), (8, 9), (9, 8)];
        assert_eq!(3, n.len());
        for (na, ne) in n.iter().zip(exp) {
            assert_eq!(*na, ne);
        }

        let n = get_neighbours((1, 1), 9, 9);
        assert_eq!(8, n.len());
        let exp = [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        for (na, ne) in n.iter().zip(exp) {
            assert_eq!(*na, ne);
        }
    }
}
