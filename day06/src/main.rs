use std::fs;
use std::io::{self, BufRead};

struct Fish {
    counter: u32,
    first_generation: bool,
}

impl Fish {
    fn new(counter: u32) -> Fish {
        Fish {
            counter: counter,
            first_generation: true,
        }
    }

    fn iterate(&mut self) {
        self.counter = if self.counter == 0 {
            self.first_generation = false;
            6
        } else {
            self.counter - 1u32
        }
    }
}

struct Bank {
    ages: Vec<u64>,
    generation_counter: u32,
}

impl Bank {
    fn new(age_list: &[u32]) -> Bank {
        let mut bank = Bank {
            ages: vec![0; 9],
            generation_counter: 0u32,
        };
        for idx in age_list {
            bank.ages[*idx as usize] += 1
        }
        bank
    }

    fn iterate(&mut self) {
        let zero_val = self.ages[0];
        // i <= i+1
        // 0 -> 6 & 8
        for i in 0..8 {
            self.ages[i] = self.ages[i + 1];
        }
        self.ages[8] = zero_val;
        self.ages[6] = self.ages[6] + zero_val;

        self.generation_counter += 1;
    }

    fn sum(&self) -> u64 {
        let mut total = 0;
        for i in &self.ages {
            total += *i;
        }
        total
    }
}

fn get_children_start_times_seed(max_time: u32, start_time: u32, start_value: u32) -> Vec<u32> {
    let mut time = start_time + start_value + 1;
    let mut vec = vec![];
    while time <= max_time {
        vec.push(time);
        time += 7;
    }
    vec
}

fn count_fishes(max_time: u32, start_time: u32, start_value: u32) -> usize {
    let mut vec = vec![(start_time, start_value)];
    let mut counter = 0;
    while vec.len() > 0 {
        let node = vec.pop().unwrap();
        counter += 1;
        for ch in get_children_start_times_seed(max_time, node.0, node.1) {
            vec.push((ch, 8));
        }
    }
    counter
}

fn main() {
    let file_name = "input06.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);
    let mut ages = vec![];

    for line in reader.lines() {
        match line {
            Ok(lstr) => {
                for n in lstr.trim().split(',') {
                    ages.push(n.parse::<u32>().unwrap());
                }
            }
            Err(why) => panic!("Error reading input file: {}", why),
        }
    }
    let mut bank = Bank::new(&ages);
    while bank.generation_counter < 80 {
        bank.iterate()
    }
    println!("Size after 80 days: {}", bank.sum());
    while bank.generation_counter < 256 {
        bank.iterate()
    }
    println!("Size after 80 days: {}", bank.sum());

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_fishes() {
        let m = count_fishes(18, 0, 3);
        assert_eq!(5, m);

        let mut counter = 0;
        for i in [3, 4, 3, 1, 2] {
            counter += count_fishes(18, 0, i)
        }
        assert_eq!(26, counter);

        counter = 0;
        for i in [3, 4, 3, 1, 2] {
            counter += count_fishes(80, 0, i)
        }
        assert_eq!(5934, counter);
    }


    #[test]
    fn test_get_children_start_times_seed() {
        let start_times = get_children_start_times_seed(18, 0, 3);

        assert_eq!(3, start_times.len());
        for (i, t) in start_times.iter().zip([4u32, 11, 18].into_iter()) {
            assert_eq!(*i, t);
        }

        let start_times = get_children_start_times_seed(18, 0, 4);

        assert_eq!(2, start_times.len());
        for (i, t) in start_times.iter().zip([5u32, 12].into_iter()) {
            assert_eq!(*i, t);
        }
    }

    #[test]
    fn test_bank_iterate() {
        let mut bank = Bank::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(0, bank.generation_counter);

        bank.iterate();
        for (i, val) in [1u64, 1, 1, 1, 1, 1, 2, 1, 1].iter().enumerate() {
            assert_eq!(1, bank.generation_counter);
            assert_eq!(*val, bank.ages[i], "index: {}", i)
        }


        let mut bank =  Bank::new(&[3, 4, 3, 1, 2]);
        for _ in 0..80 {
            bank.iterate()
        }
        assert_eq!(5934, bank.sum());

    }

    #[test]
    fn test_bank_new() {
        let bank = Bank::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(0, bank.generation_counter);

        for age in bank.ages.iter() {
            assert_eq!(1, *age);
        }
    }

    #[test]
    fn test_fish_new() {
        let fish = Fish::new(5);
        assert_eq!(5, fish.counter);
        assert_eq!(true, fish.first_generation);
    }

    #[test]
    fn test_fish_iterate() {
        let mut fish = Fish::new(0);

        assert_eq!(0, fish.counter);
        assert_eq!(true, fish.first_generation);

        fish.iterate();
        assert_eq!(6, fish.counter);
        assert_eq!(false, fish.first_generation);

        for i in (0..6).rev() {
            fish.iterate();
            assert_eq!(i, fish.counter);
            assert_eq!(false, fish.first_generation);
        }
        fish.iterate();
        assert_eq!(6, fish.counter);
        assert_eq!(false, fish.first_generation);
    }
}
