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

use core::fmt::Debug;
use std::{
    mem::size_of,
    error::Error,
    fs::File,
    io::{BufReader, Read}, fmt::Display
};

use ndarray::{Array, IxDyn, Array1};
use num_traits::Num;
use simple_error::SimpleError;

use crate::bitpix::Bitpix;

use super::Xtension;

use rustronomy_core::data_type_traits::io_utils::Decode;

pub trait Img: Xtension{
    //Only used for dyn Image objects!
}

#[derive(Debug)]
pub struct Image<T> where
    T: Debug + Num
{
    shape: Vec<usize>,
    data: Array<T, IxDyn>
}

//Fake object trait implementations
impl<T> Xtension for Image<T> where T: Debug + Num {} 
impl<T> Img for Image<T> where T: Debug + Num {}

//Real user-facing helper functions
impl<T> Image<T> where T: Debug + Num {

    pub fn as_ndarray(self) -> Array<T, IxDyn> {
        self.data
    }

    pub fn from_ndarray(array: Array<T, IxDyn>) -> Self {
        Image { shape: array.shape().to_vec(), data: array }
    }
    
}

//Helper struct for reading/writing Images
pub struct ImgParser {}
impl ImgParser {

    //Public decoder for parsing images
    pub fn decode_img(reader: &mut BufReader<File>, shape: &Vec<usize>, bitpix: Bitpix)
        -> Result<Box<dyn Xtension>, Box<dyn Error>>
    {
        Ok(
        match bitpix {
            Bitpix::Byte => Box::new(Self::decode_helper::<u8>(reader, shape)?),
            Bitpix::Short => Box::new(Self::decode_helper::<i16>(reader, shape)?),
            Bitpix::Int => Box::new(Self::decode_helper::<i32>(reader, shape)?),
            Bitpix::Long => Box::new(Self::decode_helper::<i64>(reader, shape)?),
            Bitpix::Spf => Box::new(Self::decode_helper::<f32>(reader, shape)?),
            Bitpix::Dpf => Box::new(Self::decode_helper::<f64>(reader, shape)?)
        }
        )
    }

    fn decode_helper<T>(reader: &mut BufReader<File>, shape: &Vec<usize>)
        -> Result<Image<T>, Box<dyn Error>>
    where
        T: Debug + Num + Sized + Decode + Display
    {
        let type_size = size_of::<T>();

        //First, we put all the bytes in a giant 1D vector
        // a (n_1, n_2, ..., n_N) dimensional array has n_1* ... * n_N elements,
        // hence we can yeet all the data in an array that is n_entries long
        let n_entries: usize = (&shape).iter().fold(1, |prod, &x| prod * x);
        println!("{shape:?}");
        println!("Entries: {n_entries}");
        
        //Actually filling the vector
        let mut flat: Vec<T> = Vec::new();
        let mut buf = vec![0u8; type_size];

        for _ in 0..n_entries {
            //Read to the buffer
            reader.read_exact(&mut buf)?;
            flat.push(T::from_bytes(&buf));
        }

        //Next we turn the (VERY EXPENSIVE) vector into an array:
        let flat_arr = Array1::<T>::from_vec(flat);
        
        //which we reshape into the desired form
        let img_data = flat_arr.into_shape(shape.to_vec())?;
        print!("{img_data}");

        Ok(Image::<T> {shape: shape.to_vec(), data: img_data})
    }
}