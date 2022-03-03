/*
    Copyright (C) 2022 Raúl Wolters
    
    This file is part of rustronomy-fits.
    
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

use std::{
    str,
    error::Error
};

use simple_error::SimpleError;

#[derive(Debug, Clone)]
pub struct KeywordRecord {
    pub keyword: String,
    pub value: Option<String>
}

impl KeywordRecord {

    //returns empty record
    pub fn empty() -> Self {
        KeywordRecord {
            keyword: String::from(""),
            value: None
        }
    }

    pub fn from_string(keyword: String, value: String) -> Self {
        KeywordRecord{
            keyword: keyword,
            value: Some(value)
        }
    }

    pub fn from_str(keyword: &str, value: &str) -> Self {
        KeywordRecord {
            keyword: String::from(keyword),
            value: Some(String::from(value))
        }
    }

    pub fn decode_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        //Make sure that we got 80 bytes:
        if bytes.len() != 80 {
            return Err(Box::new(
                SimpleError::new("Keyword record buffer was not 80 bytes long.")
            ));
        }

        //Decode into keyword and record
        let keyword = String::from(str::from_utf8(&bytes[0..8])?);
        let has_val = match str::from_utf8(&bytes[8..10])? {
            "= " => true,
            _ => false
        };
        let value = String::from(str::from_utf8(&bytes[10..80])?);

        //Keyword and value should be valid ASCII
        if !keyword.is_ascii() || !value.is_ascii() {
            return Err(Box::new(
                SimpleError::new("Keyword-valule pair contains illegal characters")
            ));
        }

        Ok( KeywordRecord {
            keyword: String::from(keyword.trim()),
            value: match has_val {
                false => None,
                true => Some(String::from(value.trim()))
            }
        })
    }

    pub fn encode_fill_buff(self, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>>{
        let mut string_buf = vec![b' '; 80];

        //put the keyword in the temp.buf
        for i in 0..self.keyword.len() {
            string_buf[i] = self.keyword.as_bytes()[i];
        }

        //put the value in the temp. buf if there is one
        match self.value {
            None => {}, //do nothing
            Some(val) => {
                //We have a value, so bytes 8 and 9 should be '= '
                string_buf[8] = b'=';
                for i in 0..val.len() {
                    string_buf[i+10] = val.as_bytes()[i];
                }
            }
        }

        //Append the result to the buffer
        buf.append(&mut string_buf);
        Ok(())
    }

}