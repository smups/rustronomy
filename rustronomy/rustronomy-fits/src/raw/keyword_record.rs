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

use rustronomy_core::data_type_traits::io_utils::Encode;
use simple_error::SimpleError;

#[derive(Debug, Clone)]
pub struct KeywordRecord {
    pub keyword: String,
    pub value: Option<String>,
    pub comment: Option<String>,
}

impl KeywordRecord {

    //returns empty record
    pub fn empty() -> Self {
        KeywordRecord {
            keyword: String::from(""),
            value: None,
            comment: None
        }
    }

    pub fn from_string(keyword: String, value: String, comment: Option<String>) -> Self {
        KeywordRecord{
            keyword: keyword,
            value: Some(value),
            comment: comment
        }
    }

    pub fn decode_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        //Make sure that we got 80 bytes:
        if bytes.len() != 80 {
            return Err(Box::new(
                SimpleError::new("Keyword record buffer was not 80 bytes long.")
            ));
        }

        //value and comment vlags
        let mut has_val: bool;
        let has_com: bool;

        //Decode into keyword and record
        let keyword = String::from(str::from_utf8(&bytes[0..8])?.trim());
        has_val = match str::from_utf8(&bytes[8..10])? {
            "= " => true,
            _ => false
        };
        let record = String::from(str::from_utf8(&bytes[10..80])?.trim());

        //Keyword and value should be valid ASCII
        if !keyword.is_ascii() || !record.is_ascii() {
            return Err(Box::new(
                SimpleError::new("Keyword-valule pair contains illegal characters")
            ));
        }

        //Split record into value and comment
        let (mut value, comment);
        let split= record.split("/").collect::<Vec<_>>();
        
        match split.len() {
              1 => {
                //There was no comment in the record
                value = String::from(split[0].trim());
                comment = String::from("");
                has_com = false;
            } 2 => {
                //There was a comment in the record, there MAY have been a value
                value = String::from(split[0].trim());
                comment = String::from(split[1].trim());

                //Update value and comment flags
                has_com = true;
                if value.len() == 0 {has_val = false;}
            } _ => {
                //This makes no sense, just set the value the entire string
                value = record;
                comment = String::from("");
                has_com = false;
            }
        }

        Ok( KeywordRecord {
            keyword: keyword,
            value: match has_val {
                false => None,
                true => Some(value)
            },
            comment: match has_com {
                false => None,
                true => Some(comment)
            }
        })
    }

    pub fn encode_fill_buff(self, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>>{
        
        //(1) Encode keyword and make sure it's 8 bytes long
        let keyword_len = self.keyword.len();
        self.keyword.fill_buf(buf);
        for _ in 0..(8-keyword_len) {buf.push(0);}

        //(2) Encode value
        match self.value {
            None => {} //do nothing
            Some(val) => {
                //(2a) add the value indicator
                String::from("= ").fill_buf(buf);

                //(2b) check if the value spans multiple keywordrecords
                if val.len() < 70 {val.fill_buf(buf);}
                else {
                    //Write first string to the record with the keyword\

                }
            }
        }

        Ok(())
    }

}