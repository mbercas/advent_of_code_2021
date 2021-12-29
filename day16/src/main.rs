use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};
use std::io::{Cursor, Read};

fn parse_input(input: &str) -> Vec<u8> {
    let nibble_stream = input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect::<Vec<u8>>();

    nibble_stream
        .chunks(2)
        .map(|w| w[1] + 16 * w[0])
        .collect::<Vec<u8>>()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use super::*;

    const F: &'static str = "D2FE28";

    #[test]
    fn test_bitstream_input() {
        let byte_stream = parse_input(&F);

        let mut cursor = Cursor::new(&byte_stream);

        let mut reader = BitReader::endian(&mut cursor, BigEndian);

        match reader.read(3) {
            Ok(version) => assert_eq!(6, version),
            Err(msg) => assert!(false, "Should not return error: {}", msg),
        }

        match reader.read(3) {
            Ok(type_id) => assert_eq!(4, type_id),
            Err(msg) => assert!(false, "Should not return error: {}", msg),
        }

        for v in [7, 14] {
            match reader.read_bit() {
                Ok(is_not_last) => assert_eq!(true, is_not_last),
                Err(msg) => assert!(false, "Should not return error: {}", msg),
            }

            match reader.read(4) {
                Ok(b) => assert_eq!(v, b),
                Err(msg) => assert!(false, "Should not return error: {}", msg),
            }
        }
        match reader.read_bit() {
            Ok(is_not_last) => assert_eq!(false, is_not_last),
            Err(msg) => assert!(false, "Should not return error: {}", msg),
        }

        match reader.read(4) {
            Ok(b) => assert_eq!(5, b),
            Err(msg) => assert!(false, "Should not return error: {}", msg),
        }
    }
}
