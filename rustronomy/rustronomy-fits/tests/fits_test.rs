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

use rustronomy_fits::{fits::Fits, extensions::{image::Image, Extension}};

static REAL_FILE: &str = "resources/tests/real.fits";

#[test]
fn read_fits_test() {

    let mut real = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    real.push(REAL_FILE);

    let fits_real = Fits::open(&real).unwrap();
    print!("{fits_real}");

    //Get the data out of the fits file
    let hdu = fits_real.get_hdu(0).unwrap();
    let data;

    match hdu.get_data().unwrap() {
        Extension::Image(img) => {
            data = img.as_f64_array().unwrap()
        } _ => {panic!()} //do nothing
    }

    println!("{data}");

}