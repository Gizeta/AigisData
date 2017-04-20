pub struct Reader<'a> {
    data: &'a Vec<u8>,
    pub offset: usize,
    bit_cache: u32,
    bit_count: u8,
}

impl<'a> Reader<'a> {
    pub fn create(data: &Vec<u8>) -> Reader {
        Reader {
            data: data,
            offset: 0,
            bit_cache: 0,
            bit_count: 0,
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.offset > self.data.len()
    }

    pub fn read_u8(&mut self) -> u8 {
        let ret = self.data[self.offset];
        self.offset += 1;

        ret
    }

    pub fn read_u32(&mut self) -> u32 {
        let vec = self.read_bytes(4);

        vec[0] as u32 | ((vec[1] as u32) << 8) | ((vec[2] as u32) << 16) | ((vec[3] as u32) << 24)
    }

    pub fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(len);
        for _ in 0..len {
            ret.push(self.data[self.offset]);
            self.offset += 1;
        }

        ret
    }

    pub fn read_chars(&mut self, len: usize) -> String {
        String::from_utf8(self.read_bytes(len)).unwrap()
    }

    pub fn read_bit(&mut self) -> u8 {
        self.read_bits(1)
    }

    pub fn read_bits(&mut self, len: u8) -> u8 {
        while self.bit_count < len {
            self.bit_cache |= (self.read_u8() as u32) << self.bit_count;
            self.bit_count += 8;
        }

        let mut mask = 0;
        for x in 0..len {
            mask |= 1 << x;
        }

        let ret = self.bit_cache & mask;
        self.bit_cache >>= len;
        self.bit_count -= len;

        ret as u8
    }
}
