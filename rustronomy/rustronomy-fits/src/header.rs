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
    fmt::{self, Display},
    str::FromStr,
    rc::Rc
};

use chrono::{Utc, Datelike};
use simple_error::SimpleError;

use crate::raw::{
    header_block::HeaderBlock,
    raw_io::{RawFitsReader, RawFitsWriter},
    BlockSized,
    keyword_record::KeywordRecord
};

const BLOCK_SIZE: usize = crate::BLOCK_SIZE;

/*
    Public version of the header is a Simple HashMap with a wrapper around it
    for creating a Header from a FITS HDU or the other way around.
*/
#[derive(Debug, Clone)]
pub struct Header {
    records: HashMap<Rc<String>, KeywordRecord>,
    block_len: usize
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,">================================<|FITS Header|>================================")?;
        writeln!(f, ">Size in FITS blocks: {}", self.block_len)?;
        for (_, record) in &self.records {
            writeln!(f, ">  {record}")?;
        }
        writeln!(f,">===============================================================================")?;
        Ok(())
    }
}

impl Header {

    pub fn decode_header(raw: &mut RawFitsReader) -> Result<Self, Box<dyn Error>> {
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

    fn from_parts(hbs: Vec<HeaderBlock>, block_len: usize)
        -> Result<Self, Box<dyn Error>>
    {
        //Parse the Keywordrecords to plain Key-Data pairs
        let mut parsed_map: HashMap<Rc<String>, KeywordRecord> = HashMap::new();

        //Keep track of the last keyword for multi-keyword strings
        let mut last_keyword = String::from("");

        for hb in hbs {          
            for unparsed_record in hb.records {

                //Deal with multi-line strings
                match unparsed_record.keyword.as_str() {
                    "CONTINUE" => {
                        //This record actually belongs to the previous keyword!
                        //Should never panic... hopefully
                        let last_parsed = parsed_map.get_mut(
                            &last_keyword
                        ).unwrap();

                        //(1) remove the trailing {'&} from the previous record's
                        //value
                        last_parsed.value.as_mut().unwrap().pop();
                        last_parsed.value.as_mut().unwrap().pop();    

                        //(2) append the continued value
                        last_parsed.value.as_mut().unwrap()
                            .push_str(unparsed_record.value.unwrap().as_str());

                        //(3) do not append keyword-record pair as seperate entry
                        continue;
                    },
                    _ => {} //do nothing
                }

                //update last keyword
                last_keyword = (*unparsed_record.keyword).clone();

                //and add our beatiful string
                parsed_map.insert(unparsed_record.keyword.clone(), unparsed_record);
            }
        }

        Ok(Header {records: parsed_map, block_len: block_len})
    }

    pub fn encode_header(self, writer: &mut RawFitsWriter)
        -> Result<(), Box<dyn Error>>
    {
        //Buffer to write whole header in one go.
        //Also keeps track of number of bytes we wrote to the header!
        let mut buf = Vec::new();

        for (_, record) in self.records {
            record.encode_fill_buff(&mut buf)?;
        }

        //We musn´t forget to add an END keyword!
        KeywordRecord{
            keyword: Rc::new(String::from("END     ")),
            value: None,
            comment: None
        }.encode_fill_buff(&mut buf)?;

        //make sure that the size of the whole header is an integer multiple
        //of the block size. Btw we fill it with spaces not zeroes
        while buf.len() % BLOCK_SIZE != 0 {buf.push(b' ');}

        //...write the thing
        writer.write_blocks(&buf)?;

        //(R) we good
        Ok(())
    }

    pub(crate) fn new() -> Self {
        /*
            Creates new empty header.
            pub(crate) since the end user should only create empty HDU's, not
            bare headers!
        */
        let mut header = Header {
            records: HashMap::new(),
            block_len: 0 //contains nothing
        };
        //we modified the header, so we should indicate that!
        header.update_last_modified();
        return header;
    }

    /*
        Some getters for full records and single values or comments (just some
        utility funcs)
    */
    pub fn get_record(&self, keyword: &str) -> Option<&KeywordRecord> {
        self.records.get(&keyword.to_string())
    }

    pub fn get_value(&self, keyword: &str) -> Option<&String> {
        match self.records.get(&keyword.to_string()) {
            Some(record) => record.value.as_ref(),
            None => None
        }
    }

    pub fn get_comment(&self, keyword: &str) -> Option<&String> {
        match self.records.get(&keyword.to_string()) {
            Some(record) => record.comment.as_ref(),
            None => None
        }
    }

    pub(crate) fn update_last_modified(&mut self) {
        /*
            This function modifies the DATE keyword in the primary header which
            indicates the last time the HDU was modified. This function is called
            by all functions that modify the HDU (dûh).  
            
            pub(crate) because it must also be callable by the HDU, but not by
            the end-user
        */
        let now = Utc::now();
        let now_fmtd = format!(
            "{:0000}/{:00}/{:00}", //yyyy/mm/dd format as specified in standard
            now.year(),
            now.month(),
            now.day()
        );

        //create the keyword record and update the internal hashmap
        let key = Rc::new(String::from("DATE"));
        let date = KeywordRecord::from_string(
            key.clone(),
            now_fmtd,
            Some(String::from("last modified by Rustronomy-fits"))
        );
        self.records.insert(key, date);
    }

    //Helper function for parsing keyword records
    pub fn get_value_as<T>(&self, keyword: &str)
        -> Result<T, Box<dyn Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: 'static + Error
    {
        match self.get_value(&keyword.to_string()) {
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