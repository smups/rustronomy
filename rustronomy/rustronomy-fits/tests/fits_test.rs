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

use std::path::PathBuf;

use rustronomy_fits as rsf;

use dirs;

static REAL_FILE: &str = "resources/Hubble_NICMOS.fits";

#[test]
fn read_test() {

    let mut real = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    real.push(REAL_FILE);

    let mut fits = rsf::Fits::open(&real).unwrap();
    print!("{fits}");

    //Destruct the FITS file
    let hdu = fits.remove_hdu(1).unwrap();
    let (header, data) = hdu.to_parts();
    println!("{header}");

    //Get the data as an array
    let array = match data.unwrap() {
        rsf::Extension::Image(img) => img.as_owned_f32_array().unwrap(),
        _ => panic!()
    };
    println!("{array}");
}

#[test]
fn write_correctness_test() {
    let mut real_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    real_path.push(REAL_FILE);

    //Read the FITS file and make a copy
    let fits = rsf::Fits::open(&real_path).unwrap();
    let original = fits.clone();

    //Write the FITS file under a new name
    let chc  = dirs::cache_dir().unwrap();
    let mut copy_path = chc.clone();
    copy_path.push("copy.fits");

    fits.write(&copy_path.clone()).unwrap();

    //Read it back
    let tested = rsf::Fits::open(&copy_path.clone()).unwrap();
    print!("{original}");
    print!("{tested}");
}