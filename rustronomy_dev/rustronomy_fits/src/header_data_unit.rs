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

use crate::keyword_record::KeywordRecord;

use std::{
    collections::HashMap,
    error::Error
};

use simple_error::SimpleError;

/*
    A FITS file contains at least one HeaderDataUnit (HDU), but may contain
    more. This module revolves around the HeaderDataUnit struct.

    Header data units consist of a number of 2880 byte Header blocks, followed
    by a number of 2880 data blocks (optionally).
*/


/*
    Public version of the header is a Simple HashMap with a wrapper around it
    for creating a Header from a FITS HDU or the other way around.
*/
#[derive(Debug)]
pub struct Header {
    records: HashMap<String, KeywordRecord>,
    is_primary: bool
}

impl Header {

    pub fn new(is_primary: bool) -> Self {
        Header {
            records: HashMap::new(),
            is_primary: is_primary
        }
    }

    pub fn get_record(&self, keyword: &String) -> Option<&KeywordRecord> {
        self.records.get(keyword)
    }

    pub fn is_primary_header(&self) -> bool {
        self.is_primary
    }

}

#[derive(Debug)]
pub struct HeaderBlock{
    pub records: Vec<KeywordRecord>
}

impl HeaderBlock {

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        
        //First, require that the Header block is 2880 blocks long
        if bytes.len() != 2880 {
            return Err(Box::new(
                SimpleError::new("Header block buffer was not 2880 bytes long.")
            ));
        }

        //Create vector of keywordrecords and return it
        let mut records: Vec<KeywordRecord> = Vec::new();
        for i in 0..(bytes.len() / 80) {
            //Decode
            let record = KeywordRecord::decode_from_bytes(&bytes[(i*80)..(i*80+80)])?;
            //And parse
            if record.keyword == String::from("END") {
                //This is the final keyword
                break;
            }

            //Append the parsed keyword
            records.push(record);
        }

        return Ok(HeaderBlock{records:records});
    }

}