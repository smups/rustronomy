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
    fmt::{self, Display}
};

use crate::raw::header_block::HeaderBlock;

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

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,">================================<|FITS Header|>================================")?;
        for (k,v) in &self.records {
            writeln!(f, ">  [{}] : {}", k, v)?;
        }
        writeln!(f,">===============================================================================")?;
        Ok(())
    }
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

    pub fn get_record(&self, keyword: &str) -> Option<&String> {
        self.records.get(keyword)
    }

}