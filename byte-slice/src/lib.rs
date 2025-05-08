use anyhow::{Result, anyhow};
use std::ops::{Index, Range, RangeFrom, RangeTo, RangeFull, RangeInclusive};


#[derive(Debug)]
pub struct Bytes<'a> {
    idx_first: usize, 
    idx_last: usize, 
    data: &'a [u8]
}


pub trait Hex {
    fn hex_stream_to_vec(&self) -> Vec<u8>;
}

impl Hex for &str {
    fn hex_stream_to_vec(&self) -> Vec<u8> {
        let mut bytes_vec: Vec<u8> = Vec::new();

        for i in 0..self.len() / 2 {
            match u8::from_str_radix(&self[i*2 .. (i*2+2)], 16) {
                Ok(byte) => {
                    bytes_vec.push(byte);
                },
                Err(_) => panic!("Bytes::from_hex_str: Invalid hex str"),
            }
        }

        bytes_vec
    }
}



impl<'a> Bytes<'a>{
    pub fn from_slice(slice: &'a[u8]) -> Self {
        Self { 
            idx_first: 0, 
            idx_last: slice.len(), 
            data: slice 
        }
    }

    // moves the index that the bytes object is reading from

    // bytes           = [1, 2, 3, 4, 5, 6, 7]
    // bytes.idx_first =  ^
    // bytes.shift_first(2);
    // bytes           = [1, 2, 3, 4, 5, 6, 7]
    // bytes.idx_first =        ^
    pub fn shift_first(&mut self, byte_count: usize) -> Result<()> {
        self.idx_first += byte_count;

        match self.idx_first <= self.idx_last {
            true => Ok(()),
            false => Err(anyhow!("Not enough data left to shift {byte_count} bytes"))
        }
    }

    pub fn shift_last(&mut self, byte_count: usize) -> Result<()> {
        self.idx_last -= byte_count;

        match self.idx_last >= self.idx_first {
            true => Ok(()),
            false => Err(anyhow!("Not enough data left to shift {byte_count} bytes"))
        }
    }

    pub fn reset(&mut self) {
        self.idx_first = 0;
        self.idx_last = self.data.len();
    }
}

impl<'a> Index<usize> for Bytes<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.idx_first + index]
    }
}

impl<'a> Index<Range<usize>> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[
            self.idx_first+index.start..
            index.end
        ]
    }
}

impl<'a> Index<RangeFrom<usize>> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.data[
            self.idx_first+index.start..
            self.idx_last
        ]
    }
}

impl<'a> Index<RangeTo<usize>> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.data[
            self.idx_first..
            index.end
        ]
    }
}

impl<'a> Index<RangeFull> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.data[
            self.idx_first..
            self.idx_last
        ]
    }
}

impl<'a> Index<RangeInclusive<usize>> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.data[
            self.idx_first+index.start()..=
            *index.end()
        ]
    }
}



#[macro_export]
macro_rules! slice_to_unsigned {
    ($slice:expr, $unsigned:ty) => {
        {
            // let _assert: u8 = $slice[0];

            let mut count: $unsigned = 0;
            for byte in $slice.iter() {
                count <<= 8;
                count += *byte as $unsigned;
            }

            count
        }
    };
}