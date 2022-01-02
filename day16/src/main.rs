use bitstream_io::{BigEndian, BitRead, BitReader};
use std::io::Cursor;

fn parse_input(input: &str) -> Vec<u8> {
    let nibble_stream = input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).expect(format!("can not convert from hex char: {}", c).as_str()) as u8)
        .collect::<Vec<u8>>();

    nibble_stream
        .chunks(2)
        .map(|w| w[1] + 16 * w[0])
        .collect::<Vec<u8>>()
}

#[derive(Debug)]
struct Packet {
    version: u32,
    type_id: u32,
    size: u32,
    payload: Payload,
}

impl Packet {
    fn len(&self) -> u32 {
        self.size
    }

    fn get_version_sum(&self) -> u32 {
        let mut ver_sum = 0;

        ver_sum += match &self.payload {
            Payload::Literal(_) => self.version,
            Payload::PacketList(pl) => {
                let mut sum  = self.version;
                for sp in pl {
                    sum += sp.get_version_sum();
                }
                sum
            }
        };
        return ver_sum;
    }

}


#[derive(Debug)]
enum Payload {
    Literal(u32),
    PacketList(Vec<Packet>),
}

fn read_packet<I: BitRead>(reader: &mut I) -> Option<Packet> {
    let version = reader.read(3).unwrap_or(0xFF);
    if version == 0xFF {
        return None;
    }

    let type_id = reader.read(3).unwrap_or(0xFF);
    if type_id == 0xFF {
        return None;
    }

    let payload = match type_id {
        4 => read_literal(reader),
        _ => read_packet_list(reader),
    };

    match payload {
        None => None,
        Some((p, sz)) => Some(Packet {
            version: version,
            type_id: type_id,
            size: sz,
            payload: p,
        }),
    }
}

fn read_literal<I: BitRead>(reader: &mut I) -> Option<(Payload, u32)> {
    let mut more_nibbles = true;
    let mut literal = 0;

    let mut nibble_counter = 0;
    while more_nibbles {
        more_nibbles = reader.read_bit().unwrap();
        let nibble = reader.read::<u8>(4).unwrap() as u32;
        literal <<= 4;
        literal += nibble;

        nibble_counter += 1;
    }

    Some((
        Payload::Literal(literal),
        6 + 5 * nibble_counter,
    ))

}

fn read_packet_list<I: BitRead>(reader: &mut I) -> Option<(Payload, u32)> {
    let mut is_mode_number_of_subpackets: bool = false;
    match  reader.read_bit() {
        Ok(bit) => is_mode_number_of_subpackets = bit,
        Err(_) => (),
    }
    let mut length_field_sz = 15;
    let mut length_field: u32 = 0;
    let mut packet_list = vec![];

    if is_mode_number_of_subpackets {
        length_field_sz = 11;

        let packets_in_payload: u32 = reader.read(length_field_sz).unwrap();

        let mut packet = read_packet(reader);
        let mut packets_read = 0;
        loop {
            match packet {
                Some(p) => {
                    packets_read += 1;
                    packet_list.push(p);
                    if packets_read == packets_in_payload.try_into().unwrap() {
                        for pp in &packet_list {
                            length_field += pp.len() as u32;
                        }
                        break;
                    }
                    packet = read_packet(reader);
                }
                None => {
                    break;
                }
            }
        }


    } else {

        length_field = reader.read(length_field_sz).unwrap();

        let mut packet = read_packet(reader);
        let mut bits_read = 0;
        loop {
            match packet {
                Some(p) => {
                    bits_read += p.size;
                    packet_list.push(p);
                    if bits_read == length_field.try_into().unwrap() {
                        break;
                    }
                    packet = read_packet(reader);
                }
                None => {
                    break;
                }
            }
        }
    }

    Some((Payload::PacketList(packet_list), 7 + length_field_sz + length_field))
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input16.txt");
    let byte_stream = parse_input(input);
    let mut cursor = Cursor::new(&byte_stream);
    let mut reader = BitReader::endian(&mut cursor, BigEndian);

    let packet = read_packet(&mut reader).unwrap();
    let version_sum = packet.get_version_sum();

    println!("Version sum is: {}", version_sum);

}

#[cfg(test)]
mod test {

    use super::*;

    const F1: &'static str = "D2FE28";
    const F2: &'static str = "38006F45291200";
    const F3: &'static str = "EE00D40C823060";
    const F4: &'static str = "8A004A801A8002F478";
    const F5: &'static str = "620080001611562C8802118E34";
    const F6: &'static str = "C0015000016115A2E0802F182340";
    const F7: &'static str = "A0016C880162017C3686B18A3D4780";


    #[test]
    fn test_version_sum() {
        let byte_stream = parse_input(&F1);
        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let packet = read_packet(&mut reader).unwrap();

        assert_eq!(6, packet.get_version_sum());


        let byte_stream = parse_input(&F2);
        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let packet = read_packet(&mut reader).unwrap();

        assert_eq!(9, packet.get_version_sum());


        let byte_stream = parse_input(&F3);
        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let packet = read_packet(&mut reader).unwrap();

        assert_eq!(14, packet.get_version_sum());

        let byte_stream = parse_input(&F4);
        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let packet = read_packet(&mut reader).unwrap();

        assert_eq!(16, packet.get_version_sum());

        let byte_stream = parse_input(&F5);
        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let packet = read_packet(&mut reader).unwrap();

        assert_eq!(12, packet.get_version_sum());

        let byte_stream = parse_input(&F6);
        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let packet = read_packet(&mut reader).unwrap();

        assert_eq!(23, packet.get_version_sum());

        let byte_stream = parse_input(&F7);
        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let packet = read_packet(&mut reader).unwrap();

        assert_eq!(31, packet.get_version_sum());

    }



    #[test]
    fn test_read_packet_with_packet_list_packetcounttype() {
        let byte_stream = parse_input(&F3);

        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);

        let packet = read_packet(&mut reader).unwrap();
        assert_eq!(7, packet.version);
        assert_eq!(3, packet.type_id);
        assert_eq!(14*4-5, packet.len());
        let plist = match packet.payload {
            Payload::PacketList(plist) => plist,
            _ => vec![],
        };
        assert_eq!(3, plist.len());

        let packet = &plist[0];
        assert_eq!(2, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(11, packet.size);
        let literal = match packet.payload {
            Payload::Literal(literal) => literal,
            _ => 0,
        };
        assert_eq!(1, literal);

        let packet = &plist[1];
        assert_eq!(4, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(11, packet.size);
        let literal = match packet.payload {
            Payload::Literal(literal) => literal,
            _ => 0,
        };
        assert_eq!(2, literal);

        let packet = &plist[2];
        assert_eq!(1, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(11, packet.size);
        let literal = match packet.payload {
            Payload::Literal(literal) => literal,
            _ => 0,
        };
        assert_eq!(3, literal);

    }


    #[test]
    fn test_read_packet_with_packet_list_lengthtype() {
        let byte_stream = parse_input(&F2);

        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);

        let packet = read_packet(&mut reader).unwrap();
        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);
        assert_eq!(14*4-7, packet.size);
        assert_eq!(14*4-7, packet.len());
        let plist = match packet.payload {
            Payload::PacketList(plist) => plist,
            _ => vec![],
        };
        assert_eq!(2, plist.len());

        let packet = &plist[0];
        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(11, packet.size);
        let literal = match packet.payload {
            Payload::Literal(literal) => literal,
            _ => 0,
        };
        assert_eq!(10, literal);

        let packet = &plist[1];
        assert_eq!(2, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(16, packet.size);
        let literal = match packet.payload {
            Payload::Literal(literal) => literal,
            _ => 0,
        };
        assert_eq!(20, literal)
    }

    #[test]
    fn test_read_packet_with_literal() {
        let byte_stream = parse_input(&F1);

        let mut cursor = Cursor::new(&byte_stream);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);

        let packet = read_packet(&mut reader).unwrap();
        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(21, packet.len());
        let literal = match packet.payload {
            Payload::Literal(literal) => literal,
            _ => 0,
        };
        assert_eq!(2021, literal)
    }

    #[test]
    fn test_bitstream_input() {
        let byte_stream = parse_input(&F1);

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
