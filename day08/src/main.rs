use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

type Readings = Vec<(Vec<String>, Vec<String>)>;

fn parse_input<I>(reader: I) -> Readings
where
    I: BufRead,
{
    let mut output: Readings = vec![];
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let mut parts = line.split('|');

        let inputs = parts
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|x| String::from(x.trim()))
            .collect::<Vec<String>>();
        let outputs = parts
            .next()
            .unwrap()
            .trim()
            .split(' ')
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
                _ => (),
            }
        }
    }
    (cnt_1, cnt_4, cnt_7, cnt_8)
}

// Converts the string into an integer, each bit represents one character
fn encode(str_reading: &str) -> u8 {
    let mut num_reading = 0;
    for letter in str_reading.chars() {
        num_reading += match letter {
            'a' => 0b000_0001,
            'b' => 0b000_0010,
            'c' => 0b000_0100,
            'd' => 0b000_1000,
            'e' => 0b001_0000,
            'f' => 0b010_0000,
            'g' => 0b100_0000,
            _ => panic!("Unsupported character found"),
        };
    }
    num_reading
}

fn find_encoding(inputs: &[String]) -> HashMap<u8, u8> {
    let mut omap: HashMap<u8, u8> = HashMap::new();

    let mut encoded_one = 0;
    let mut encoded_three = 0;
    let mut encoded_five = 0;
    let mut encoded_six = 0;


    // find the unique numbers 1, 4, 7, 8
    for s in inputs {
        match s.len() {
            2 => {
                encoded_one = encode(s);
                omap.insert(encoded_one, 1);
            },
            3 => {
                omap.insert(encode(s), 7);
            },
            4 => {omap.insert(encode(s), 4);},
            7 => {omap.insert(encode(s), 8);},
            _ => (),
        }
    }

    // 1, 3, 4, 6, 7, 8
    for s in inputs {
        match s.len() {
            6 => {
                let tmp = encode(s);
                if (encoded_one & tmp) != encoded_one {
                    encoded_six = tmp;
                    omap.insert(tmp, 6);
                }
            },
            5 => {
                let tmp  = encode(s);
                if encoded_one & tmp == encoded_one {
                    encoded_three = tmp;
                    omap.insert(encoded_three, 3);
                }
            },
            _ => ()
        }
    }

    // 1, 2, 3, 4, 5, 6, 7, 8
    for s in inputs {
        match s.len() {
            5 => {
                let tmp = encode(s);
                if tmp != encoded_three {
                    if (tmp & encoded_six) == tmp {
                        encoded_five = tmp;
                        omap.insert(tmp, 5);
                    } else {
                        omap.insert(tmp, 2);
                    }
                }
            },
            _ => ()
        }
    }

    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    for s in inputs {
        match s.len() {
            6 => {
                let tmp = encode(s);
                if tmp != encoded_six {
                    if (encoded_five & tmp) == encoded_five {
                        omap.insert(tmp, 9);
                    } else {
                        omap.insert(tmp, 0);
                    }
                }
            },
            _ => ()
        }
    }
    omap
}

fn output_to_number(omap: &HashMap<u8, u8>, output_strings: &Vec<String>) -> u32 {
    let mut onumber = 0;

    let mut i: i32 = 3;
    for s in output_strings {
        onumber += 10_u32.pow(i as u32) * (*omap.get(&encode(s)).unwrap() as u32);
        i -= 1;
    }
    onumber
}

fn main() {
    let file_name = "input08.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);

    let readings = parse_input(reader);
    let (cnt_1, cnt_4, cnt_7, cnt_8) = simple_counter(&readings);
    println!("Total outputs 1, 4, 7, 8: {}, expected {}", cnt_1 + cnt_4 + cnt_7 + cnt_8, 261);


    let mut result = 0;
    for (inputs, outputs) in readings {
        let omap = find_encoding(&inputs);
        result +=  output_to_number(&omap, &outputs);
    }
    println!("Total sum of outputs: {} - expected {}", result, 987553)


}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    const F: &'static [u8] =
        b"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            as &[u8];

    const SOLS: &'static [u32; 10] = &[8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];


    #[test]
    fn test_output_to_number() {
        let reader = io::BufReader::new(F);
        let readings = parse_input(reader);
        for ((inputs, outputs), sol) in readings.iter().zip(SOLS) {
            let omap = find_encoding(inputs);
            let onum =  output_to_number(&omap, outputs);
            assert_eq!(*sol, onum);

        }
    }

    #[test]
    fn test_find_encoding() {
        let inputs = vec![
            String::from("abcefg"),
            String::from("cf"),
            String::from("acdeg"),
            String::from("acdfg"),
            String::from("bcdf"),
            String::from("abdfg"),
            String::from("abdefg"),
            String::from("acf"),
            String::from("abcdefg"),
            String::from("abcdfg")
        ];
        let omap = find_encoding(&inputs);

        assert_eq!(1, *omap.get(&0b010_0100).unwrap());
        assert_eq!(3, *omap.get(&0b110_1101).unwrap());
        assert_eq!(6, *omap.get(&0b111_1011).unwrap());
        assert_eq!(7, *omap.get(&0b010_0101).unwrap());
        assert_eq!(8, *omap.get(&0b111_1111).unwrap());

        assert_eq!(2, *omap.get(&0b101_1101).unwrap());


        assert_eq!(0, *omap.get(&0b111_0111).unwrap());
        assert_eq!(5, *omap.get(&0b110_1011).unwrap());
        assert_eq!(9, *omap.get(&0b110_1111).unwrap());

    }

    #[test]
    fn test_encode() {
        assert_eq!(1, encode("a"));
        assert_eq!(0b111_1111, encode("abcdefg"));
        assert_eq!(0b111_1111, encode("gfedcba"));
        assert_eq!(0b010_1010, encode("fdb"));
    }

    #[test]
    #[should_panic]
    fn test_encode_panic_invalid_char() {
        encode("m");
    }


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
