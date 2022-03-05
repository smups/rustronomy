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

/*
    This is the public FITS interface
*/

use std::{
    path::Path,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    str::FromStr
};

use simple_error::SimpleError;

use crate::{
    header_data_unit::Header,
    extensions::{Xtension, image::{Image, ImgParser}},
    bitpix::Bitpix,
    raw::{
        header_block::HeaderBlock,
        raw_fits::{RawFitsReader, RawFitsWriter}
    }
};

#[derive(Debug)]
pub struct Fits {
    pub header: Header,
    data: Vec<Box<dyn Xtension>>,
    reader: Option<RawFitsReader>,
    writer: Option<RawFitsWriter>
}

impl Fits {

    pub fn open(path: &Path) -> Result<Self, Box<dyn Error>> {

        //(1) Read file as raw fits
        let mut raw = RawFitsReader::new(path)?;

        //(2) Read Header
        let (mut hbs, mut end) = (Vec::<HeaderBlock>::new(), false);
        let mut hb_buf = vec![0u8; 2880]; //1 FITS block

        while !end {
            //Read the next headerblock (2880 bytes) and decode it!
            raw.read_blocks(&mut hb_buf)?;
            let (hb, finished) = HeaderBlock::decode_from_bytes(&hb_buf)?;

            //Append the keywords that we found
            hbs.push(hb);

            //Update end value
            end = finished;
        }

        //Aaand finally we write the contents into a Header
        let header = Header::from(hbs)?;

        Ok(Fits {
            header: header,
            data: Vec::new(),
            reader: Some(raw),
            writer: None
        })
    }

    //Helper function for parsing keyword records
    fn parse_keyword_record<T>(&self, keyword: &str)
        -> Result<T, Box<dyn Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: 'static + Error
    {
        match self.header.get_record(keyword) {
            None => Err(Box::new(SimpleError::new(
                format!("Error while looking for keyword: keyword [{}] not present in FITS file!", keyword)
            ))),
            Some(val) => {
                //Remove the comment
                let unparsed = val.split("/").collect::<Vec<_>>()[0].trim();
                Ok(str::parse::<T>(unparsed)?)
            }
        }
    }

    pub fn read_img(&mut self) -> Result<(), Box<dyn Error>> {

        //Let's start by getting the number of axes
        let naxis: usize = self.parse_keyword_record("NAXIS")?;

        //And now the lengths
        let mut axes: Vec<usize> = Vec::new();
        for i in 1..=naxis {
            axes.push(self.parse_keyword_record(format!("NAXIS{i}").as_str())?);
        }

        //And the datatype ofc
        let bitpix = Bitpix::from_code(
            &self.parse_keyword_record::<isize>("BITPIX")?
        )?;

        //Now the scary part: reading the image
        let img = ImgParser::decode_img(
            self.reader.as_mut().ok_or(Box::new(SimpleError::new(
                "Error while parsing image: handle to FITS file stream was lost"
            )))?,
            &axes,
            bitpix
        )?;

        //Append the image to the fits file
        self.data.push(img);
        
        Ok(())
    }

}