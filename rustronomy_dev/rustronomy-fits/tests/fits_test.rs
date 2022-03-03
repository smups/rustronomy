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

use rustronomy_fits::fits::Fits;

static FAKE_FILE: &str = "resources/tests/fake.fits";
static REAL_FILE: &str = "resources/tests/real.fits";
static FAKE_WRITE_FILE: &str = "resources/tests/write.fits";

#[test]
fn read_fits_test() {

    //(1) Fake file
    //Get the path to the fake file
    let mut fake = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fake.push(FAKE_FILE);

    //And open it (there is no data in this file)
    let fits_fake = Fits::open(&fake).unwrap();
    print!("{}", fits_fake.header);

    //(2) Real file
    let mut real = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    real.push(REAL_FILE);

    let mut fits_real = Fits::open(&real).unwrap();
    print!("{}", fits_real.header);

    //Try to read the image in the file
    fits_real.read_img().unwrap();
}