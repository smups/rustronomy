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
    error::Error,
    rc::Rc,
    fmt::{Display, self}
};

use rustronomy_core::data_type_traits::io_utils::Encode;
use simple_error::SimpleError;

#[derive(Debug, Clone)]
pub struct KeywordRecord {
    /*  THIS STRUCT IS PART OF THE USER-FACING API
        Users should be able to create their own keyword-records, except for
        certain restricted records that specify information about the data
        contained in the data section of the HDU. Those restricted keywords
        should always be updated in unison with the data they describe.
    */
    pub keyword: Rc<String>,
    pub value: Option<String>,
    pub comment: Option<String>,
}

impl KeywordRecord {

    pub const RESTRICTED_KEYWORDS: [&'static str; 49] = [
        //KWRD{i} type keywords are not included in this list, should be
        //parsed seperately!
        "SIMPLE", "BITPIX", "NAXIS", "END", "PCOUNT", "GCOUNT", "DATE", "EXTEND",
        "BLOCKED", "CONTINUE", "BSCALE", "BZERO", "BLANK", "DATAMAX", "DATAMIN",
        "EXTNAME", "EXTVER", "EXTLEVEL", "INHERIT", "DATASUM", "CHECKSUM",
        "GROUPS", "XTENSION", "IMAGE", "TABLE", "BINTABLE", "TFIELDS", "THEAP",
        "ZIMAGE", "ZBITPIX", "ZNAXIS", "ZCPMTYPE", "ZTABLE", "ZTILELEN", "ZMASKCMP",
        "ZQUANTIZ", "ZDITHER0", "ZSIMPLE", "ZEXTEND", "ZBLOCKED", "ZTENSION",
        "ZPCOUNT", "ZGCOUNT", "ZCHECKSUM", "ZDATASUM", "FZTILELN", "FZALGOR",
        "ZTHEAP", "EPOCH"
    ];

    /*
        THE FOLLOWING FUNCS ARE PART OF THE PUBLIC API
    */

    pub fn empty() -> Self {
        KeywordRecord {
            keyword: Rc::new(String::from("")),
            value: None,
            comment: None
        }
    }

    /*
        THE FOLLOWING FUNCS ARE INTERNAL
    */

    pub(crate) fn from_string(keyword: Rc<String>, value: String, comment: Option<String>) -> Self {
        KeywordRecord{
            keyword: keyword,
            value: Some(value),
            comment: comment
        }
    }

    //Helper function for decoding. Not part of API
    pub(crate) fn decode_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
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
        let (value, comment);
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
            keyword: Rc::new(keyword),
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

    pub(crate) fn encode_fill_buff(self, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>>{

        //keep track of how long the last keyword is
        let mut one_rec_buf = Vec::new();
        
        //(1) Encode keyword and make sure it's 8 bytes long
        let keyword_len = self.keyword.len();
        self.keyword.fill_buf(&mut one_rec_buf);
        for _ in 0..(8-keyword_len) {one_rec_buf.push(b' ');}

        //(2) Encode value
        match self.value {
            None => {} //do nothing
            Some(mut val) => {
                //(2a) add the value indicator
                String::from("= ").fill_buf(&mut one_rec_buf);

                //(2b) check if the value spans multiple keywordrecords
                if val.len() < 70 {
                    val.fill_buf(&mut one_rec_buf);
                }
                else {
                    //Write first string to the record with the keyword
                    let mut remainder = val.split_off(68);
                    val += "&'";
                    val.truncate(70);
                    val.fill_buf(&mut one_rec_buf);

                    //Write to the header buffer
                    assert!(one_rec_buf.len() == 80);
                    buf.append(&mut one_rec_buf);

                    //Write the remaining part of the string to CONTINUE records
                    let mut continue_buf = Vec::new();
                    String::from("CONTINUE= '").fill_buf(&mut continue_buf);

                    while remainder.len() > 0 {
                        if continue_buf.len() == 78 {
                            continue_buf.push(b'&');
                            continue_buf.push(b"'"[0]);

                            //Write to the header buffer
                            assert!(continue_buf.len() == 80);
                            buf.append(&mut continue_buf);
                        }
                        match remainder.pop() {
                            Some(ch) => continue_buf.push(ch as u8),
                            None => {} //Loop will break
                        }
                    }

                    //Last keyword record may not have full length value
                    for _ in 0..(80 - continue_buf.len()) {continue_buf.push(b' ');}

                    //Write to the header buffer
                    assert!(continue_buf.len() == 80);
                    buf.append(&mut continue_buf);

                    //we're done
                    return Ok(());
                }
            }
        }

        //(3) Encode comment
        match self.comment {
            None => {}, //do nothing
            Some(com) => {
                String::from("/").fill_buf(&mut one_rec_buf);
                com.fill_buf(&mut one_rec_buf);
            }
        }

        //(4) Make sure the keywordrecord is 80 bytes long
        for _ in 0..(80-one_rec_buf.len()) {one_rec_buf.push(b' ');}

        //write to the header buffer
        assert!(one_rec_buf.len() == 80);
        buf.append(&mut one_rec_buf);

        Ok(())
    }

}

impl Display for KeywordRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //keyword
        write!(f, "[{}] ", self.keyword)?;
        //value
        match &self.value {
            None => {}, //do nothing,
            Some(val) => write!(f, "- {val}")?
        }
        //comment
        match &self.comment {
            None => {}, //do nothing,
            Some(com) => write!(f, " //{com}")?
        }

        Ok(())
    }
}