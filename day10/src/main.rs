use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::fs;

fn parse_input<I: BufRead>(reader: I) -> Vec<String> {
    let mut inputs = vec![];
    for line_ in reader.lines() {
        let line = line_.unwrap();
        inputs.push(String::from(line));
    }
    inputs
}

fn get_closing_bracket(input_bracket: char) -> Option<char> {
    match input_bracket {
        '[' => Some(']'),
        '(' => Some(')'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn get_closing_bracket_score(closing_bracket: char) -> Option<u32> {
    match closing_bracket {
        ']' => Some(57),
        ')' => Some(3),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

// Returns expected and found
fn find_first_error_bracket(input: &str) -> Option<(char, char)> {
    let mut expected_brackets = VecDeque::<char>::new();

    for ib in input.chars() {
        match get_closing_bracket_score(ib) {
            Some(_) => {
                let expected = expected_brackets.pop_back().unwrap();
                if expected == ib {
                    continue;
                } else {
                    return Some((expected, ib));
                }
            }
            None => {
                let ob = get_closing_bracket(ib);
                match ob {
                    Some(c) => expected_brackets.push_back(c),
                    None => panic!("found unexpected char as closing bracket"),
                }
            }
        }
    }
    None
}

fn get_score(inputs: &Vec<String>) -> u32 {

    let mut score = 0;

    for input in inputs {
        let error = find_first_error_bracket(input);
        score += match  error {
            Some(v) => get_closing_bracket_score(v.1).unwrap(),
            None => 0,
        }
    }
    score
}

fn main() {
    let file_name = "input10.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);
    let inputs = parse_input(reader);

    println!("Total error score: {}", get_score(&inputs));
}

#[cfg(test)]
mod test {
    use super::*;

    const F : &'static [u8] = b"[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]" as &[u8];

    #[test]
    fn test_find_score() {
        let reader = io::BufReader::new(F);
        let inputs = parse_input(reader);
        let score = get_score(&inputs);
        assert_eq!(26397, score);
    }

    #[test]
    fn test_find_first_error_bracket() {
        let reader = io::BufReader::new(F);
        let inputs = parse_input(reader);
        let expected = vec![None, None,
                            Some( (']', '}') ), None,
                            Some( (']', ')') ),
                            Some( (')', ']') ), None,
                            Some( ('>', ')') ),
                            Some( (']', '>') ), None];

        for (expected_values, input) in expected.iter().zip(inputs) {
            assert_eq!(*expected_values, find_first_error_bracket(&input));
        }
    }

    #[test]
    fn test_get_closing_bracket() {
        let inputs = "[{(<";
        let outputs = "]})>";

        for (i, o) in inputs.chars().zip(outputs.chars()) {
            assert_eq!(Some(o), get_closing_bracket(i));
        }
    }
}
