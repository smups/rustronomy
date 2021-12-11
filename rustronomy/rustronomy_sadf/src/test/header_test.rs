
use std::{env, fs::File, io::Write};
use rand::{Rng, prelude::ThreadRng};
use crate::{
    header::{header_index_block::HeaderIndexBlock, Header},
    data_types::{EncodeAndConsume, Decode}
};

/*
    This file provides tests for headers
*/

fn random_hib(rng: &mut ThreadRng) -> HeaderIndexBlock {
    //Generates random HIB using supplied Random Number Generator
    HeaderIndexBlock {
        id: rng.gen(),
        ptr: rng.gen(),
        length: rng.gen(),
        data_type: rng.gen()
    }
}

fn random_header(n_nibs: u16) -> Header {
    //Generates random header of specified length
    let mut rng = rand::thread_rng();
    let mut header = Header::new();

    for _ in 0..n_nibs {
        header.add_hib(random_hib(&mut rng))
    }

    return header;
}

#[test]
fn test_header_index_block_io() {
    /*
        This test writes a test header to the OS tmp directory and then reads it
        back in. 

        This tests the following traits:
            - EncodeAndConsume
            - Decode
            - Clone
        
        This tests assumes that the equals trait was implemented correctly.
    */    
    let mut rng = rand::thread_rng();

    // (1) Create a random HIB and copy it
    let hib1 = random_hib(&mut rng);

    //We need the copy because to_bytes consumes the HIB instance
    let hib1_copy = hib1.clone();

    // (1a) Check that the copy trait was implemented correctly
    assert!(hib1 == hib1_copy);

    // (2) Write the header to a file to test encoding functionality
    let path = env::temp_dir().as_path().join("test_header_index_block_io.sadf");

    let mut file = File::create(&path).unwrap();
    file.write(&hib1.to_bytes()).unwrap();
    file.flush().unwrap();

    // (3) Read the file back to test decoding functionality
    let bytes = std::fs::read(&path).unwrap();
    let hib2 = HeaderIndexBlock::from_bytes(&bytes);

    // (4) Check that the file we read is the same as the one we put in
    assert!(hib1_copy == hib2);
}

#[test]
fn test_header_io() {
    /*
        This test tests the header io.
    */
    
    // (1) Create a random Header and copy it
    let header1 = random_header(48 as u16);
    let header1_copy = header1.clone();

    // (1a) Check that the copy was successfull
    assert!(header1 == header1_copy);

    // (2) Write the header to a file to test encoding functionality
    let path = env::temp_dir().as_path().join("test_header_io.sadf");

    let mut file = File::create(&path).unwrap();
    file.write(&header1.to_bytes()).unwrap();
    file.flush().unwrap();

    // (3) Read the file back to test decoding functionality
    let bytes = std::fs::read(&path).unwrap();
    let header2 = Header::from_bytes(&bytes);

    // (4) Check that the written-then-read version is the same as the old one
    assert!(header1_copy == header2);
}

#[test]
fn test_header_formatting() {
    // Create a random Header
    let mut header1 = random_header(24);

    //Add a metadata block
    header1.add_hib(HeaderIndexBlock {
        id: 0x0000,
        ptr: 0x1230,
        length: 1045,
        data_type: 0xffff
    });

    print!("{}", header1);

    //Now print a header without (probably) a metadata block
    print!("{}", random_header(24));
}