use aoc_lib::*;
use bumpalo::{
    collections::{CollectIn, Vec},
    Bump,
};

aoc_setup!(Day16, sample 1: 6, part 1: 967, part 2: 12883091136209);

#[derive(Debug)]
struct BitReader<'a> {
    data: &'a [u8],
    index: usize,
    bit_index: usize,
}

impl<'a> BitReader<'a> {
    pub fn from(data: &'a [u8]) -> Self {
        BitReader {
            data,
            index: 0,
            bit_index: 0,
        }
    }

    pub fn bit_index(&self) -> usize {
        self.index * 8 + self.bit_index
    }

    pub fn read_bit(&mut self) -> bool {
        let mask = 1 << (7 - self.bit_index);
        let bit = self.data[self.index] & mask != 0;
        self.increment_count();
        bit
    }

    pub fn read_bits(&mut self, count: usize) -> u64 {
        let mut bits = 0;

        for _ in 0..count {
            bits <<= 1;
            bits |= self.read_bit() as u64;
        }

        bits
    }

    pub fn read_uleb16(&mut self) -> u64 {
        let mut total = 0;

        loop {
            let next = self.read_bit();
            let data = self.read_bits(4);

            total <<= 4;
            total |= data;

            if !next {
                break;
            }
        }

        total
    }

    fn increment_count(&mut self) {
        self.bit_index += 1;

        if self.bit_index == 8 {
            self.bit_index = 0;
            self.index += 1;
        }
    }
}

#[derive(Debug)]
struct Packet<'bump> {
    pub version: u8,
    pub data: PacketData<'bump>,
}

impl<'a> Packet<'a> {
    pub fn parse(data: &mut BitReader<'a>, bump: &'a Bump) -> Self {
        let version = data.read_bits(3) as u8;
        let data = PacketData::parse(data, bump);

        Self { version, data }
    }

    pub fn sum_versions(&self) -> usize {
        self.version as usize + self.data.sum_versions()
    }

    pub fn evaluate(&self) -> u64 {
        self.data.evaluate()
    }
}

#[derive(Debug)]
enum PacketData<'a> {
    Literal(u64),                      // id 4
    Operator(u8, Vec<'a, Packet<'a>>), // any other id
}

impl<'a> PacketData<'a> {
    pub fn parse(data: &mut BitReader<'a>, bump: &'a Bump) -> Self {
        match data.read_bits(3) as u8 {
            4 => PacketData::Literal(data.read_uleb16()),
            i => {
                // operator packet
                let length_type_id = data.read_bit();

                let sub_packets = match length_type_id {
                    false => {
                        let bit_count = data.read_bits(15) as usize;
                        let bit_start = data.bit_index();

                        // borrowchecker complains if I use take_while on infinite iterator
                        let mut sub_packets = Vec::new_in(bump);
                        while data.bit_index() - bit_start != bit_count {
                            sub_packets.push(Packet::parse(data, bump));
                        }
                        sub_packets
                    }
                    true => {
                        let count = data.read_bits(11);

                        (0..count)
                            .map(|_| Packet::parse(data, bump))
                            .collect_in::<Vec<_>>(bump)
                    }
                };

                PacketData::Operator(i, sub_packets)
            }
        }
    }

    pub fn sum_versions(&self) -> usize {
        match self {
            PacketData::Literal(_) => 0,
            PacketData::Operator(_, packets) => packets.iter().map(Packet::sum_versions).sum(),
        }
    }

    pub fn evaluate(&self) -> u64 {
        match &self {
            Self::Literal(value) => *value,
            PacketData::Operator(type_id, packets) => match type_id {
                0 => packets.iter().map(Packet::evaluate).sum(),
                1 => packets.iter().map(Packet::evaluate).product(),
                2 => packets.iter().map(Packet::evaluate).min().unwrap(),
                3 => packets.iter().map(Packet::evaluate).max().unwrap(),
                5 => (packets[0].evaluate() > packets[1].evaluate()) as u64,
                6 => (packets[0].evaluate() < packets[1].evaluate()) as u64,
                7 => (packets[0].evaluate() == packets[1].evaluate()) as u64,
                _ => panic!("unexpected type id {}", type_id),
            },
        }
    }
}

pub struct Day16;

impl AdventOfCode for Day16 {
    type Input = std::vec::Vec<u8>;
    type Output = u64;

    fn parse_input(s: &str) -> Self::Input {
        hex::decode(s).unwrap()
    }

    fn solve_1(input: &Self::Input) -> Self::Output {
        let bump = Bump::new();
        let mut iter = BitReader::from(input);
        let data = Packet::parse(&mut iter, &bump);
        data.sum_versions() as u64
    }

    fn solve_2(input: &Self::Input) -> Self::Output {
        let bump = Bump::new();
        let mut iter = BitReader::from(input);
        let data = Packet::parse(&mut iter, &bump);
        data.evaluate()
    }
}

macro_rules! add_test {
    ($name:ident for $ident:ident: $source:literal becomes $target:literal) => {
        #[test]
        pub fn $name() {
            let input = hex::decode($source).unwrap();
            let output = Day16::$ident(&input);
            assert_eq!(output, $target);
        }
    };
}

add_test!(part_1_extra_test_1 for solve_1: "8A004A801A8002F478" becomes 16);
add_test!(part_1_extra_test_2 for solve_1: "620080001611562C8802118E34" becomes 12);
add_test!(part_1_extra_test_3 for solve_1: "C0015000016115A2E0802F182340" becomes 23);
add_test!(part_1_extra_test_4 for solve_1: "A0016C880162017C3686B18A3D4780" becomes 31);

add_test!(part_2_extra_test_sum for solve_2: "C200B40A82" becomes 3);
add_test!(part_2_extra_test_product for solve_2: "04005AC33890" becomes 54);
add_test!(part_2_extra_test_min for solve_2: "880086C3E88112" becomes 7);
add_test!(part_2_extra_test_max for solve_2: "CE00C43D881120" becomes 9);
add_test!(part_2_extra_test_lt for solve_2: "D8005AC2A8F0" becomes 1);
add_test!(part_2_extra_test_gt for solve_2: "F600BC2D8F" becomes 0);
add_test!(part_2_extra_test_eq for solve_2: "9C005AC2F8F0" becomes 0);
add_test!(part_2_extra_test_eq_compound for solve_2: "9C0141080250320F1802104A08" becomes 1);
