use anyhow::{Result, anyhow};
use std::{fmt::{Debug, Write}, ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo}};





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




#[derive(Default)]
pub struct Bytes<'a> {
    idx_first: usize, 
    idx_last: usize, 
    data: &'a [u8],

    no_print: bool,
}

impl<'a> Bytes<'a>{
    pub fn from_slice(slice: &'a[u8]) -> Self {
        Self { 
            idx_first: 0, 
            idx_last: slice.len(), 
            data: slice,

            no_print: false,
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

    pub fn no_print(&mut self) {
        self.no_print = true;
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
            self.idx_first+index.end
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
            self.idx_first+index.end
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
            self.idx_first + *index.end()
        ]
    }
}


const DEFAULT_BYTES_SLICE: [u8; 0] = [];

impl Default for &Bytes<'_> {
    fn default() -> Self {
        &Bytes {
            idx_first: 0,
            idx_last: 0,
            data: &DEFAULT_BYTES_SLICE,

            no_print: false,
        }
    }
}

impl<'a> Debug for Bytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.no_print {
            true => f.write_str("UNPRINTED BYTES OBJECT"),
            false => f.debug_tuple("Bytes")
                .field(&format_args!("Payload Length: {}", self.data.len()))
                .field(&&self[..])
                .finish()
        }
    }
}



#[macro_export]
macro_rules! slice_to_unsigned {
    ($slice:expr, $unsigned:ty) => {
        {
            let mut count: $unsigned = 0;
            for byte in $slice.iter() {
                count <<= 8;
                count += *byte as $unsigned;
            }

            count
        }
    };
}


#[derive(Default)]
pub struct Ipv4addr(pub u32);

impl Debug for Ipv4addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f1 = (self.0 >> 24) & 0xff;
        let f2 = (self.0 >> 16) & 0xff;
        let f3 = (self.0 >>  8) & 0xff;
        let f4 =  self.0 & 0xff;

        f.write_fmt(format_args!("{f1}.{f2}.{f3}.{f4} Ipv4"))
    }
}


#[derive(Default)]
pub struct Ipv6addr(pub u128);

impl Debug for Ipv6addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f1 = (self.0 >> 112) & 0xffff;
        let f2 = (self.0 >>  96) & 0xffff;
        let f3 = (self.0 >>  80) & 0xffff;
        let f4 = (self.0 >>  64) & 0xffff;
        let f5 = (self.0 >>  48) & 0xffff;
        let f6 = (self.0 >>  32) & 0xffff;
        let f7 = (self.0 >>  16) & 0xffff;
        let f8 = self.0 & 0xffff;

        f.write_fmt(format_args!("{f1:04x}:{f2:04x}:{f3:04x}:{f4:04x}:{f5:04x}:{f6:04x}:{f7:04x}:{f8:04x}"))
    }
}


#[derive(Default)]
pub struct MacAddress(u16, u32); // 48 bit address so a u16 and u32 will pack the values together

impl From<u64> for MacAddress {
    fn from(value: u64) -> Self {
        Self(
            ((value >> 32) & 0xffff) as u16,
            (value & 0xffff_ffff) as u32
        )
    }
}

impl Debug for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f1 = self.0 >> 8;
        let f2 = self.0 & 0x00ff;
        let f3 = self.1 >> 24;
        let f4 = (self.1 >> 16) & 0xff;
        let f5 = (self.1 >> 8) & 0xff;
        let f6 = self.1 & 0xff;

        f.write_fmt(format_args!("{f1:x}:{f2:x}:{f3:x}:{f4:x}:{f5:x}:{f6:x} Mac"))
    }
}



pub trait SliceToUnsigned {
    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_u32(&self) -> u32;
    fn to_u64(&self) -> u64;
    fn to_u128(&self) -> u128;
}

impl SliceToUnsigned for [u8] {
    fn to_u8(&self) -> u8 {
        assert!(self.len() <= 1, "[u8]::SliceToUnsigned::to_u8() self.len must be <= 1");

        self[0]
    }

    fn to_u16(&self) -> u16 {
        assert!(self.len() <= 2, "[u8]::SliceToUnsigned::to_u16() self.len must be <= 2");

        let mut res = 0;
        for byte in self {
            res <<= 8;
            res |= *byte as u16;
        }

        res
    }

    fn to_u32(&self) -> u32 {
        assert!(self.len() <= 4, "[u8]::SliceToUnsigned::to_u32() self.len must be <= 4");


        let mut res = 0;
        for byte in self {
            res <<= 8;
            res |= *byte as u32;
        }

        res
    }

    fn to_u64(&self) -> u64 {
        assert!(self.len() <= 8, "[u8]::SliceToUnsigned::to_u64() self.len must be <= 8");

        let mut res = 0;
        for byte in self {
            res <<= 8;
            res |= *byte as u64;
        }

        res
    }

    fn to_u128(&self) -> u128 {
        assert!(self.len() <= 16, "[u8]::SliceToUnsigned::to_u128() self.len must be <= 16");

        let mut res = 0;
        for byte in self {
            res <<= 8;
            res |= *byte as u128;
        }

        res
    }
}