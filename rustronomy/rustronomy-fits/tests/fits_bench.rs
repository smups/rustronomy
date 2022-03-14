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

use std::{path::PathBuf, fs, time::Instant};

use rustronomy_fits as rfs;

const BENCH_FOLDER: &str = "resources/tests/real_assortment";

#[test]
fn read_fits_benchmark() {

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push(BENCH_FOLDER);

    //Log files and times
    let mut fits_files = Vec::new();
    let mut times = Vec::new();

    for path in fs::read_dir(root).unwrap() {
        let now = Instant::now();
        let fits = rfs::Fits::open(&path.unwrap().path()).unwrap();
        let time = now.elapsed().as_micros();
        println!("Read time: {time}μs");
        times.push(time);
        fits_files.push(fits);
    }

    print!("{}", fits_files[0]);
    println!("Average read time: {}μs", times.iter().sum::<u128>() as usize / times.len())

}