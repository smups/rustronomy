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

use core::fmt;
use std::{error::Error, fmt::Display, borrow::Cow};

use simple_error::SimpleError;

use crate::{
    header::Header,
    extensions::{Extension, image::ImgParser},
    raw::{raw_io::{RawFitsReader, RawFitsWriter}, BlockSized},
    bitpix::Bitpix
};

#[derive(Debug, Clone)]
pub struct HeaderDataUnit {
    header: Header,
    data: Option<Extension>
}

impl HeaderDataUnit {

    /*
        INTERNAL CODE
    */

    pub(crate) fn decode_hdu(raw: &mut RawFitsReader) -> Result<Self, Box<dyn Error>> {
        
        //(1) Read the header
        let header = Header::decode_header(raw)?;

        //(2) Read data, if there is any
        let extension = match &header.get_value("XTENSION") {
            None => {
                /*  (2a)
                    This is the primary header (or there is simply no data in
                    this hdu). This means that this HDU may contain random
                    groups. Random groups and emtpy arrays have the NAXIS 
                    keyword set to zero.
                */
                if header.get_value_as::<usize>("NAXIS")? == 0 {
                    //For now I'll just return None rather than implement random
                    //groups
                    None
                } else {
                    //Image
                    Some(Self::read_img(raw, &header)?)
                }
            } Some(extension_type) => {
                /*  (2b)
                    This is not a primary header, but the header of an extension
                    hdu.
                */
                match extension_type.as_str() {
                    "'IMAGE   '" => Some(Self::read_img(raw, &header)?),
                    kw @ "'TABLE   '" => Err(Self::not_impl(kw))?,
                    kw @ "'BINTABLE'" => Err(Self::not_impl(kw))?,
                    kw => Err(Box::new(SimpleError::new(
                        format!("Error while constructing HDU: {kw} is not a valid extension type!")
                    )))?
                }
            }
        };
        
        //(3) return complete HDU
        Ok(HeaderDataUnit {header: header, data: extension})
    }

    fn read_img(raw: &mut RawFitsReader, header: &Header)
        -> Result<Extension, Box<dyn Error>>
    {
        //Let's start by getting the number of axes from the NAXIS keyword
        let naxis: usize = header.get_value_as("NAXIS")?;

        //Axis sizes are encoded in the NAXIS{i} keywords
        let mut axes: Vec<usize> = Vec::new();
        for i in 1..=naxis {
            axes.push(header.get_value_as(format!("NAXIS{i}").as_str())?);
        }

        //Datatype is encoded in the BITPIX keyword
        let bitpix = Bitpix::from_code(&header.get_value_as("BITPIX")?)?;

        //Now do the actual decoding of the image:
        Ok(ImgParser::decode_img(raw, &axes, bitpix)?)
    }

    pub(crate) fn encode_hdu(self, writer: &mut RawFitsWriter)
        -> Result<(), Box<dyn Error>>
    {
        //(1) Write header
        self.header.encode_header(writer)?;

        //(2) If we have data, write the data
        match self.data {
            Some(data) => data.write_to_buffer(writer)?,
            _ => {}//no data, do nothing
        }

        //(R) ok
        Ok(())
    }

    fn not_impl(keyword: &str) -> Box<SimpleError> {
        Box::new(SimpleError::new(
            format!("Error while constructing HDU: extension {keyword} not implemented yet!")
        ))
    }

    /*
        USER-FACING API STARTS HERE    
    */

    //Some simple getters
    pub fn get_header(&self) -> &Header {&self.header}
    pub fn get_data(&self) -> Option<&Extension> {self.data.as_ref()}

    //Destructs HDU into parts
    pub fn to_parts(self) -> (Header, Option<Extension>) {
        (self.header, self.data)
    }

    pub fn pretty_print_header(&self) -> String {
        format!("[Header] - size: {}, #records: {}",
            self.header.get_block_len(), self.header.get_num_records()
        )
    }

    pub fn pretty_print_data(&self) -> String {
        let data_string: Cow<str> = match &self.data {
            None => "(NO_DATA)".into(),
            Some(data) => format!("{data}").into()
        };
        format!("[Data] - {data_string}")
    }
}

impl BlockSized for HeaderDataUnit {
    fn get_block_len(&self) -> usize {
        self.header.get_block_len() + match &self.data {
            None => 0,
            Some(data) => data.get_block_len()
        }
    }
}

impl Display for HeaderDataUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print_header())?;
        write!(f, "{}", self.pretty_print_data())?;
        Ok(())
    }
}