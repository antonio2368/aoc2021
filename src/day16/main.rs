struct ByteIter<'a> {
    data: &'a Vec<u8>,
    byte_pos: usize,
    bit_pos: usize,
}

impl<'a> ByteIter<'a> {
    fn new(data: &Vec<u8>) -> ByteIter {
        ByteIter {
            data,
            byte_pos: 0,
            bit_pos: 7,
        }
    }
}

impl<'a> Iterator for ByteIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_pos == self.data.len() {
            return None;
        }

        let result = self.data[self.byte_pos] & (1 << self.bit_pos) > 0;

        if self.bit_pos == 0 {
            self.bit_pos = 7;
            self.byte_pos += 1;
        } else {
            self.bit_pos -= 1;
        }

        Some(result as u8)
    }
}

fn num_from_bits(byte_iter: &mut ByteIter, num_bits: usize) -> usize {
    let mut num: usize = 0;

    for i in (0..num_bits).rev() {
        num |= (byte_iter.next().unwrap() as usize) << i;
    }
    num
}

fn apply_operation(operands: Vec<usize>, packet_type: usize) -> usize {
    match packet_type {
        0 => operands.iter().sum(),
        1 => operands.into_iter().reduce(|a, b| a * b).unwrap(),
        2 => operands.into_iter().reduce(|a, b| a.min(b)).unwrap(),
        3 => operands.into_iter().reduce(|a, b| a.max(b)).unwrap(),
        5 => {
            assert_eq!(operands.len(), 2);
            (operands[0] > operands[1]) as usize
        }
        6 => {
            assert_eq!(operands.len(), 2);
            (operands[0] < operands[1]) as usize
        }
        7 => {
            assert_eq!(operands.len(), 2);
            (operands[0] == operands[1]) as usize
        }
        _ => panic!("Invalid opearator"),
    }
}

fn solution(byte_iter: &mut ByteIter) -> (usize, usize, usize) {
    let mut read_bits = 0;
    let mut read_num = |num| {
        read_bits += num;
        num_from_bits(byte_iter, num)
    };
    let mut version_sum = read_num(3);
    let packet_type = read_num(3);
    if packet_type == 4 {
        let mut num: usize = 0;
        loop {
            let literal_bits = read_num(5);
            num <<= 4;
            num |= literal_bits & 0xF;
            if literal_bits & (1 << 4) == 0 {
                return (num, version_sum, read_bits);
            }
        }
    } else {
        let mut operands: Vec<usize> = Vec::new();
        if read_num(1) == 0 {
            let mut bits_to_read = read_num(15);

            while bits_to_read > 0 {
                let (literal, subpacket_sum, subpacket_bits_num) = solution(byte_iter);
                read_bits += subpacket_bits_num;
                version_sum += subpacket_sum;
                bits_to_read -= subpacket_bits_num;
                operands.push(literal);
            }
        } else {
            let subpacket_num = read_num(11);
            for _ in 0..subpacket_num {
                let (literal, subpacket_sum, subpacket_bits_num) = solution(byte_iter);
                read_bits += subpacket_bits_num;
                version_sum += subpacket_sum;
                operands.push(literal);
            }
        }
        (
            apply_operation(operands, packet_type),
            version_sum,
            read_bits,
        )
    }
}

fn main() {
    let file_content = std::fs::read_to_string("res/day16/input.txt").unwrap();

    for line in file_content.lines() {
        let mut bytes: Vec<u8> = Vec::new();

        for i in (0..line.len() - 1).step_by(2) {
            bytes.push(u8::from_str_radix(&line[i..i + 2], 16).unwrap());
        }

        if line.len() % 2 == 1 {
            bytes.push((line.chars().rev().next().unwrap().to_digit(16).unwrap() as u8) << 4);
        }

        let mut byte_iter = ByteIter::new(&bytes);

        let (result, version_sum, _) = solution(&mut byte_iter);
        println!("Part 1 result {}", version_sum);
        println!("Part 2 result {}", result);
    }
}
