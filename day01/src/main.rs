use std::fs;
use std::io::{self, BufRead};

// Read a sequence and count how many times the next number is greater than the previous
//

fn count_increments(collection: &[u32]) -> u32 {
    let mut iter = collection.into_iter();
    if iter.len() < 2 {
        return 0;
    }
    let mut prev_val = match iter.next() {
        Some(it) => it,
        _ => unreachable!(),
    };
    let mut counter = 0;
    for val in iter {
        if val > prev_val {
            counter += 1;
        }
        prev_val = val
    }
    counter
}

fn make_sliding_window(collection: &[u32]) -> Vec<u32> {
    let mut iter = collection.into_iter();
    let mut result: Vec<u32> = Vec::new();

    if iter.len() < 4 {
        return result;
    }

    let mut one = iter.next().unwrap();
    let mut two = iter.next().unwrap();

    for val in iter {
        result.push(one + two + val);
        one = two;
        two = val;
    }

    result
}
/*
 *
 */
fn main() {
    let file_name = "input_01.txt";
    let solution1 = 1228;
    let solution2 = 1257;

    /*
    let f = match fs::File::open(file_name) {
        Err(why) => panic!("Count not open {}: {}", file_name, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(f).lines();
    let mut numbers = vec![];

    for line in lines {
        match line {
            Ok(value) => numbers.push(value.parse::<u32>().unwrap()),
            Err(why) => println!("Error when reading file: {}", why),
        }
    }
     */

    let numbers = io::BufReader::new(fs::File::open(file_name).unwrap())
        .lines()
        .map(|x| x.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let counter = count_increments(numbers.as_slice());
    println!("Found {} increments, expected {}", counter, solution1);

    let window = make_sliding_window(numbers.as_slice());
    let scounter = count_increments(&window);
    println!(
        "Found {} smooth increments, expected {}",
        scounter, solution2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let test_cases = [
            (vec![0, 0, 1, 1, 2, 3, 3, 4, 5], 5, "1"),
            (vec![0, 0, 0, 0, 0, 0, 0, 0, 0], 0, "2"),
            (vec![1, 2, 1, 2, 1, 2, 1, 2], 4, "3"),
        ];

        for test_case in test_cases {
            assert_eq!(
                count_increments(test_case.0.as_slice()),
                test_case.1,
                "test #{:02}",
                test_case.2
            );
        }
    }

    #[test]
    fn test_smooth_count() {
        let test_cases = [
            (
                vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263],
                5,
                "1",
            ),
            (vec![199], 0, "2"),
            (vec![199, 200], 0, "3"),
            (vec![199, 200, 208], 0, "4"),
        ];

        for test_case in test_cases {
            let counter = make_sliding_window(test_case.0.as_slice());
            assert_eq!(
                count_increments(counter.as_slice()),
                test_case.1,
                "test #{:02}",
                test_case.2
            );
        }
    }
}
