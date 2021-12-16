fn parse_input(handle: impl std::io::BufRead) -> String {
    let line = handle.lines().next().unwrap().unwrap();
    let mut result = String::with_capacity(line.len() * 4);
    line.chars()
        .for_each(|c| result.push_str(Parser::base_convert(c)));
    result
}

struct Parser {
    total_version: u32,
}

impl Parser {
    fn base_convert(s: char) -> &'static str {
        match s {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!(),
        }
    }

    fn new() -> Self {
        Parser { total_version: 0 }
    }

    fn parse_literal(&mut self, stream: &mut impl Iterator<Item = char>) -> (u32, i64) {
        let mut result_str = String::new();
        let mut is_continuation = true;
        let mut bits_read = 0;

        while is_continuation {
            is_continuation = stream.next().unwrap() == '1';
            let digits_str = stream.take(4).collect::<String>();
            result_str.push_str(&digits_str);
            bits_read += 5;
        }

        let result = i64::from_str_radix(&result_str, 2).unwrap();

        (bits_read, result)
    }

    fn parse_operator(
        &mut self,
        stream: &mut impl Iterator<Item = char>,
        op: fn(i64, i64) -> i64,
    ) -> (u32, i64) {
        let length_type_id = stream.next().unwrap();
        let mut bits_read = 1;
        let result;
        let mut subpackets_results = Vec::new();
        let mut subpackets_bits_read = 0;

        if length_type_id == '0' {
            let total_length =
                u32::from_str_radix(&stream.take(15).collect::<String>(), 2).unwrap();
            bits_read += 15;
            while subpackets_bits_read < total_length {
                let subpacket_result = self.parse_pkt(stream);
                subpackets_bits_read += subpacket_result.0;
                subpackets_results.push(subpacket_result.1);
            }
            assert_eq!(subpackets_bits_read, total_length);
        } else {
            let num_subpackets =
                u32::from_str_radix(&stream.take(11).collect::<String>(), 2).unwrap();
            bits_read += 11;
            for _ in 0..num_subpackets {
                let subpacket_result = self.parse_pkt(stream);
                subpackets_bits_read += subpacket_result.0;
                subpackets_results.push(subpacket_result.1);
            }
        }

        bits_read += subpackets_bits_read;

        let initial_value = subpackets_results[0];
        result = subpackets_results
            .into_iter()
            .skip(1)
            .fold(initial_value, op);

        (bits_read, result)
    }

    fn parse_pkt(&mut self, stream: &mut impl Iterator<Item = char>) -> (u32, i64) {
        let pkt_version = u32::from_str_radix(&stream.take(3).collect::<String>(), 2).unwrap();
        self.total_version += pkt_version;
        let pkt_type = u32::from_str_radix(&stream.take(3).collect::<String>(), 2).unwrap();
        let mut bits_read = 6;

        let result = match pkt_type {
            4 => self.parse_literal(stream),
            0 => self.parse_operator(stream, |a, b| a + b),
            1 => self.parse_operator(stream, |a, b| a * b),
            2 => self.parse_operator(stream, std::cmp::min),
            3 => self.parse_operator(stream, std::cmp::max),
            5 => self.parse_operator(stream, |a, b| (a > b) as i64),
            6 => self.parse_operator(stream, |a, b| (a < b) as i64),
            7 => self.parse_operator(stream, |a, b| (a == b) as i64),
            _ => panic!(),
        };
        bits_read += result.0;

        (bits_read, result.1)
    }
}

fn main() {
    let stream = parse_input(std::io::stdin().lock());
    let mut p = Parser::new();
    let (bits_read, result) = p.parse_pkt(&mut stream.chars());

    println!("Total bits read: {}", bits_read);
    println!("Part 1 - Result: {}", p.total_version);
    println!("Part 2 - Result: {}", result);
}
