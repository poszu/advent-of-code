#[derive(Debug, PartialEq)]
struct PacketHeader {
    version: u8,
    id: u8,
}

#[derive(Debug, PartialEq)]
struct Packet {
    header: PacketHeader,
    value: PacketValue,
}
#[derive(Debug, PartialEq)]

enum PacketValue {
    Literal(usize),
    Operator(Vec<Packet>),
}

impl PacketValue {
    fn as_literal(&self) -> usize {
        if let PacketValue::Literal(val) = self {
            *val
        } else {
            panic!("PacketValue is not a literal");
        }
    }
}

impl Packet {
    fn version_sum(&self) -> usize {
        let sum = self.header.version as usize;

        match &self.value {
            PacketValue::Literal(_) => sum,
            PacketValue::Operator(packets) => {
                sum + packets
                    .iter()
                    .fold(0, |acc, packet| acc + packet.version_sum())
            }
        }
    }

    fn eval(&self) -> PacketValue {
        match &self.value {
            &PacketValue::Literal(val) => PacketValue::Literal(val),
            PacketValue::Operator(packets) => {
                let eval_to_literals = || packets.iter().map(|p| p.eval().as_literal());
                match self.header.id {
                    0 => {
                        //sum
                        PacketValue::Literal(eval_to_literals().sum())
                    }
                    1 => {
                        // product
                        PacketValue::Literal(eval_to_literals().product())
                    }
                    2 => {
                        // minimum
                        PacketValue::Literal(eval_to_literals().min().expect("subpackets empty"))
                    }
                    3 => {
                        // maximum
                        PacketValue::Literal(eval_to_literals().max().expect("subpackets empty"))
                    }
                    5 => {
                        // greater
                        PacketValue::Literal(
                            (packets[0].eval().as_literal() > packets[1].eval().as_literal())
                                as usize,
                        )
                    }
                    6 => {
                        // less
                        PacketValue::Literal(
                            (packets[0].eval().as_literal() < packets[1].eval().as_literal())
                                as usize,
                        )
                    }
                    7 => {
                        // equal
                        PacketValue::Literal(
                            (packets[0].eval().as_literal() == packets[1].eval().as_literal())
                                as usize,
                        )
                    }
                    other => {
                        panic!("Invalid packet ID: {}", other)
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Bits<'a> {
    data: &'a str,
    cursor: usize,
}

impl<'a> Bits<'a> {
    fn new(data: &'a str) -> Self {
        Self { data, cursor: 0 }
    }

    fn next_u8(&mut self, count: usize) -> Option<u8> {
        assert!(count <= 8);

        let pos_start = self.cursor / 4;
        let pos_end = (self.cursor + count - 1) / 4 + 1;
        let mut byte = if let Some(first_byte_substr) = self.data.get(pos_start..pos_end) {
            u16::from_str_radix(first_byte_substr, 16).unwrap()
        } else {
            return None;
        };
        let width = (pos_end - pos_start) * 4;
        let pos = self.cursor % 4;
        byte <<= 16 - width + pos;
        byte >>= 16 - count;
        self.cursor += count;

        Some(byte as u8)
    }

    fn next_u16(&mut self, count: usize) -> Option<u16> {
        assert!(count <= 16);

        let pos_start = self.cursor / 4;
        let pos_end = (self.cursor + count - 1) / 4 + 1;
        let mut byte = if let Some(first_byte_substr) = self.data.get(pos_start..pos_end) {
            u32::from_str_radix(first_byte_substr, 16).unwrap()
        } else {
            return None;
        };
        let width = (pos_end - pos_start) * 4;
        let pos = self.cursor % 4;
        byte <<= 32 - width + pos;
        byte >>= 32 - count;
        self.cursor += count;

        Some(byte as u16)
    }
}

enum Len {
    Bits(u16),
    SubPackets(u16),
}

impl From<(u8, u16)> for Len {
    fn from((id, data): (u8, u16)) -> Self {
        match id {
            0 => Len::Bits(data),
            1 => Len::SubPackets(data),
            _ => panic!("Wrong Len ID"),
        }
    }
}

fn parse_packet(mut bits: Bits) -> Option<(Packet, Bits)> {
    let header = PacketHeader {
        version: bits.next_u8(3)?,
        id: bits.next_u8(3)?,
    };

    let value = if header.id == 4 {
        // literal
        let mut literal = 0;
        loop {
            let end = bits.next_u8(1)? == 0;
            let val = bits.next_u8(4)?;
            literal = (literal << 4) + val as usize;

            if end {
                break;
            }
        }
        PacketValue::Literal(literal)
    } else {
        // operator
        let len_id = bits.next_u8(1)?;
        let len_field_size = match len_id {
            0 => 15,
            1 => 11,
            _ => unreachable!(),
        };
        let len = Len::from((len_id, bits.next_u16(len_field_size)?));
        let offset = bits.cursor;
        let mut subpackets = Vec::new();
        while let Some((packet, new_bits)) = parse_packet(bits) {
            bits = new_bits;
            subpackets.push(packet);
            match len {
                Len::Bits(size) => {
                    if bits.cursor >= offset + size as usize {
                        break;
                    }
                }
                Len::SubPackets(size) => {
                    if subpackets.len() == size as usize {
                        break;
                    }
                }
            }
        }
        PacketValue::Operator(subpackets)
    };

    Some((Packet { header, value }, bits))
}

fn main() {
    let bits = Bits::new(include_str!("input.txt").lines().next().unwrap());

    let packet = if let Some((p, _)) = parse_packet(bits) {
        p
    } else {
        panic!("Failed to parse the packet!")
    };

    println!("PART1: The sum is {}", packet.version_sum());

    println!("PART2: The result of evaluation is: {:?}", packet.eval());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bits() {
        let input = "D2FE28";
        let mut bits = Bits {
            data: input,
            cursor: 0,
        };

        assert_eq!(Some(6), bits.next_u8(3));
        assert_eq!(Some(4), bits.next_u8(3));
        assert_eq!(Some(0b10111), bits.next_u8(5));
        assert_eq!(Some(0b11110), bits.next_u8(5));
        assert_eq!(Some(0b00101), bits.next_u8(5));
        assert_eq!(Some(0), bits.next_u8(3));
        assert_eq!(None, bits.next_u8(1));
    }

    #[test]
    fn test_parse_literal_packet() {
        let input = "D2FE28";
        let (packet, data) = parse_packet(Bits::new(input)).unwrap();
        assert_eq!(
            Packet {
                header: PacketHeader { version: 6, id: 4 },
                value: PacketValue::Literal(2021)
            },
            packet
        );
        assert_eq!(6, packet.version_sum());
        assert_eq!(None, parse_packet(data));
    }

    #[test]
    fn test_parse_operator_packet() {
        let input = "8A004A801A8002F478";
        let (packet, data) = parse_packet(Bits::new(input)).unwrap();
        assert_eq!(
            Packet {
                header: PacketHeader { version: 4, id: 2 },
                value: PacketValue::Operator(vec![Packet {
                    header: PacketHeader { version: 1, id: 2 },
                    value: PacketValue::Operator(vec![Packet {
                        header: PacketHeader { version: 5, id: 2 },
                        value: PacketValue::Operator(vec![Packet {
                            header: PacketHeader { version: 6, id: 4 },
                            value: PacketValue::Literal(15)
                        }])
                    }])
                }])
            },
            packet
        );
        assert_eq!(16, packet.version_sum());
        assert_eq!(None, parse_packet(data));

        let input = "EE00D40C823060";
        let (packet, _) = parse_packet(Bits::new(input)).unwrap();
        assert_eq!(14, packet.version_sum());

        let input = "620080001611562C8802118E34";
        let (packet, _) = parse_packet(Bits::new(input)).unwrap();
        assert_eq!(12, packet.version_sum());

        let input = "C0015000016115A2E0802F182340";
        let (packet, _) = parse_packet(Bits::new(input)).unwrap();
        assert_eq!(23, packet.version_sum());

        let input = "A0016C880162017C3686B18A3D4780";
        let (packet, _) = parse_packet(Bits::new(input)).unwrap();
        assert_eq!(31, packet.version_sum());
    }

    #[test]
    fn test_add_packets() {
        assert_eq!(
            PacketValue::Literal(7),
            Packet {
                header: PacketHeader { version: 4, id: 0 },
                value: PacketValue::Operator(vec![
                    Packet {
                        header: PacketHeader { version: 1, id: 2 },
                        value: PacketValue::Literal(3)
                    },
                    Packet {
                        header: PacketHeader { version: 1, id: 2 },
                        value: PacketValue::Literal(4)
                    }
                ])
            }
            .eval()
        );
    }

    #[test]
    fn test_packets_prod() {
        assert_eq!(
            PacketValue::Literal(12),
            Packet {
                header: PacketHeader { version: 4, id: 1 },
                value: PacketValue::Operator(vec![
                    Packet {
                        header: PacketHeader { version: 1, id: 2 },
                        value: PacketValue::Literal(3)
                    },
                    Packet {
                        header: PacketHeader { version: 1, id: 2 },
                        value: PacketValue::Literal(4)
                    }
                ])
            }
            .eval()
        );
    }

    #[test]
    fn test_part2() {
        let (packet, _) = parse_packet(Bits::new("C200B40A82")).unwrap();
        assert_eq!(PacketValue::Literal(3), packet.eval());

        let (packet, _) = parse_packet(Bits::new("04005AC33890")).unwrap();
        assert_eq!(PacketValue::Literal(54), packet.eval());

        let (packet, _) = parse_packet(Bits::new("880086C3E88112")).unwrap();
        assert_eq!(PacketValue::Literal(7), packet.eval());

        let (packet, _) = parse_packet(Bits::new("CE00C43D881120")).unwrap();
        assert_eq!(PacketValue::Literal(9), packet.eval());

        let (packet, _) = parse_packet(Bits::new("D8005AC2A8F0")).unwrap();
        assert_eq!(PacketValue::Literal(1), packet.eval());

        let (packet, _) = parse_packet(Bits::new("F600BC2D8F")).unwrap();
        assert_eq!(PacketValue::Literal(0), packet.eval());

        let (packet, _) = parse_packet(Bits::new("9C005AC2F8F0")).unwrap();
        assert_eq!(PacketValue::Literal(0), packet.eval());

        let (packet, _) = parse_packet(Bits::new("9C0141080250320F1802104A08")).unwrap();
        assert_eq!(PacketValue::Literal(1), packet.eval());
    }
}
