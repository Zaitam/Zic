pub struct BitwiseBuffer {
    buffer: Vec<u8>,
    bit_count: usize,
    current_byte: u8,
}

pub struct EncodedBit<T> {
    pub(crate) data: T,
    pub(crate) bit_len: usize,
}

impl BitwiseBuffer {
    pub fn new() -> BitwiseBuffer {
        BitwiseBuffer {
            buffer: vec![],
            bit_count: 0,
            current_byte: 0,
        }
    }

    pub fn push_bits<T>(&mut self, bits: Vec<EncodedBit<T>>) where T: Into<u8> {
        for encoded_bit in bits {
            self.current_byte |= encoded_bit.data << (8 - self.bit_count - encoded_bit.bit_len);
            self.bit_count += encoded_bit.bit_len;
            if self.bit_count >= 8 {
                self.push_last_bit();
            }
        }
    }

    fn push_last_bit(&mut self) {
        self.buffer.push(self.current_byte);
        self.current_byte = 0;
        self.bit_count -= 8;
    }

    fn flush(&mut self) -> Vec<u8> {
        if self.bit_count > 0 {
            self.buffer.push(self.current_byte << (8 - self.bit_count));
        }
        std::mem::replace(&mut self.buffer, vec![])
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