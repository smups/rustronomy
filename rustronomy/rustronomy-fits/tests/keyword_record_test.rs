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
    fs::{File, OpenOptions},
    io::{BufReader, Read, BufWriter, Write},
    path::PathBuf, env
};

use rustronomy_fits::{
    raw::{
        header_block::HeaderBlock,
        keyword_record::KeywordRecord
    }, 
    header::Header
};

static REAL_FILE: &str = "resources/tests/real.fits";
static FAKE_WRITE_FILE: &str = "resources/tests/write.fits";

#[test]
pub fn read_real_fits_test() {

    //Read test file
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(REAL_FILE);
    let file = File::open(path).unwrap();

    //Read the first HDU
    let mut buf = vec![0u8; 2880];
    BufReader::new(file).read_exact(&mut buf).unwrap();

    //make a header
    let (hb, _) = HeaderBlock::decode_from_bytes(&buf).unwrap();
    for entry in &hb.records {
        println!("{entry:?}");
    }

    //Make a public Header and print it
    let h = Header::from_parts(vec![hb], 0).unwrap();
    print!("{h}");
}

/*

FOR NOW: NOT IN USE
writing FITS files will come with version 0.2

#[test]
pub fn write_fake_fits_test() {

    //Create test file
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(FAKE_WRITE_FILE);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    //Create fake keywords
    let hb = HeaderBlock::from_vec(vec![
        KeywordRecord::from_str("SIMPLE  ", "T"),
        KeywordRecord::from_str("BITPIX  ", "-64"),
        KeywordRecord::from_str("NAXIS   ", "3"),
        KeywordRecord::from_str("NAXIS1  ", "100"),
        KeywordRecord::from_str("NAXIS2  ", "100"),
        KeywordRecord::from_str("NAXIS3  ", "100"),
        KeywordRecord::from_str("HISTORY ", "'this is a fake string that I had to fill with text so here you go y&'"),
        KeywordRecord::from_str("CONTINUE", "'ou get to listen to me just blabbeling about using a multi-keyword.'"),
    ]);

    //Encode
    let bytes = hb.encode_to_bytes().unwrap();

    //Write to file
    BufWriter::new(file).write(&bytes).unwrap();
}
*/