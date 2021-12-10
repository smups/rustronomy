/*
    This module specifies decoding and encoding trait implementations all SADF
    supported data types. Most of these implementations use the byteorder crate
    to simplify things. All allowed data types from the SADF v1.0 standard are
    implemented.
*/

use byteorder::{BigEndian, ByteOrder};

pub trait Encode {
    fn to_bytes(&self) -> Vec<u8>;
    fn fill_buf(&self, buf: &mut Vec<u8>) {
        buf.append(&mut self.to_bytes());
    }
}

pub trait EncodeAndConsume {
    fn fill_buf(self, buf: &mut Vec<u8>);
    fn to_bytes(self) -> Vec<u8>;
}

pub trait Decode {
    fn from_bytes(data: &Vec<u8>) -> Self;
}

/*
    Encode implementations for basic types
*/

impl Encode for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 2];
        BigEndian::write_u16(&mut buf, *self);
        return buf;
    }
}

impl Encode for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 4];
        BigEndian::write_u32(&mut buf, *self);
        return buf;
    }
}

impl Encode for u64 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 8];
        BigEndian::write_u64(&mut buf, *self);
        return buf;
    }
}

impl Encode for i16 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 2];
        BigEndian::write_i16(&mut buf, *self);
        return buf;
    }
}

impl Encode for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 4];
        BigEndian::write_i32(&mut buf, *self);
        return buf;
    }
}

impl Encode for i64 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 8];
        BigEndian::write_i64(&mut buf, *self);
        return buf;
    }
}

impl Encode for f32 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 4];
        BigEndian::write_f32(&mut buf, *self);
        return buf;
    }
}

impl Encode for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0 as u8; 8];
        BigEndian::write_f64(&mut buf, *self);
        return buf;
    }
}

impl Encode for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

/*
    Decode implementations for basic types
*/

impl Decode for u16 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_u16(data)
    }
}

impl Decode for u32 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_u32(data)
    }
}

impl Decode for u64 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_u64(data)
    }
}

impl Decode for i16 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_i16(data)
    }
}

impl Decode for i32 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_i32(data)
    }
}

impl Decode for i64 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_i64(data)
    }
}

impl Decode for f32 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_f32(data)
    }
}

impl Decode for f64 {
    fn from_bytes(data: &Vec<u8>) -> Self {
        BigEndian::read_f64(data)
    }
}

impl Decode for String {
    fn from_bytes(data: &Vec<u8>) -> Self {
        // !CLONES DATA, DONT GIVE LARGE VECTOR!
        String::from_utf8(data.clone().to_vec()).unwrap()
    }
}