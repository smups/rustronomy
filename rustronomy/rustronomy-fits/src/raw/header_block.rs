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

use std::error::Error;

use simple_error::SimpleError;

use super::keyword_record::KeywordRecord;

#[derive(Debug)]
pub(crate) struct HeaderBlock{
    /*  NOT PART OF USER-FACING API
        This struct and its implementations are used in decoding/encoding
        headers and should not be used directly by the user
    */
    pub(crate) records: Vec<KeywordRecord>
}

impl HeaderBlock {

    pub(crate) fn decode_from_bytes(bytes: &[u8]) -> Result<(Self, bool), Box<dyn Error>> {

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
            if *record.keyword == String::from("END") {
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

    pub(crate) fn encode_fill_buff(self, buf: &mut Vec<u8>) -> Result<(), Box<dyn Error>>{
        for record in self.records {
            record.encode_fill_buff(buf)?;
        }
        Ok(())
    }

    pub(crate) fn encode_to_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
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