use std::fs;
use std::io::{self, BufRead};

fn avg(ns: &[i64]) -> i64 {
    let mut c = 0;
    ns.iter().for_each(|n| c += n);
    c / ns.len() as i64
}

fn calculate_cost(ns: &[i64], pos: i64) -> i64 {
    ns.iter().map(|x| (x - pos).abs()).into_iter().sum()
}

fn sum_series(n: i64) -> i64 {
    n*(n+1)/2
}

fn calculate_crab_cost(ns: &[i64], pos: i64) -> i64 {
    ns.iter().map(|x| sum_series( (x-pos).abs() ) ).into_iter().sum()
}

// returns index of the max and value
fn max(ns: &[i64]) -> (usize, i64) {
    let mut max_val = 0;
    let mut max_index: usize = 0;
    for (i, n) in ns.iter().enumerate() {
        if *n > max_val {
            max_val = *n;
            max_index = i;
        }
    }
    (max_index, max_val)
}

fn min(ns: &[i64]) -> i64 {
    let mut min_val = 0;
    for n in ns.iter() {
        if *n < min_val {
            min_val = *n;
        }
    }
    min_val
}

fn calculate_min_cost(ns: &[i64]) -> (usize, i64) {
    let mut min_cost = calculate_cost(ns, 0 as i64);
    let mut min_pos = 0;

    for i in 1..min_cost {
        let cost = calculate_cost(ns, i as i64);
        if cost <= min_cost {
            min_cost = cost;
            min_pos = i as usize;
        }
    }
    (min_pos, min_cost)
}

fn calculate_min_crab_cost(ns: &[i64]) -> (usize, i64) {
    let mut min_cost = calculate_crab_cost(ns, 0 as i64);
    let mut min_pos = 0;

    for i in 1..min_cost {
        let cost = calculate_crab_cost(ns, i as i64);
        if cost <= min_cost {
            min_cost = cost;
            min_pos = i as usize;
        }
    }
    (min_pos, min_cost)
}

fn main() {
    let file_name = "input07.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);

    let line = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("line: {:?}", line);
    println!("avg: {}", avg(&line));

    let a = avg(&line);
    //let (pos, min_cost) = calculate_min_cost(&line);
    //println!("Min cost: {} at pos {}", min_cost, pos);
    //let (pos, min_cost) = calculate_min_crab_cost(&line);
    let mut costs = vec![];
    for pos in [a-1, a, a+1] {
        costs.push(calculate_crab_cost(&line, pos));
        println!("{} => ({:?}", pos, costs)
    }
    println!("Min crab cost: {}", min(costs.as_slice()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_series() {
        assert_eq!(10, sum_series(4));
        assert_eq!(15, sum_series(5));
        assert_eq!(0, sum_series(0));
        assert_eq!(1, sum_series(1));
        assert_eq!(3, sum_series(2));
        assert_eq!(6, sum_series(3));

    }



    #[test]
    fn test_calculate_min_crab_cost() {
        let ns = &[16i64, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let (p, m) = calculate_min_crab_cost(ns);
        assert_eq!(168, m);
        assert_eq!(5, p);
    }


    #[test]
    fn test_calculate_min_cost() {
        let ns = &[16i64, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let (p, m) = calculate_min_cost(ns);
        assert_eq!(37, m);
        assert_eq!(2, p);
    }

    #[test]
    fn test_calculate_avg() {
        let ns = &[16i64, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let m = avg(ns);
        assert_eq!(4, m);
    }

    #[test]
    fn test_max() {
        let ns = &[16i64, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(0, max(ns).0);
        assert_eq!(16, max(ns).1);
    }

    #[test]
    fn test_min() {
        let ns = &[16i64, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(0, min(ns));

    }

    #[test]
    fn test_calculate_cost() {
        let ns: &[i64] = &[16i64, 1, 2, 0];
        assert_eq![ns.iter().sum::<i64>(), calculate_cost(ns, 0)];

        let ns: &[i64] = &[16i64, 1, 2, 0];
        assert_eq![8 + 7 + 6 + 8, calculate_cost(ns, 8)]
    }

    #[test]
    fn test_calculate_crab_cost() {

        let ns = &[16i64, 16, 16];
        let c = calculate_crab_cost(ns, 16);
        assert_eq!(0, c);

        let c = calculate_crab_cost(ns, 15);
        assert_eq!(3, c);




    }

}
