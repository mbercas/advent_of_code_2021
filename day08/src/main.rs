use std::io::{self, BufRead};
use std::fs;

type Readings = Vec<(Vec<String>, Vec<String>)>;

fn parse_input<I>(reader: I) -> Readings
where I: BufRead {
    let mut output: Readings = vec![];
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let mut parts = line.split('|');

        let inputs = parts.next().unwrap()
            .trim().split(' ')
            .map(|x| String::from(x.trim()))
            .collect::<Vec<String>>();
        let outputs = parts.next().unwrap()
            .trim().split(' ')
            .map(|x| String::from(x.trim()))
            .collect::<Vec<String>>();

        output.push((inputs, outputs));
    }
    output
}

fn simple_counter(readings: &Readings) -> (i32, i32, i32, i32) {
    let mut cnt_1 = 0;
    let mut cnt_4 = 0;
    let mut cnt_7 = 0;
    let mut cnt_8 = 0;
    for (_, r) in readings {
        for s in r {
            match s.len() {
                2 => cnt_1 += 1,
                3 => cnt_7 += 1,
                4 => cnt_4 += 1,
                7 => cnt_8 += 1,
                _ => ()
            }
        }
    }
    (cnt_1, cnt_4, cnt_7, cnt_8)
}

fn main() {
    let file_name = "input08.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);

    let readings = parse_input(reader);
    let (cnt_1, cnt_4, cnt_7, cnt_8) = simple_counter(&readings);
    println!("Total outputs 1,4, 7, 8: {}", cnt_1 + cnt_4 + cnt_7 + cnt_8)
}


#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    const F: &'static [u8] = b"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce" as &[u8];


    #[test]
    fn test_parse_input() {
        let reader = io::BufReader::new(F);
        let readings = parse_input(reader);
        assert_eq!(10, readings.len());
        for r in readings {
            assert_eq!(10, r.0.len());
            assert_eq!(4, r.1.len());
        }
    }
    #[test]
    fn test_simple_counter() {
        let reader = io::BufReader::new(F);
        let readings = parse_input(reader);

        let (cnt_1, cnt_4, cnt_7, cnt_8) = simple_counter(&readings);
        assert_eq!(8, cnt_1);
        assert_eq!(6, cnt_4);
        assert_eq!(5, cnt_7);
        assert_eq!(7, cnt_8);
        assert_eq!(26, cnt_1 + cnt_4 + cnt_7 + cnt_8);
    }
}
