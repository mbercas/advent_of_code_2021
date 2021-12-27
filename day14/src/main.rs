use std::io::{self, BufRead};
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

type Buckets = HashMap<String, usize>;
type Map = HashMap<String, char>;
type ElementMap = HashMap<char, usize>;

fn parse_input<I: BufRead>(reader: I) -> (Buckets, Map, ElementMap) {
    let mut buckets = Buckets::new();
    let mut map = Map::new();
    let mut elements = ElementMap::new();

    /*
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for a in &alphabet {
        for b in &alphabet {
            buckets.insert(format!("{}{}", a, b), 0);
        }
    }
     */
    for line_ in reader.lines() {
        let line =  line_.unwrap();

        if line.contains("->") {
            let mut parts = line.trim().split("->");
            let key = parts.next().unwrap().trim();
            let c = parts.next().unwrap().trim();
            map.insert(key.to_string(), c.chars().next().unwrap());
        } else {
            if 0 < line.len() {
                let vec: Vec<char> = line.chars().collect();
                vec.iter().for_each(|c| *elements.entry(*c).or_insert(0) += 1);

                for pair in vec.windows(2) {
                    let key = format!("{}{}", pair[0], pair[1]);
                    *buckets.entry(key).or_insert(0) += 1;
                }
            }
        }
    }

    (buckets, map, elements)
}

fn step(buckets: &mut Buckets, map: &Map, elements: &mut ElementMap) {
    let mut pairs = buckets.clone();

    //println!("-> {:?}", pairs);

    for (key, val) in pairs.iter_mut() {

        let new_char = map.get(key).unwrap();
        *elements.entry(*new_char).or_insert(0) += *val;

        let mut key_iter = key.chars();
        let pair_first_char = key_iter.next().unwrap();
        let pair_second_char = key_iter.next().unwrap();

        let new_pair_1 = format!("{}{}",
                               pair_first_char,
                               new_char);

        let new_pair_2 = format!("{}{}",
                                 new_char,
                                 pair_second_char);


        if let Entry::Occupied(mut o) = buckets.entry(String::from(key)) {
            if *o.get_mut() > *val {
                *o.get_mut() -= *val;
            } else {
                o.remove_entry();
            }
        }

        *buckets.entry(new_pair_1).or_insert(0) += *val;
        *buckets.entry(new_pair_2).or_insert(0) += *val;
    }
    //println!("<- {:?}", buckets);
    //println!("{:?}", elements);
}

fn get_difference(elements: &mut ElementMap) -> usize {

    let mut vec: Vec<usize> = elements.values()
        .map(|v| *v)
        .collect();
    vec.sort();

    vec[vec.len()-1] - vec[0]
}

fn main() {
    let input_file = "input14.txt";
    let f= fs::File::open(input_file).unwrap();
    let reader = io::BufReader::new(f);
    let (mut buckets, map, mut elements) = parse_input(reader);
    for _ in 0..10 {
        step(&mut buckets, &map, &mut elements);
    }

    let diff = get_difference(&mut elements);
    println!("Diff after 10 iterations is {}", diff);

        for _ in 10..40 {
        step(&mut buckets, &map, &mut elements);
    }

    let diff = get_difference(&mut elements);
    println!("Diff after 40 iterations is {}", diff);
}


#[cfg(test)]
mod test {
    use super::*;

    const F: &'static [u8] = b"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C" as &[u8];

    fn get_elements_sum(elements: &ElementMap ) -> usize {
        let mut sum = 0;
        for (_key, val) in  elements {
            sum += *val;
        }
        sum
    }
    fn get_buckets_sum(b: &Buckets ) -> usize {
        let mut sum = 0;
        for (_key, val) in b {
            sum += *val;
        }
        sum
    }

    #[test]
    fn test_get_difference() {
        let reader = io::BufReader::new(F);
        let (mut buckets, map, mut elements) = parse_input(reader);
        for _ in 0..10 {
            step(&mut buckets, &map, &mut elements);
        }

        assert_eq!(1588, get_difference(&mut elements) );
    }

    #[test]
    fn test_step() {
        let reader = io::BufReader::new(F);
        let (mut buckets, map, mut elements) = parse_input(reader);

        let mut elements_sum = get_elements_sum(&elements);
        let buckets_sum = get_buckets_sum(&buckets);
        assert_eq!(4, elements_sum);
        assert_eq!(elements_sum-1, buckets_sum);

        step(&mut buckets, &map, &mut elements);

        elements_sum = 2*elements_sum - 1;
        assert_eq!(elements_sum, get_elements_sum(&elements));
        assert_eq!(elements_sum-1, get_buckets_sum(&buckets));

        assert_eq!(Some(&2), elements.get(&'B'));
        assert_eq!(Some(&2), elements.get(&'N'));
        assert_eq!(Some(&2), elements.get(&'C'));
        assert_eq!(Some(&1), elements.get(&'H'));

        assert_eq!( Some(&(1_usize)), buckets.get(&String::from("NC")));
        assert_eq!( Some(&(1_usize)), buckets.get(&String::from("CN")));
        assert_eq!( Some(&(1_usize)), buckets.get(&String::from("NB")));
        assert_eq!( Some(&(1_usize)), buckets.get(&String::from("BC")));
        assert_eq!( Some(&(1_usize)), buckets.get(&String::from("CH")));
        assert_eq!( Some(&(1_usize)), buckets.get(&String::from("HB")));


        step(&mut buckets, &map, &mut elements);

        elements_sum = 2*elements_sum - 1;
        assert_eq!(elements_sum, get_elements_sum(&elements));
        assert_eq!(elements_sum-1, get_buckets_sum(&buckets));

        assert_eq!(Some(&6), elements.get(&'B'));
        assert_eq!(Some(&2), elements.get(&'N'));
        assert_eq!(Some(&4), elements.get(&'C'));
        assert_eq!(Some(&1), elements.get(&'H'));

        step(&mut buckets, &map, &mut elements);

        elements_sum = 2*elements_sum - 1;
        assert_eq!(elements_sum, get_elements_sum(&elements));
        assert_eq!(elements_sum-1, get_buckets_sum(&buckets));

        assert_eq!(Some(&11), elements.get(&'B'));


        for _ in 3..10 {
            step(&mut buckets, &map, &mut elements);
            elements_sum = 2*elements_sum - 1;
            assert_eq!(elements_sum, get_elements_sum(&elements));
            assert_eq!(elements_sum-1, get_buckets_sum(&buckets));
        }
        assert_eq!(Some(&1749), elements.get(&'B'));
        assert_eq!(Some(&298), elements.get(&'C'));
        assert_eq!(Some(&161), elements.get(&'H'));
        assert_eq!(Some(&865), elements.get(&'N'));


        assert_eq!(16, map.len());
    }

    #[test]
    fn test_parse_input() {
        let reader = io::BufReader::new(F);
        let (buckets, map, elements) = parse_input(reader);

        assert_eq!(3, elements.len());
        assert_eq!(Some(&2), elements.get(&'N'));


        assert_eq!(3, buckets.len());

        let pairs = ["NN", "NC", "CB"];

        for pair in pairs {
            assert_eq!( Some(&(1_usize)), buckets.get(pair));
        }

        assert_eq!(16, map.len());

        let pairs = ["CH", "HH", "CB", "NH",       "CN"];
        let chars = ['B',  'N',  'H',  'C',        'C' ];

        for (p, c) in pairs.iter().zip(chars) {
            assert_eq!(Some(&c), map.get(*p));
        }
    }
}
