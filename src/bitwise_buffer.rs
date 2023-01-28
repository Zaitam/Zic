pub(crate) struct BitwiseBuffer {
    buffer: Vec<u8>,
    remaining_bits: usize,
    current_byte: u8,
}

pub(crate) struct EncodedBit {
    pub data: u8, //TODO: Make this work for u8, u16, u32, u64
    pub bit_len: usize,
}

impl BitwiseBuffer {
    pub fn new() -> BitwiseBuffer {
        BitwiseBuffer {
            buffer: vec![],
            remaining_bits: 8,
            current_byte: 0,
        }
    }

    pub fn push_bits(&mut self, bits: &Vec<EncodedBit>) {
        for encoded_bit in bits {
            if encoded_bit.bit_len <= self.remaining_bits {
                self.push_to_curr_bit(encoded_bit.data, encoded_bit.bit_len);
                if self.remaining_bits == 0 {
                    self.push_bit();
                }
                continue;
            }
            let n = encoded_bit.bit_len - self.remaining_bits;
            self.push_to_curr_bit(encoded_bit.data >> (n), self.remaining_bits);
            self.push_bit();
            self.push_to_curr_bit(((encoded_bit.data << (8 - n)) as u8) >> (8 - n) ,n);
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