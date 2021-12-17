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

//Imports
use std::{
    env,
    fs::File,
    io::Write
};

use crate::{
    data_blocks::text::TextDB,
    header::{
        Header,
        header_index_block::HeaderIndexBlock
    }
};

use rustronomy_core::data_type_traits::io_utils::{
    EncodeAndConsume,
    Decode
};


#[test]
fn test_db_text_io() {
    //Generate a text db with the lipsum module
    let txt = lipsum::lipsum(1000);
    let (text_db1, len) = TextDB::from_text(txt, 1);
    
    //Encode the text_db thing
    let enc = text_db1.to_bytes();
    
    //Check that the encoded length equals the len parameter
    assert!(enc.len() as u64 == len);

    //Write the db to a file
    //(1) get the tmp folder
    let path = env::temp_dir().as_path().join("text_db_text_io.dbtxt");
    let mut file = File::create(&path).unwrap();

    //(2) write the file
    file.write(&enc).unwrap();
    file.flush().unwrap();

    // Read the file back to test decoding functionality
    let bytes = std::fs::read(&path).unwrap();
    let text_db2 = TextDB::from_bytes(&bytes);

    //Console check to see if we asdlfkjfs
    println!("{}", text_db2);

}

#[test]
fn test_db_text_full_sadf_io() {
    //Generate a text db with the lipsum module
    let txt = lipsum::lipsum(1000);
    let (text_db1, len) = TextDB::from_text(txt, 1);
    
    //Encode the text_db thing
    let mut enc = text_db1.to_bytes();
    
    //Check that the encoded length equals the len parameter
    assert!(enc.len() as u64 == len);

    //Generate a header to complete the sadf file
    let mut header = Header::new();
    header.add_hib(HeaderIndexBlock {
        id: 0,
        ptr: 24, //data starts after hib + header fixed data fields
        length: enc.len() as u64,
        data_type: 0x0000
    });

    //Add the header and the data block to make the full sadf file
    let mut enc_tot = header.to_bytes();
    enc_tot.append(&mut enc);

    //Write the db to a file
    //(1) get the tmp folder
    let path = env::temp_dir().as_path().join("text_db_text_full_io.sadf");
    let mut file = File::create(&path).unwrap();

    //(2) write the file
    file.write(&enc_tot).unwrap();
    file.flush().unwrap();
}