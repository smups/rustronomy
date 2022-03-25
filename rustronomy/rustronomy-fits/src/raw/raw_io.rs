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
    io::{Read, Write, self},
    fs::{File, Metadata},
    error::Error,
    path::Path
};

use simple_error::SimpleError;

//Get block size from root
const BLOCK_SIZE: usize = crate::BLOCK_SIZE;

/*
    RawFitsReader and RawFitsWriter are fields of the Fits struct which is part
    of the public API. Therefore, the structs must be public themselves, even
    though none of their methods are public.

    NOTE: the file_meta field for file metadata *is* publicly accesible!
*/

#[derive(Debug)]
pub struct RawFitsReader {
    pub file_meta: Metadata,
    block_index: usize,
    n_fits_blocks: usize,
    reader_handle: File
}

impl RawFitsReader {

    pub(crate) fn new(path: &Path) -> Result<Self, Box<dyn Error>> {

        //(1) Open the file
        let f = File::open(path)?;
        
        //(2) Get metadata -> number of fits blocks
        let meta = f.metadata()?;
        
        if meta.len() as usize % BLOCK_SIZE != 0 {
            //Throw an error for files that are not integer multiples of 2880
            return Err(Box::new(SimpleError::new(
                "Error while opening FITS file: file does not conform to FITS standard (not cleanly divisible into FITS blocks)"
            )));
        }
        let n_blocks = meta.len() as usize / BLOCK_SIZE;

        //Return file as raw FITS
        Ok(RawFitsReader {
            file_meta:meta,
            block_index: 0,
            n_fits_blocks: n_blocks,
            reader_handle: f
        })
    }

    pub(crate) fn read_blocks(&mut self, buffer: &mut [u8])
        -> Result<usize, Box<dyn Error>>
    {
        //(1) Calculate how many header blocks we have to read
        let n_blocks = buffer.len() / BLOCK_SIZE;

        //(2) Check if the buffer is an integer multiple of a FITS block
        if n_blocks * BLOCK_SIZE != buffer.len() {
            return Err(Box::new(SimpleError::new(
                "Error while reading from FITS file: supplied buffer not an integer multiple of FITS blocks"
            )));
        }
        
        //(3) Check if the number of header blocks we need to read does not exceed
        //the number of header blocks still left in the file
        if n_blocks > (self.n_fits_blocks - self.block_index) {
            return Err(Box::new(SimpleError::new(
                "Error while reading from FITS file: trying to read more FITS blocks than the FITS file actually contains!"
            )));
        }

        //(4) Read the data (panic if this fails, since it fucks up the indexing)
        self.reader_handle.read_exact(buffer).unwrap();

        //(5) Update the block index
        self.block_index += n_blocks;

        Ok(n_blocks) //return the number of blocks read
    }

    pub(crate) fn get_block_len(&self) -> usize {self.n_fits_blocks}
    pub(crate) fn get_block_index(&self) -> usize {self.block_index}
}

#[derive(Debug)]
pub struct RawFitsWriter{
    pub file_meta: Metadata,
    writer_handle: File
}

impl RawFitsWriter {

    pub(crate) fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        //(1) Open the file if it exists, create it if it doesn't
        let out = File::create(path)?;

        //(2) Create the required derivatives
        let meta = out.metadata()?;

        //(R)
        Ok(RawFitsWriter{file_meta: meta, writer_handle: out})
    }

    pub(crate) fn write_blocks(&mut self, buffer: &[u8])
        -> Result<usize, Box<dyn Error>>
    {
        //(1) Check if the buffer is an integer number of FITS blocks
        if buffer.len() % BLOCK_SIZE != 0 {
            return Err(Box::new(SimpleError::new(
                "Error while writing FITS file: length of buffer not an integer multiple of the FITS block size (2880B)"
            )));
        }

        //(2) Write the thing
        self.writer_handle.write_all(buffer)?;

        //(R) the number of FITS blocks that we wrote
        Ok(buffer.len() / BLOCK_SIZE)
    }

    pub(crate) fn flush(&mut self) -> io::Result<()> {Ok(self.writer_handle.flush()?)}

}