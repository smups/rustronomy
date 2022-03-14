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
    collections::HashMap,
    error::Error,
    fmt::{self, Display}, str::FromStr
};

use simple_error::SimpleError;

use crate::raw::{header_block::HeaderBlock, raw_fits::RawFitsReader, BlockSized};

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
    records: HashMap<String, (String, String)>,
    block_len: usize
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,">================================<|FITS Header|>================================")?;
        writeln!(f, ">Size in FITS blocks: {}", self.block_len)?;
        for (key,(val, com)) in &self.records {
            if key.as_str() == "" {
                if com.as_str() == "" {
                    writeln!(f, ">")?;
                } else {
                    writeln!(f, ">  //{com}")?;
                }
            } else {
                if com.as_str() == "" {
                    writeln!(f, ">  [{key}] - {val}")?;
                } else {
                    writeln!(f, ">  [{key}] - {val} //{com}")?;
                }
            }
        }
        writeln!(f,">===============================================================================")?;
        Ok(())
    }
}

impl Header {

    pub fn from_raw(raw: &mut RawFitsReader) -> Result<Self, Box<dyn Error>> {
        /*  Setup:
            We'll keep reading headerblocks (= FITS blocks) untill we encounter
            the END keyword. We'll also have to keep track of the block size of
            the entire header.   
        */
        let (mut hbs, mut end) = (Vec::<HeaderBlock>::new(), false);
        let mut hb_buf = vec![0u8; 2880]; //1 FITS block
        let mut block_len = 0usize;

        while !end {
            //Read the next headerblock (2880 bytes) and decode it!
            block_len += raw.read_blocks(&mut hb_buf)?;
            let (hb, finished) = HeaderBlock::decode_from_bytes(&hb_buf)?;

            //Append the keywords that we found
            hbs.push(hb);

            //Update end value
            end = finished;
        }

        Ok(Self::from_parts(hbs, block_len)?)
    }

    pub fn from_parts(hbs: Vec<HeaderBlock>, block_len: usize)
        -> Result<Self, Box<dyn Error>>
    {
        //Parse the Keywordrecords to plain Key-Data pairs
        let mut record_map: HashMap<String, (String, String)> = HashMap::new();

        //Keep track of the last keyword for multi-keyword strings
        let mut last_keyword = String::from("");

        for hb in hbs {          
            for record in hb.records {

                //Deal with multi-line strings
                match record.keyword.as_str() {
                    "CONTINUE" => {
                        //This record actually belongs to the previous keyword!
                        //Should never panic... hopefully
                        let (last_value, last_comment) = record_map.get_mut(
                            last_keyword.as_str()
                        ).unwrap();

                        //(1) remove the trailing {&} from the previous record
                        last_value.pop();           

                        //(2) append the continued value
                        last_value.push_str(record.value.unwrap().as_str());

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
                    ( //Value is a tuple containing the value and comment
                        //associated with the keyword!
                        match record.value {
                        Some(value) => value,
                        None => String::from("")
                        }, match record.comment {
                        Some(comment) => comment,
                        None => String::from("")
                        }
                    )
                );
            }
        }

        Ok(Header {records: record_map, block_len: block_len})
    }

    /*
        Some getters for full records and single values or comments (just some
        utility funcs)
    */
    pub fn get_record(&self, keyword: &str) -> Option<&(String, String)> {
        self.records.get(keyword)
    }

    pub fn get_value(&self, keyword: &str) -> Option<&String> {
        match self.records.get(keyword) {
            Some((val, _com)) => Some(val),
            None => None
        }
    }

    pub fn get_comment(&self, keyword: &str) -> Option<&String> {
        match self.records.get(keyword) {
            Some((_val, com)) => Some(com),
            None => None
        }
    }

    //Helper function for parsing keyword records
    pub fn get_value_as<T>(&self, keyword: &str)
        -> Result<T, Box<dyn Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: 'static + Error
    {
        match self.get_value(keyword) {
            None => Err(Box::new(SimpleError::new(
                format!("Error while looking for keyword: keyword [{}] not present in FITS file!", keyword)
            ))),
            Some(val) => Ok(str::parse::<T>(val)?)
        }
    }

    pub fn get_num_records(&self) -> usize {
        self.records.len()
    }
}

impl BlockSized for Header {
    fn get_block_len(&self) -> usize {
        self.block_len
    }
}