/*
    Copyright (C) 2021 Raúl Wolters
    
    This file is part of rustronomy-core.
    
    rustronomy is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    
    rustronomy is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    
    You should have received a copy of the GNU General Public License
    along with rustronomy.  If not, see <http://www.gnu.org/licenses/>.
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
    fn from_bytes(data: &[u8]) -> Self;
}

/*
    Encode implementations for basic types
*/

impl Encode for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

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

impl Decode for u8 {
    fn from_bytes(data: &[u8]) -> Self {data[0]}
}

impl Decode for u16 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_u16(data)
    }
}

impl Decode for u32 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_u32(data)
    }
}

impl Decode for u64 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_u64(data)
    }
}

impl Decode for i16 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_i16(data)
    }
}

impl Decode for i32 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_i32(data)
    }
}

impl Decode for i64 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_i64(data)
    }
}

impl Decode for f32 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_f32(data)
    }
}

impl Decode for f64 {
    fn from_bytes(data: &[u8]) -> Self {
        BigEndian::read_f64(data)
    }
}

impl Decode for String {
    fn from_bytes(data: &[u8]) -> Self {
        // !CLONES DATA, DONT GIVE LARGE VECTOR!
        String::from_utf8(data.clone().to_vec()).unwrap()
    }
}