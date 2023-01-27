struct BitwiseBuffer {
    buffer: Vec<u8>,
    remaining_bits: usize,
    current_byte: u8,
}

struct EncodedBit {
    data: u8, //TODO: Make this work for u8, u16, u32, u64
    bit_len: usize,
}
/*enum Unsigned {
    U16(u16),
    U32(u32),
    U64(u64),
}*/

impl BitwiseBuffer {
    fn new() -> BitwiseBuffer {
        BitwiseBuffer {
            buffer: vec![],
            remaining_bits: 4,
            current_byte: 0,
        }
    }

    fn push_bits<T: Into<Unsigned>>(&mut self, bits: Vec<EncodedBit>) {
        for encoded_bit in bits {
            if encoded_bit.bit_len <= self.remaining_bits { //If the data enters in remaining bits
                self.push_to_curr_bit(encoded_bit.data, encoded_bit.bit_len);
                if self.remaining_bits == 0 {
                    self.push_bit();
                }
            } else {
                // usize division does a floor func automatically so no need to floor or ciel, just add 7
                let iterations = (encoded_bit.bit_len - self.remaining_bits + 7)/8;
                // first iter
                let n = encoded_bit.bit_len - self.remaining_bits;
                self.push_to_curr_bit(encoded_bit.data >> (8 - self.remaining_bits), self.remaining_bits);
                self.push_bit();
                // Doesn't include first and last iterations
                for i in 1..iterations {
                    self.push_to_curr_bit( (encoded_bit.data >> n - (i * 8)) as u8, 8);
                    self.push_bit();
                }
                // last iter
                let len = n - ((iterations - 1) * 8);
                self.push_to_curr_bit((((encoded_bit.data << (8 - len)) as u8) >> (8 - len)) ,len);
            }
        }
    }

    fn push_to_curr_bit(&mut self, data: u8, len: usize) {
        self.current_byte = (self.current_byte << len) + data;
        self.remaining_bits -= len;
    }

    fn push_bit(&mut self) {
        self.buffer.push(self.current_byte);
        self.current_byte = 0;
        self.remaining_bits = 8;
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer() {
        let mut buffer = BitwiseBuffer::new();
        buffer.push_bits(vec![EncodedBit {data: 3, bit_len: 4 }, EncodedBit {data: 2, bit_len: 2 }, EncodedBit {data: 1, bit_len: 2 }, EncodedBit {data: 0, bit_len: 2 }, EncodedBit {data: 3, bit_len: 2 }]);
        println!("{:?}", buffer.buffer);

        //assert_eq!(result, 4);
    }
}*/