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

/// Filters out the corrupted lines at the input by removing them from the vector
/// Returns the Result with number of lines filtered or Error
fn filter_out_corrupted_lines(inputs: &mut Vec<String>) -> Result<u32, String> {
    let mut remove_counter = 0;

    inputs.retain(|input| {
        let keep = match find_first_error_bracket(input) {
            None => true,
            Some(_) => {
                remove_counter += 1;
                false
            },
        };
        keep
    });


    Ok(remove_counter)
}

fn get_score_missing_sequence(seq: Vec<char>) -> u64 {
    let mut score: u64 = 0;
    for c in seq {
        score = 5 * score + get_closing_bracket_score2(c).unwrap()
    }
    score
}


fn get_closing_bracket_score2(closing_bracket: char) -> Option<u64> {
    match closing_bracket {
        ']' => Some(2),
        ')' => Some(1),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}

/// Returns a string vector with the missing chars
fn get_missing_sequence(input: &str) -> Vec<char> {
    let mut expected_brackets = VecDeque::<char>::new();

    for ib in input.chars() {
        match get_closing_bracket_score(ib) {
            Some(_) => {
                let expected = expected_brackets.pop_back().unwrap();
                if expected == ib {
                    continue;
                } else {
                    panic!("Found an incorrect closing bracket!");
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
    let mut missing_chars = Vec::with_capacity(expected_brackets.len());
    while expected_brackets.len() > 0 {
        missing_chars.push(expected_brackets.pop_back().unwrap());
    }
    missing_chars
}


fn get_middle_score(inputs: &mut Vec<String>) -> u64 {
   let _ = filter_out_corrupted_lines(inputs);
    let mut scores: Vec<u64> = Vec::with_capacity(inputs.len());
    inputs.into_iter().for_each(|input| {
        scores.push(
            get_score_missing_sequence(
                get_missing_sequence(&input)
            ));
    });
    scores.sort();
    let middle = scores.len()/2;  // force round up
    scores[middle]
}

fn main() {
    let file_name = "input10.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);
    let mut inputs = parse_input(reader);

    println!("Total error score (part 1): {}", get_score(&inputs));

    let score = get_middle_score(&mut inputs);
    println!("Middle score {}", score );

}

#[cfg(test)]
mod test {
    use super::*;

    const F : &'static [u8] = b"[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]" as &[u8];


    #[test]
    fn test_get_middle_score() {
       let reader = io::BufReader::new(F);
        let mut inputs = parse_input(reader);

        assert_eq!(288957, get_middle_score(&mut inputs));
    }

    #[test]
    fn test_get_score_missing_sequence() {
        let reader = io::BufReader::new(F);
        let mut inputs = parse_input(reader);

        filter_out_corrupted_lines(&mut inputs);

        let missing_seq_scores: [u64; 5] = [288957, 5566, 1480781, 995444, 294];
        for (input, score) in inputs.iter().zip(missing_seq_scores) {
            let missing = get_missing_sequence(input);
            assert_eq!(score, get_score_missing_sequence(missing));
        }

        //assert_eq!(288957, get_middle_score(inputs);
    }

    #[test]
    fn test_get_missing_sequences() {
        let reader = io::BufReader::new(F);
        let mut inputs = parse_input(reader);

        filter_out_corrupted_lines(&mut inputs);

        let mut counter = 0;
        let missing_seq_lens: [usize; 5] = [8, 6, 9, 9, 4];
        for (input, ln) in inputs.iter().zip(missing_seq_lens) {
            let missing = get_missing_sequence(input);
            assert_eq!(ln, missing.len());
            if counter == 0 {
                let vals = "}}]])})]";
                for (ec, fc) in vals.chars().zip(missing) {
                    assert_eq!(ec, fc);
                }
            }
            counter += 1;
        }
    }

    #[test]
    fn test_filter_out_corrupted_lines() {
        let reader = io::BufReader::new(F);
        let mut inputs = parse_input(reader);
        let initial_size = inputs.len();
        let res = filter_out_corrupted_lines(&mut inputs);

        assert_eq!(5, res.unwrap());
        assert_eq!(initial_size - 5, inputs.len());

        for input in inputs {
            println!("{}", input);
            assert_eq!(None, find_first_error_bracket(&input));
        }

    }


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
