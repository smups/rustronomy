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
    path::PathBuf,
    fs,
    time::Instant,
    error::Error
};

use nd::Array2;
use rustronomy_fits as rfs;
use plotters::prelude::*;
use ndarray as nd;

//Starfields of M37 taken by myself
const BENCH_FOLDER: &str = "resources/bench_data";

//output folder
const OUTPUT: &str = "resources/bench_out";

#[test]
fn read_fits_benchmark() {

    /*  Description:
        This test reads 65 ~7MB FITS files containing Images of 
    */

    //Root folder of project
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    
    let mut data_f = root.clone();
    data_f.push(BENCH_FOLDER);

    let mut result_f = root.clone();
    result_f.push(OUTPUT);

    //Log files and times
    let mut read_times = Vec::new();
    let mut plot_times = Vec::new();

    //These are all ~7MB files.
    //Result (Ryzen 3600X, NVME SSD) average read time ~131ms
    for (index, path) in fs::read_dir(data_f).unwrap().enumerate() {
        let now = Instant::now();
        let mut fits = rfs::Fits::open(&path.unwrap().path()).unwrap();
        let time = now.elapsed().as_millis();
        println!("Read time: {time}ms");
        read_times.push(time);

        let now = Instant::now();
        let (_header, data) = fits.remove_hdu(1).unwrap().to_parts();
        let array = match data.unwrap() {
            rfs::Extension::Image(img) => img.as_owned_f64_array().unwrap(),
            _ => panic!()
        };
        let img: Array2<f64> = array.into_dimensionality::<nd::Ix2>().unwrap();

        //Get an appropriate path
        let mut output_path = result_f.clone();
        output_path.push(format!("master_science{index}.png"));

        plot(img, &output_path);
        let time = now.elapsed().as_millis();
        println!("Read time: {time}ms");
        plot_times.push(time);
        
    }


    println!("Average read time: {}ms", read_times.iter().sum::<u128>() as usize / read_times.len());
    println!("Average plot time: {}ms", plot_times.iter().sum::<u128>() as usize / plot_times.len())

}

fn plot(img: Array2<f64>, output: &std::path::Path) {

    //Calculate size of the image
    let x_size = img.shape()[0];
    let y_size = img.shape()[1];

    //Calculate max and min values in the Image
    let mut max: f64 = f64::MIN;
    for val in img.iter() { if val > &max {max = *val} }

    let mut min: f64 = f64::MAX;
    for val in img.iter() { if val < &min {min = *val} }

    //Create a BitMap with the same size in pixels as the Image
    let root = BitMapBackend::new(output, (x_size as u32, y_size as u32)).into_drawing_area();
    root.fill(&RED).unwrap();

    //create base chart with the size of the image
    let mut chart = ChartBuilder::on(&root).build_cartesian_2d(0..x_size, 0..y_size).unwrap();

    //Remove the mesh from the chart
    chart.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw().unwrap();

    let plotting_area = chart.plotting_area();

    for ((x,y), count) in img.indexed_iter() {
        plotting_area.draw_pixel((x, y), &grey_scale(*count, min, max.log10()).unwrap()).unwrap()
    }
}

fn grey_scale(count: f64, min: f64, log_max: f64)
    -> Result<RGBColor, Box<dyn Error>>
{
    let col: u8 =
    (//This should be within the 0-255 range!
        255. * (count/min).abs().log10() / log_max
    ) as u8;

    Ok(RGBColor(col, col, col))
}