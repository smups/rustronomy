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

use rustronomy_fits as rfs;

static REAL_FILE: &str = "resources/tests/real.fits";

#[test]
fn read_fits_test() {

    let mut real = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    real.push(REAL_FILE);

    let mut fits = rfs::Fits::open(&real).unwrap();
    print!("{fits}");

    //Destruct the FITS file
    let hdu = fits.remove_hdu(0).unwrap();
    let (header, data) = hdu.to_parts();
    println!("{header}");

    //Get the data as an array
    let array = match data.unwrap() {
        rfs::Extension::Image(img) => img.as_owned_f64_array().unwrap(),
        _ => panic!()
    };
    println!("{array}");
}