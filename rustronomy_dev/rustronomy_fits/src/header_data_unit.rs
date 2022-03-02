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
    records: HashMap<String, String>
}

impl Header {

    pub fn from(hbs: Vec<HeaderBlock>) -> Result<Self, Box<dyn Error>> {

        //Parse the Keywordrecords to plain Key-Data pairs
        let mut record_map: HashMap<String, String> = HashMap::new();

        //Keep track of the last keyword for multi-keyword strings
        let mut last_keyword = String::from("");

        for hb in hbs {          
            for record in hb.records {
                //Deal with multi-line strings
                match record.keyword.as_str() {
                    "CONTINUE" => {
                        //This record actually belongs to the previous keyword!
                        //Should never panic... hopefully
                        let last_value = record_map.get_mut(
                            last_keyword.as_str()
                        ).unwrap();

                        //(1) remove the trailing {&'} from the previous record
                        last_value.pop();
                        last_value.pop();            

                        //(2) append the continued value, without the starting
                        // {'}
                        let mut next_value = record.value.unwrap();
                        next_value.remove(0);
                        last_value.push_str(next_value.as_str());

                        //(3) do not append keyword-record pair as seperate entry
                        continue;
                    },
                    _ => {} //do nothing
                }

                //update last keyword
                last_keyword = record.keyword.clone();

                //and add our beatiful string
                record_map.insert(
                    record.keyword,
                    match record.value {
                        Some(value) => value,
                        None => String::from("")
                    }
                );
            }
        }
        Ok(Header {records:record_map})
    }

    pub fn get_record(&self, keyword: &String) -> Option<&String> {
        self.records.get(keyword)
    }

}

#[derive(Debug)]
pub struct HeaderBlock{
    pub records: Vec<KeywordRecord>
}

impl HeaderBlock {

    pub fn decode_from_bytes(bytes: &Vec<u8>) -> Result<(Self, bool), Box<dyn Error>> {
        /*  If we're in the last headerblock of the header (denoted by the END
            keyword, then we have to set the return value of is_final to true
        */
        let mut is_final = false;


        //First, require that the Header block is 2880 blocks long
        if bytes.len() != 2880 {
            return Err(Box::new(
                SimpleError::new("Header block buffer was not 2880 bytes long.")
            ));
        }

        //Create vector of keywordrecords and return it
        let mut records: Vec<KeywordRecord> = Vec::new();
        for i in 0..36 { //36 keywords in a HeaderBlock
            //Decode
            let record = KeywordRecord::decode_from_bytes(&bytes[(i*80)..(i*80+80)])?;
            //And parse
            if record.keyword == String::from("END") {
                //This is the END keyword, which we DON'T append!
                // -> but we should set is_final to true
                is_final = true;
                break;
            }

            //Append the parsed keyword
            records.push(record);
        }

        return Ok((HeaderBlock{records:records}, is_final));
    }

    pub fn from_vec(vec: Vec<KeywordRecord>) -> Self {
        HeaderBlock{records: vec}
    }

    pub fn encode_fill_buff(self, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>>{
        for record in self.records {
            record.encode_fill_buff(buf)?;
        }
        Ok(())
    }

    pub fn encode_to_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        //Fill buf with data
        let mut buf: Vec<u8> = Vec::new();
        self.encode_fill_buff(&mut buf)?;

        //FITS files must always come in 2880 byte chunks. We fill the remaining
        //bytes with zeros to satisfy this condition
        if buf.len() < 2880 {
            buf.append(&mut vec![0u8; 2880 - buf.len()]);
            return Ok(buf);
        } else if buf.len() > 2880 {
            return Err(Box::new(SimpleError::new(
                "Error while encoding HeaderBlock: Header block contained more than 2880 bytes of data!"
            )));
        } else {
            return Ok(buf);
        }
    }

}