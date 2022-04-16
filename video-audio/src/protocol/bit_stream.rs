use std::{cell::Cell, io::Read};

pub struct BitStream<'a> {
    size: usize,
    bits_left: Cell<usize>,
    data: &'a [u8],
    pos: Cell<usize>,
}

impl<'a> BitStream<'a> {
    pub fn new<'b>(data: &'b [u8]) -> Self
    where
        'b: 'a,
    {
        Self {
            size: data.len() * 8,
            bits_left: Cell::new(8),
            data,
            pos: Cell::new(0),
        }
    }

    pub fn get_one_bit(&self) -> u8 {
        let mut r = 0;
        {
            let num = self.bits_left.get();
            self.bits_left.set(num)
        }
        r = r >> (self.bits_left.get()) & 0x01;
        if self.bits_left.get() == 0 {
            self.pos.set(self.pos.get() + 1);
            self.bits_left.set(0);
        }
        r
    }

    pub fn get_n_bit(&self, n_bits: usize) -> usize {
        let mut r = 0usize;
        for i in 0..n_bits {
            r = r | ((self.get_one_bit() as usize) << (n_bits - i - 1));
        }
        r
    }

    pub fn get_ue(&self) -> usize {
        let mut r = 0;
        let mut r_size = 0;
        while self.get_one_bit() == 0 && r_size < 32 {
            r_size += 1;
        }
        r = self.get_n_bit(r_size);
        r = r - 1;
        r
    }

    pub fn get_se(&self) -> i64 {
        let mut r = self.get_ue() as i64;
        if r & 0x01 == 1 {
            r = (r + 1) / 2;
        } else {
            r = -(r / 2);
        }
        r
    }
}
