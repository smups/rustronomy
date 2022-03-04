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

//Imports
use std::fmt;

use crate::data_types::DataType;
use super::DataBlock;

use rustronomy_core::data_type_traits::io_utils::{
    EncodeAndConsume,
    Decode, Encode
};

pub struct TextDB {
    db_ty: u16,
    db_id: u16,
    data_type: DataType,
    text: String
}

impl Decode for TextDB {
    fn from_bytes(data: &[u8]) -> Self {
        //Decode all the fields in order
        let db_ty = 0x0000; //Code for text db
        let db_id = u16::from_bytes(&data[2..4].to_vec());
        let data_type = DataType::from_bytes(&data[4..6].to_vec());
        //Panic if the data type is not a string
        assert!(data_type == DataType::Utf8 || data_type == DataType::Utf16);
        let text = String::from_bytes(&data[6..data.len()].to_vec());

        TextDB {
            db_ty: db_ty,
            db_id: db_id,
            data_type: data_type,
            text: text
        }
    }
}

impl EncodeAndConsume for TextDB {
    fn fill_buf(self, buf: &mut Vec<u8>) {
        //Encode all the fields in order
        self.db_ty.fill_buf(buf);
        self.db_id.fill_buf(buf);
        self.data_type.fill_buf(buf);
        self.text.fill_buf(buf);
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        self.fill_buf(&mut buf);
        return buf;
    }
}

impl DataBlock for TextDB {

}

impl TextDB {
    pub fn from_text(text: String, db_id: u16) -> (Self, u64) {
        let len = text.len() + 6;
        let txt = TextDB {
            db_ty: 0x0000,
            db_id: db_id,
            data_type: DataType::Utf8,
            text: text
        };
        return (txt, len as u64);
    }
}

impl fmt::Display for TextDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "[Text Data Block]\nlength: {}\nencoding: {}\ntext ->\n{}",
            self.text.len() + 4,
            match self.data_type {
                DataType::Utf8 => "utf-8",
                DataType::Utf16 => "utf-16",
                _ => "CORRUPTED"
            },
            self.text
        )
    }
}