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
    fmt::Display
};

use ndarray::{Array, IxDyn, Array1};
use num_traits::Num;
use simple_error::SimpleError;

use crate::{bitpix::Bitpix, raw::raw_fits::RawFitsReader};
use super::Xtension;

use rustronomy_core::data_type_traits::io_utils::Decode;

//Get block size from root
const BLOCK_SIZE: usize = crate::BLOCK_SIZE; // = 2880B

//IO consts
const MAX_BLOCKS_IN_BUF: usize = 128; // = 369kB
const MIN_BLOCKS_IN_BUF: usize = 1; // = 3kB

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
    pub fn decode_img(reader: &mut RawFitsReader, shape: &Vec<usize>, bitpix: Bitpix)
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

    fn decode_helper<T>(reader: &mut RawFitsReader, shape: &Vec<usize>)
        -> Result<Image<T>, Box<dyn Error>>
    where
        T: Debug + Num + Sized + Decode + Display
    {
        /*  (1)
            To create a ndarray we need to provide an underlying data structure.
            For now we'll use the easiest one to implement: a giant 1D vector
            (this happens to also be somewhat efficient). First, we must find
            the number of entries in this vector by folding the array shape.
        */
        let entry_size = size_of::<T>();
        let n_entries = (&shape).iter().fold(1, |prod, &x| prod * x);
        let byte_size = n_entries * entry_size;
        let total_blocks = (byte_size as f64 / BLOCK_SIZE as f64).ceil() as usize;

        /*  Notes:
            FITS supports integers and floats as data types. These are either 1,
            2, 4 or 8 bytes long. Hence BLOCK_SIZE % entry_size == 0 for all data
            types recognized by the FITS standard (we do not have to deal with
            data types spanning multiple FITS blocks).
        */

        /*  (2)
            For performance reasons, we want to read in chunks of at least one 
            FITS block, which is 2880 bytes ~3kB. It takes about as many CPU
            cycles to copy 4kB as it does to make a syscall. We do not want to
            make the buffer larger than the L3 cache of the CPU though, so we
            must limit ourselves to below ~1MB. Hence we must determine an
            optimal size for the buffer to pass to the read_blocks function of
            the RawFitsReader.
        */

        //Get the buffer size and the number of times we have to fill the buffer
        let (buf_size, n_reads) = Self::calc_buf_size(total_blocks);

        //Create the vector underpinning the ndarray and the reusable buffer
        let mut flat: Vec<T> = Vec::new();
        let mut buf = vec![0u8; buf_size.try_into().unwrap()];

        //calculate number of entries in a buffer
        let entries_in_buffer = buf_size / entry_size;

        for _ in 0..n_reads{
            //fill the buffer
            reader.read_blocks(&mut buf)?;

            for i in 0..entries_in_buffer {
                //fill the flat vector
                flat.push(T::from_bytes(&buf[i*entry_size..(i+1)*entry_size]));
            }
        }

        /*  (3)
            So far we have read an integer multiple of BLOCK_SIZE in bytes.
            Although we are guaranteed to have captured all the data necessary,
            we probably read too many values because the last FITS block may be
            partially empty. Hence we need to pop the difference of the vector.
        */
        flat.truncate(n_entries);

        //(4) Next we turn the (VERY EXPENSIVE) vector into an array:
        let flat_arr = Array1::<T>::from_vec(flat);
        
        //which we reshape into the desired form
        let img_data = flat_arr.into_shape(shape.to_vec())?;
        print!("{img_data}");

        Ok(Image::<T> {shape: shape.to_vec(), data: img_data})
    }

    fn calc_buf_size(total_blocks: usize) -> (usize, usize) {
        //Return tuple: (buffer size in bytes, #syscalls/reads)

        /* Notes:
            As per the FITS standard, we may only read a FITS block of bytes per
            read. We want the largest integer multiple of the size of a FITS block
            below a maximum buffer size (around ~370kB).
        */

        let mut n_buf_blocks = 1;

        for i in MIN_BLOCKS_IN_BUF..=MAX_BLOCKS_IN_BUF {
            //If the buffer is the same size as the image, we don't need a bigger buf
            if n_buf_blocks == total_blocks {break;}

            //If a larger buffer works, use it!
            if total_blocks % i == 0 {n_buf_blocks = i;}
        }

        let n_reads = total_blocks / n_buf_blocks;

        println!("Buffer size: {n_buf_blocks}");

        (n_buf_blocks * BLOCK_SIZE, n_reads)
    }
}