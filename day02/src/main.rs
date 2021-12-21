use std::fs;
use std::io;
use std::io::BufRead;

#[allow(dead_code)]
fn parse_commands<I>(reader: I) -> (i32, i32)
where
    I: BufRead,
{
    let mut horizontal = 0;
    let mut vertical = 0;
    for (cnt, line) in reader.lines().enumerate() {
        let lstr = match line {
            Ok(lstr) => lstr,
            Err(why) => panic!("Error reading line {}: {}", cnt, why),
        };

        let vec: Vec<&str> = lstr.as_str().split(" ").collect();
        match vec[0] {
            "forward" => horizontal += vec[1].parse::<i32>().unwrap(),
            "up" => vertical -= vec[1].parse::<i32>().unwrap(),
            "down" => vertical += vec[1].parse::<i32>().unwrap(),
            _ => panic!("invalid command {} in line {}", vec[0], cnt),
        }
    }
    (horizontal, vertical)
}

#[allow(dead_code)]
fn parse_commands_corrected<I>(reader: I) -> (i32, i32)
where
    I: BufRead,
{
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for (cnt, line) in reader.lines().enumerate() {
        let lstr = match line {
            Ok(lstr) => lstr,
            Err(why) => panic!("Error reading line {}: {}", cnt, why),
        };

        let vec: Vec<&str> = lstr.as_str().split(" ").collect();
        match vec[0] {
            "forward" => {
                let x = vec[1].parse::<i32>().unwrap();
                horizontal += x;
                vertical += aim * x;
            }
            "up" => aim -= vec[1].parse::<i32>().unwrap(),
            "down" => aim += vec[1].parse::<i32>().unwrap(),
            _ => panic!("invalid command {} in line {}", vec[0], cnt),
        }
    }
    (horizontal, vertical)
}

fn main() {
    let file_name = "input02.txt";
    let f = match fs::File::open(file_name) {
        Ok(f) => f,
        Err(why) => panic!("Could not open file {}: error -> {}", file_name, why),
    };

    let reader = io::BufReader::new(f);
    //let (horizontal, vertical) = parse_commands(reader);
    // println!(
    //     "Solution 1\nH: {}, V: {}, HxV: {}",
    //     horizontal,
    //     vertical,
    //     horizontal * vertical
    // );
    let (horizontal, vertical) = parse_commands_corrected(reader);
    println!(
        "Solution 2\nH: {}, V: {}, HxV: {}",
        horizontal,
        vertical,
        horizontal * vertical
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let file = b"forward 1\ndown 2\nup 1" as &[u8];
        let reader = io::BufReader::new(file);
        let (horizontal, vertical) = parse_commands(reader);

        assert_eq!(horizontal, 1);
        assert_eq!(vertical, 1);
    }

    #[test]
    #[should_panic]
    fn test_parsing_invalid_shift() {
        let file = b"forward A\ndown 2\nup 1" as &[u8];
        let reader = io::BufReader::new(file);
        let (_horizontal, _vertical) = parse_commands(reader);
    }

    #[test]
    #[should_panic]
    fn test_parsing_invalid_command() {
        let file = b"foo 1\ndown 2\nup 1" as &[u8];
        let reader = io::BufReader::new(file);
        let (_horizontal, _vertical) = parse_commands(reader);
    }

    #[test]
    fn test_parsing_corr() {
        let file = b"forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2" as &[u8];
        let reader = io::BufReader::new(file);
        let (horizontal, vertical) = parse_commands_corrected(reader);

        assert_eq!(horizontal, 15);
        assert_eq!(vertical, 60);
    }


    #[test]
    #[should_panic]
    fn test_parsing_corr_invalid_shift() {
        let file = b"forward A\ndown 2\nup 1" as &[u8];
        let reader = io::BufReader::new(file);
        let (_horizontal, _vertical) = parse_commands_corrected(reader);
    }

    #[test]
    #[should_panic]
    fn test_parsing_corr_invalid_command() {
        let file = b"foo 1\ndown 2\nup 1" as &[u8];
        let reader = io::BufReader::new(file);
        let (_horizontal, _vertical) = parse_commands_corrected(reader);
    }


}
