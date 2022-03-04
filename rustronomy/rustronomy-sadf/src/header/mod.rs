/*
    Copyright (C) 2021 Raúl Wolters
    
    This file is part of rustronomy-sadf.
    
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

//Module structure
pub mod header_index_block;

//Imports
use std::fmt;

use self::header_index_block::{HeaderIndexBlock, HIB_LENGTH};
use rustronomy_core::data_type_traits::io_utils::{EncodeAndConsume, Encode, Decode};

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    version: u16,
    n_hibs: u16,
    hibs: Vec<HeaderIndexBlock>
}

impl EncodeAndConsume for Header {
    fn fill_buf(self, buf: &mut Vec<u8>) {
        //First we do the header fields
        self.version.fill_buf(buf);
        self.n_hibs.fill_buf(buf);

        //Append all the HIB's
        for hib in self.hibs.into_iter(){
            /*
                into_iter omdat de hibs de eigenaar is van hib, niet de iter.
                Met into_iter wordt het eigendom van hib doorgegeven aan de 
                iter, en kunnen we hib consumeren.
            */
            hib.fill_buf(buf);
        }

    }

    fn to_bytes(self) -> Vec<u8> {
        //Standaard implementatie
        let mut bfr:Vec<u8> = Vec::new();
        self.fill_buf(&mut bfr);
        return bfr;
    }
}

impl Decode for Header {
    fn from_bytes(data: &[u8]) -> Self {
        //First two bytes contain version number
        let version = u16::from_bytes(&data[0..2].to_vec());

        //Panic if an unsupported version is found
        assert!(version <= super::SADF_VERSION,
            "{}", format!("Error: found SADF file built against SADF v{}. Rustronomy {} supports up to SADF v{}",
            version,
            super::RUSTRONOMY_SADF_VERSION,
            super::SADF_VERSION
            )
        );

        //Bytes 2 and 3 contain the number of HIBS
        let n_hibs = u16::from_bytes(&data[2..4].to_vec());

        //Vector with HeaderIndexBlocks that we should fill
        let mut hibs: Vec<HeaderIndexBlock> = Vec::new();

        for i in 0..n_hibs.into() {
            let start = 4 + HIB_LENGTH * i; //+4 for offset of first two fields
            let stop = start + HIB_LENGTH;
            let hib = HeaderIndexBlock::from_bytes(&data[start..stop].to_vec());
            hibs.push(hib);
        }

        let header = Header {
            n_hibs: n_hibs,
            hibs: hibs,
            version: version
        };

        //Decoding must fail for invalid header
        header.check_header();

        return header;
    }
}

impl Header {
    pub fn new() -> Header {
        Header {
            n_hibs: 0 as u16,
            hibs: Vec::new(),
            version: super::SADF_VERSION
        }
    }

    pub fn add_hib(&mut self, hib: HeaderIndexBlock) {
        self.hibs.push(hib);
        self.n_hibs += 1;
    }

    fn check_header(&self) {
        //Panics if the header is invalid
        
        //First condition is that the header must have as many index entries as
        //is specified in the n_hibs field. If not, something went wrong while
        //de/en-coding
        assert!(self.hibs.len() == self.n_hibs as usize, "Header decoding error: specified header index table length did not match up with the actual number of index entries!");

        //TODO
        //Next, we check that the datablocks specified in the header don´t over-
        //lap.
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data_blocks = String::new();

        for hib in self.hibs.iter() {
            data_blocks = format!("{}{}", data_blocks, format!(
                "•Data Block #{} - {} with size {}\n",
                hib.id,
                format_data_type(hib.data_type),
                format_size(hib.length as f64)
           ));
        }

        fn format_size(size: f64) -> String {
            if size < 1e3 {format!("{:.0}B", size)}
            else if size < 1e6 {format!("{:.2}kB", size / 1e3)}
            else if size < 1e9 {format!("{:.2}MB", size / 1e6)}
            else if size < 1e12 {format!("{:.2}GB", size / 1e9)}
            else {format!("{:.2}PB", size / 1e12)}
        }

        fn format_data_type(data_type: u16) -> String {
            match data_type {
                0x0000 => format!("text"),
                0x0001..=0x000f => format!("{}-dimensional array", data_type),
                0x00f0 => format!("table"),
                0xc000..=0xcfff => format!("user-defined"),
                0xffff => format!("metadata"),
                _ => format!("UNDEFINED")
            }
        }

        write!(f, "[SADF file structure]\nSADF standard version: v{}\nNumber of data blocks: {}\n{}",
            self.version, //Oops, placeholder
            self.n_hibs,
            data_blocks)
    }
    
}