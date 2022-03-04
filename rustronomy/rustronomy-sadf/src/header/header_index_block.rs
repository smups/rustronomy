/*
    Copyright (C) 2021 Raúl Wolters
    
    This file is part of rustronomy-sadf.
    
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

//This line imports the encoding/decoding trait from rustronomy-core
use rustronomy_core::data_type_traits::io_utils::*;

//Length of the encoded HID
pub static HIB_LENGTH:usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderIndexBlock {
    pub id: u16,
    pub ptr: u64,
    pub length: u64,
    pub data_type: u16
}

impl EncodeAndConsume for HeaderIndexBlock {
    fn fill_buf(self, buf: &mut Vec<u8>) {
        buf.append(&mut self.id.to_bytes());
        buf.append(&mut self.ptr.to_bytes());
        buf.append(&mut self.length.to_bytes());
        buf.append(&mut self.data_type.to_bytes());
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        self.fill_buf(&mut buf);
        return buf;
    }
}

impl Decode for HeaderIndexBlock {
    fn from_bytes(data: &[u8]) -> Self {
        //First check if the slice we got is 18 bytes long
        assert!(data.len() == HIB_LENGTH);

        //Get ID from the first two bytes
        let id = u16::from_bytes(&data[0..2].to_vec());
        let ptr = u64::from_bytes(&data[2..10].to_vec());
        let length = u64::from_bytes(&data[10..18].to_vec());
        let data_type = u16::from_bytes(&data[18..20].to_vec());

        HeaderIndexBlock {
            id: id,
            ptr: ptr,
            length: length,
            data_type: data_type
        }
    }
}

impl HeaderIndexBlock {
    //Nice little printing method
    pub fn print(&self) {
        println!("HID object: {:#?}", self)
    }
}