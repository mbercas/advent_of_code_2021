use std::fs;
use std::io::{BufRead, BufReader};

struct BinaryNumbers {
    ones: Vec<u32>,
    zeros: Vec<u32>,
    word_len: usize,
}

impl BinaryNumbers {
    fn new() -> BinaryNumbers {
        let bn = BinaryNumbers {
            ones: vec![],
            zeros: vec![],
            word_len: 0,
        };
        bn
    }

    fn rates(&self) -> (u32, u32) {
        let mut gamma = 0;
        let mut i = 0;
        for (ones, zeros) in self.ones.iter().zip(self.zeros.iter()) {
            gamma += (1 << i) * if ones >= zeros { 1 } else { 0 };
            i += 1;
        }
        let epsilon = (2_u32.pow(self.word_len.try_into().unwrap()) - 1) - gamma;
        (gamma, epsilon)
    }
}

fn parse_strings<I: BufRead>(reader: I) -> (usize, Vec<u32>) {
    let mut number_of_bits = 0;
    let mut vec: Vec<u32> = Vec::new();

    for (cnt, line) in reader.lines().enumerate() {
        match line {
            Ok(lstr) => {
                if cnt == 0 {
                    number_of_bits = lstr.len();
                } else {
                    if number_of_bits != lstr.len() {
                        panic!("words of different length");
                    }
                }
                let num = match isize::from_str_radix(lstr.as_str(), 2) {
                    Ok(num) => num,
                    Err(why) => panic!("Coudn't parse line {}:{} as binary: {}", cnt, lstr, why),
                };
                vec.push(num as u32)
            }
            Err(why) => panic!("Failed reading string in line {}: {}", cnt, why),
        }
    }

    (number_of_bits, vec)
}

fn count_numbers_in_column(number_of_bits: usize, vec: Vec<u32>) -> BinaryNumbers {
    let mut numbers = BinaryNumbers::new();
    numbers.word_len = number_of_bits;
    numbers.ones.resize(number_of_bits, 0);
    numbers.zeros.resize(number_of_bits, 0);
    for val in vec {
        for i in 0..numbers.word_len {
            if (val & (1 << i)) == 0 {
                numbers.zeros[i] += 1;
            } else {
                numbers.ones[i] += 1;
            }
        }
    }

    numbers
}

fn filter_by_count(pos: usize, number_of_bits: usize, vec: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    let mut vec_most_common: Vec<u32> = vec![];
    let mut vec_least_common: Vec<u32> = vec![];

    if pos > number_of_bits - 1 {
        panic!("search position out of bounds");
    }

    let numbers = count_numbers_in_column(number_of_bits, vec.clone());
    let mask = 1 << pos;

    let mut most_common_value = 0;
    let mut least_common_value = 0;
    if numbers.zeros[pos] <= numbers.ones[pos] {
        most_common_value = 1;
    }

    if numbers.zeros[pos] > numbers.ones[pos] {
        least_common_value = 1;
    }

    for val in vec {
        let bit = (val & mask) >> pos;
        if bit == most_common_value {
            vec_most_common.push(val);
        }
        if bit == least_common_value {
            vec_least_common.push(val);
        }
    }
    (vec_most_common, vec_least_common)
}

fn oxygen(number_of_bits: usize, vec: Vec<u32>) -> u32 {
    let mut vec_most_common = vec.clone();
    for i in (0..number_of_bits).rev() {
        let (a, _) = filter_by_count(i, number_of_bits, vec_most_common.clone());
        vec_most_common = a;
        if vec_most_common.len() == 1 {
            //return vec_most_common[0];
            break;
        }
    }
    vec_most_common[0]
}

fn co2(number_of_bits: usize, vec: Vec<u32>) -> u32 {
    let mut vec_least_common = vec.clone();
    for i in (0..number_of_bits).rev() {
        let (_, b) = filter_by_count(i, number_of_bits, vec_least_common.clone());
        vec_least_common = b;
        if vec_least_common.len() == 1 {
            break;
        }
    }
    vec_least_common[0]
}

fn main() {
    let file_name = "input03.txt";
    let f = match fs::File::open(file_name) {
        Ok(f) => f,
        Err(why) => panic!("Failed opening file {}: {}", file_name, why),
    };
    let reader = BufReader::new(f);

    let (number_of_bits, vec) = parse_strings(reader);
    let numbers = count_numbers_in_column(number_of_bits, vec.clone());

    println!(
        "zeros: {:?}\nones: {:?}\nlen: {}",
        numbers.zeros, numbers.ones, numbers.word_len
    );
    let (gamma, epsilon) = numbers.rates();
    println!(
        "gamma: {}, epsilon: {}, gamma*epsilon: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let oxygen_val = oxygen(number_of_bits, vec.clone());
    let co2_val = co2(number_of_bits, vec.clone());

    println!(
        "O2: {}, CO2: {}, product: {}",
        oxygen_val,
        co2_val,
        oxygen_val * co2_val
    );
}

#[cfg(test)]
mod test {
    //use std::io;

    use super::*;

    #[test]
    fn test_binary_numbers_new() {
        let numbers = BinaryNumbers::new();
        assert_eq!(0, numbers.ones.len());
        assert_eq!(0, numbers.zeros.len());
        assert_eq!(0, numbers.word_len);
    }

    #[test]
    fn test_parse_strings() {
        let lstr = b"010101\n101010\n111111" as &[u8];
        let vals = [0b010101, 0b101010, 0b111111];
        let reader = BufReader::new(lstr);
        let lstr_len = 6;

        let (number_of_bits, vec) = parse_strings(reader);
        assert_eq!(lstr_len, number_of_bits);
        assert_eq!(3, vec.len());
        for (val, expected) in vec.iter().zip(vals.iter()) {
            assert_eq!(val, expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_strings_panic_with_different_sizes() {
        let lstr = b"010101\n1010\n111\n000000000" as &[u8];
        let reader = BufReader::new(lstr);

        let (_, _) = parse_strings(reader);
    }

    #[test]
    #[should_panic]
    fn test_parse_strings_panic_not_binary() {
        let lstr = b"010\n101\n131\n000" as &[u8];
        let reader = BufReader::new(lstr);

        let (_, _) = parse_strings(reader);
    }

    #[test]
    fn test_count_numbers() {
        let input = vec![0b0; 10];

        for i in 1..10 {
            let numbers = count_numbers_in_column(i, input.clone());
            assert_eq!(i, numbers.word_len);
            for (ones, zeros) in numbers.ones.iter().zip(numbers.zeros.iter()) {
                assert_eq!(0, *ones);
                assert_eq!(10, *zeros);
            }
        }

        for i in 1usize..10 {
            let input = vec![2_u32.pow(i as u32) - 1; 10];
            let numbers = count_numbers_in_column(i, input.clone());
            assert_eq!(i, numbers.word_len);
            for (ones, zeros) in numbers.ones.iter().zip(numbers.zeros.iter()) {
                assert_eq!(10, *ones);
                assert_eq!(0, *zeros);
            }
        }
    }

    #[test]
    fn test_rates() {
        let mut numbers = BinaryNumbers::new();
        numbers.word_len = 5;
        numbers.ones = vec![0, 1, 1, 0, 1];
        numbers.zeros = vec![1, 0, 0, 1, 0];
        let (gamma, epsilon) = numbers.rates();
        assert_eq!(22, gamma);
        assert_eq!(9, epsilon);
    }

    #[test]
    #[should_panic]
    fn test_filter_by_count_panic_pos_outofbounds() {
        let inputs = vec![0; 10];
        let (_, _) = filter_by_count(5, 3, inputs);
    }

    #[test]
    fn test_filter_by_count() {
        let mut inputs = vec![];
        inputs.push(0b0000);
        inputs.push(0b0001);
        inputs.push(0b0010);
        inputs.push(0b0100);

        let (vec_most_common, vec_least_common) = filter_by_count(0, 4, inputs);
        assert_eq!(3, vec_most_common.len());
        assert_eq!(1, vec_least_common.len());

        assert_eq!(1, vec_least_common[0]);
        assert_eq!(0, vec_most_common[0]);
        assert_eq!(2, vec_most_common[1]);
        assert_eq!(4, vec_most_common[2]);

        let mut inputs = vec![];
        inputs.push(0b1111);
        inputs.push(0b1110);
        inputs.push(0b1101);
        inputs.push(0b1011);

        let (vec_most_common, vec_least_common) = filter_by_count(0, 4, inputs);
        assert_eq!(3, vec_most_common.len());
        assert_eq!(1, vec_least_common.len());

        assert_eq!(0b1110, vec_least_common[0]);

        assert_eq!(0b1111, vec_most_common[0]);
        assert_eq!(0b1101, vec_most_common[1]);
        assert_eq!(0b1011, vec_most_common[2]);

        let mut inputs = vec![];
        inputs.push(0b1111);
        inputs.push(0b1110);
        inputs.push(0b1101);
        inputs.push(0b1011);

        let (vec_most_common, vec_least_common) = filter_by_count(1, 4, inputs);
        assert_eq!(3, vec_most_common.len());
        assert_eq!(1, vec_least_common.len());

        assert_eq!(0b1101, vec_least_common[0]);

        assert_eq!(0b1111, vec_most_common[0]);
        assert_eq!(0b1110, vec_most_common[1]);
        assert_eq!(0b1011, vec_most_common[2]);

        let mut inputs = vec![];
        inputs.push(0b1111);
        inputs.push(0b1110);
        inputs.push(0b1101);
        inputs.push(0b1011);

        let (vec_most_common, vec_least_common) = filter_by_count(3, 4, inputs);
        assert_eq!(4, vec_most_common.len());
        assert_eq!(0, vec_least_common.len());

        assert_eq!(0b1111, vec_most_common[0]);
        assert_eq!(0b1110, vec_most_common[1]);
        assert_eq!(0b1101, vec_most_common[2]);
        assert_eq!(0b1011, vec_most_common[3]);
    }


    #[test]
    fn test_filter_by_count_complex() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let (input, _ ) = filter_by_count(4, 5, input);
        assert_eq!(7, input.len());

        let (input, _ ) = filter_by_count(3, 5, input);
        assert_eq!(4, input.len());

        let (input, _ ) = filter_by_count(2, 5, input);
        assert_eq!(3, input.len());

        let (input, _ ) = filter_by_count(1, 5, input);
        assert_eq!(2, input.len());

        let (input, _ ) = filter_by_count(0, 5, input);
        assert_eq!(1, input.len());
        assert_eq!(23, input[0]);

        let mut input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let remaining = [1, 2, 3, 4, 7];

        for i in (0..=4).rev() {
            let (a, _) = filter_by_count(i, 5, input.clone());
            input = a;
            assert_eq!(remaining[i], input.len());
            if input.len() == 1 {
                break;
            }
        }
        assert_eq!(23, input[0]);

    }


    #[test]
    fn test_oxygen_simple() {
        let input = vec![0b00100, 0b11111, 0b01111];
        let oxygen_val = oxygen(5, input);
        assert_eq!(15, oxygen_val);

        let input = vec![0b00100, 0b11111, 0b00110];
        let oxygen_val = oxygen(5, input);
        assert_eq!(6, oxygen_val);
    }

    #[test]
    fn test_oxygen() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let oxygen_val = oxygen(5, input);
        assert_eq!(23, oxygen_val);
    }

    #[test]
    fn test_c02() {
        let input = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let co2_val = co2(5, input);
        assert_eq!(10, co2_val);
    }
}
