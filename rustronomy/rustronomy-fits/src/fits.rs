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

use core::fmt;
use std::{path::Path, error::Error, fmt::{Display, Formatter}};

use crate::{
    raw::{
        raw_io::{RawFitsReader, RawFitsWriter},
        BlockSized
    },
    header_data_unit::HeaderDataUnit
};

#[derive(Debug)]
pub struct Fits {
    hdus: Vec<HeaderDataUnit>,
    reader: Option<RawFitsReader>,
    writer: Option<RawFitsWriter>
}

impl Fits {

    pub fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        //(1) Construct a RawFitsReader
        let mut reader = RawFitsReader::new(path)?;
        
        //(2) Read HDU's from the fits file until it is empty
        let mut hdus = Vec::new();
        while reader.get_block_index() < reader.get_block_len() {
            hdus.push(HeaderDataUnit::decode_hdu(&mut reader)?)
        }

        //File is empty, we don't need the reader anymore!
        // (3) return the completed file
        Ok(Fits {hdus: hdus, reader: None, writer: None})
    }

    pub fn write(self, path: &Path) -> Result<(), Box<dyn Error>> {
        //(1) Construct a RawFitsWriter
        let mut writer = RawFitsWriter::new(path)?;

        //(2) Write all HDU's to this thing
        for hdu in self.hdus {hdu.encode_hdu(&mut writer)?;}

        //(3) Flush writer and close the file
        writer.flush()?;

        //(R) done
        Ok(())
    }

    pub fn get_hdu(&self, index: usize) -> Option<&HeaderDataUnit> {
        self.hdus.get(index)
    }

    pub fn remove_hdu(&mut self, index: usize) -> Option<HeaderDataUnit> {
        if self.hdus.len() < index {return None}
        Some(self.hdus.remove(index))
    }

}

impl BlockSized for Fits {
    fn get_block_len(&self) -> usize {
        (&self.hdus).iter().fold(0, |sum, hdu| sum + hdu.get_block_len())
    }
}

impl Display for Fits {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f,"\n>=================================<|FITS File|>=================================")?;
        writeln!(f, ">Total File size in FITS blocks: {}", self.get_block_len())?;
        writeln!(f, ">Number of Header-Data-Units: {}", self.hdus.len())?;
        writeln!(f, ">Contents:")?;
        for (index, hdu) in (&self.hdus).iter().enumerate() {
            writeln!(f, ">-------------------------------------------------------------------------------\n>  [HDU #{}]", index)?;
            writeln!(f, ">  Total HDU size in FITS blocks: {}", hdu.get_block_len())?;
            writeln!(f, ">    {}", hdu.pretty_print_header())?;
            writeln!(f, ">    {}", hdu.pretty_print_data())?;
        }
        writeln!(f,">===============================================================================\n")?;
        Ok(())
    }
}

impl Clone for Fits {
    fn clone(&self) -> Self {
        Self{ hdus: self.hdus.clone(), reader: None, writer: None }
    }
}