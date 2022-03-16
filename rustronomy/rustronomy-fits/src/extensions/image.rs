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

use ndarray::{Array, IxDyn, ShapeBuilder};
use num_traits::Num;
use simple_error::SimpleError;

use crate::{bitpix::Bitpix, raw::{raw_fits::RawFitsReader, BlockSized}};
use super::Extension;

use rustronomy_core::data_type_traits::io_utils::Decode;

//Get block size from root
const BLOCK_SIZE: usize = crate::BLOCK_SIZE; // = 2880B

//IO consts
const MAX_BLOCKS_IN_BUF: usize = 128; // = 369kB
const MIN_BLOCKS_IN_BUF: usize = 1; // = 3kB

#[derive(Debug)]
pub struct Image<T> where
    T: Debug + Num
{
    shape: Vec<usize>,
    data: Array<T, IxDyn>,
    block_size: usize
}

impl<T> BlockSized for Image<T> where T: Debug + Num {
    fn get_block_len(&self) -> usize {
        self.block_size
    }
}

impl<T> Image<T> where T: Debug + Num {

    //Getters
    pub fn get_data(&self) -> &Array<T, IxDyn> {&self.data}
    pub fn get_data_owned(self) -> Array<T, IxDyn> {self.data}
    pub fn get_shape(&self) -> &Vec<usize> {&self.shape}

    pub fn pretty_print_shape(&self) -> String {
        let mut rsp = String::from("(");
        for ax in &self.shape {
            rsp += format!("{ax},").as_str();
        }
        rsp.pop(); //remove last comma
        String::from(rsp + ")")
    }
}

//Enum to differentiate between Image Types
#[derive(Debug)]
pub enum TypedImage {
    ByteImg(Image<u8>),
    I16Img(Image<i16>),
    I32Img(Image<i32>),
    I64Img(Image<i64>),
    SpfImg(Image<f32>),
    DpfImg(Image<f64>)
}

impl BlockSized for TypedImage {
    fn get_block_len(&self) -> usize {
        match self {
            Self::ByteImg(var) => var.get_block_len(),
            Self::I16Img(var) => var.get_block_len(),
            Self::I32Img(var) => var.get_block_len(),
            Self::I64Img(var) => var.get_block_len(),
            Self::SpfImg(var) => var.get_block_len(),
            Self::DpfImg(var) => var.get_block_len()
        }
    }
}

impl Display for TypedImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::ByteImg(img) => {
               write!(f, "datatype: u8, shape: {}, size: {}",
                    img.pretty_print_shape(), img.get_block_len()
                )
            } Self::I16Img(img) => {
                write!(f, "datatype: i16, shape: {}, size: {}",
                    img.pretty_print_shape(), img.get_block_len()
                )
            } Self::I32Img(img) => {
                write!(f, "datatype: i32, shape: {}, size: {}",
                    img.pretty_print_shape(), img.get_block_len()
                )
            } Self::I64Img(img) => {
                write!(f, "datatype: i64, shape: {}, size: {}",
                    img.pretty_print_shape(), img.get_block_len()
                )
            } Self::SpfImg(img) => {
                write!(f, "datatype: f32, shape: {}, size: {}",
                    img.pretty_print_shape(), img.get_block_len()
                )
            } Self::DpfImg(img) => {
                write!(f, "datatype: f64, shape: {}, size: {}",
                    img.pretty_print_shape(), img.get_block_len()
                )
            }
       }
    }
}
impl TypedImage {

    pub fn as_u8_array(&self) -> Result<&Array<u8, IxDyn>, Box<dyn Error>> {
        match &self {
            Self::ByteImg(img) => Ok(img.get_data()),
            &var => Err(Box::new(SimpleError::new(
                format!("Tried to borrow {:?} as u8 array", var)
            )))
        }
    }

    pub fn as_i16_array(&self) -> Result<&Array<i16, IxDyn>, Box<dyn Error>> {
        match &self {
            Self::I16Img(img) => Ok(img.get_data()),
            &var => Err(Box::new(SimpleError::new(
                format!("Tried to borrow {:?} as i16 array", var)
            )))
        }
    }

    pub fn as_i32_array(&self) -> Result<&Array<i32, IxDyn>, Box<dyn Error>> {
        match &self {
            Self::I32Img(img) => Ok(img.get_data()),
            &var => Err(Box::new(SimpleError::new(
                format!("Tried to borrow {:?} as i32 array", var)
            )))
        }
    }

    pub fn as_i64_array(&self) -> Result<&Array<i64, IxDyn>, Box<dyn Error>> {
        match &self {
            Self::I64Img(img) => Ok(img.get_data()),
            &var => Err(Box::new(SimpleError::new(
                format!("Tried to borrow {:?} as i64 array", var)
            )))
        }
    }

    pub fn as_f32_array(&self) -> Result<&Array<f32, IxDyn>, Box<dyn Error>> {
        match &self {
            Self::SpfImg(img) => Ok(img.get_data()),
            &var => Err(Box::new(SimpleError::new(
                format!("Tried to borrow {:?} as f32 array", var)
            )))
        }
    }

    pub fn as_f64_array(&self) -> Result<&Array<f64, IxDyn>, Box<dyn Error>> {
        match &self {
            Self::DpfImg(img) => Ok(img.get_data()),
            &var => Err(Box::new(SimpleError::new(
                format!("Tried to borrow {:?} as f64 array", var)
            )))
        }
    }

    pub fn as_owned_u8_array(self) -> Result<Array<u8, IxDyn>, Box<dyn Error>> {
        match self {
            Self::ByteImg(img) => Ok(img.get_data_owned()),
            var => Err(Box::new(SimpleError::new(
                format!("Tried to convert {:?} to an u8 array", var)
            )))
        }
    }

    pub fn as_owned_i16_array(self) -> Result<Array<i16, IxDyn>, Box<dyn Error>> {
        match self {
            Self::I16Img(img) => Ok(img.get_data_owned()),
            var => Err(Box::new(SimpleError::new(
                format!("Tried to convert {:?} to an i16 array", var)
            )))
        }
    }

    pub fn as_owned_i32_array(self) -> Result<Array<i32, IxDyn>, Box<dyn Error>> {
        match self {
            Self::I32Img(img) => Ok(img.get_data_owned()),
            var => Err(Box::new(SimpleError::new(
                format!("Tried to convert {:?} to an i32 array", var)
            )))
        }
    }

    pub fn as_owned_i64_array(self) -> Result<Array<i64, IxDyn>, Box<dyn Error>> {
        match self {
            Self::I64Img(img) => Ok(img.get_data_owned()),
            var => Err(Box::new(SimpleError::new(
                format!("Tried to convert {:?} to an i64 array", var)
            )))
        }
    }

    pub fn as_owned_f32_array(self) -> Result<Array<f32, IxDyn>, Box<dyn Error>> {
        match self {
            Self::SpfImg(img) => Ok(img.get_data_owned()),
            var => Err(Box::new(SimpleError::new(
                format!("Tried to convert {:?} to an f32 array", var)
            )))
        }
    }

    pub fn as_owned_f64_array(self) -> Result<Array<f64, IxDyn>, Box<dyn Error>> {
        match self {
            Self::DpfImg(img) => Ok(img.get_data_owned()),
            var => Err(Box::new(SimpleError::new(
                format!("Tried to convert {:?} to an f64 array", var)
            )))
        }
    }

}

//Helper struct for reading/writing Images
pub struct ImgParser {}
impl ImgParser {

    //Public decoder for parsing images
    pub fn decode_img(reader: &mut RawFitsReader, shape: &Vec<usize>, bitpix: Bitpix)
        -> Result<Extension, Box<dyn Error>>
    {
        Ok(Extension::Image(match bitpix {
            Bitpix::Byte => TypedImage::ByteImg(Self::decode_helper::<u8>(reader, shape)?),
            Bitpix::Short => TypedImage::I16Img(Self::decode_helper::<i16>(reader, shape)?),
            Bitpix::Int => TypedImage::I32Img(Self::decode_helper::<i32>(reader, shape)?),
            Bitpix::Long => TypedImage::I64Img(Self::decode_helper::<i64>(reader, shape)?),
            Bitpix::Spf => TypedImage::SpfImg(Self::decode_helper::<f32>(reader, shape)?),
            Bitpix::Dpf => TypedImage::DpfImg(Self::decode_helper::<f64>(reader, shape)?)
        }))
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
        let mut buf = vec![0u8; buf_size];

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

        /*  (4)
            The next step is to convert the flat vector into an array of an
            appropriate size. Note that the FITS specification states that Images
            are represented in the Fortran (column-major) memory-layout, not 
            row-major like C and Rust. Hence we have to call the .f() function
            on the shape of the array to tell ndarray that we have a Fortran array.
        */
        let img_data = Array::from_shape_vec(shape.clone().f(), flat)?;

        Ok(Image::<T> {shape: shape.to_vec(), data: img_data, block_size: total_blocks})
    }

    fn calc_buf_size(total_blocks: usize) -> (usize, usize) {
        //Return tuple: (buffer size in bytes, #syscalls/reads)

        /* Notes:
            As per the FITS standard, we may only read a FITS block of bytes per
            read. We want the largest integer multiple of the size of a FITS block
            below a maximum buffer size (around ~370kB).

            I decided to tune the maximum buffer size with the fits_bench.rs
            test (reading a bunch of 7MB files).
            
            These were the results (actual block size listed for clarity)
            [Block size]    [Byte size]     [Read time]
                1           2880B           135ms
                25          72kB            130ms
                107         308kB           137ms
                535         1.54MB          131ms
                2675        7.70MB          134ms

            Conclusion: This limit does not matter **at all*
            Reason: the buffer is heap-allocated, which means that cache optimi-
            zations don't work.
            RFC #1909 will implement this feature in the future, so I'll leave
            the code intact for now...
        */

        let mut n_buf_blocks = 1;

        for i in MIN_BLOCKS_IN_BUF..=MAX_BLOCKS_IN_BUF {
            //If the buffer is the same size as the image, we don't need a bigger buf
            if n_buf_blocks == total_blocks {break;}

            //If a larger buffer works, use it!
            if total_blocks % i == 0 {n_buf_blocks = i;}
        }

        let n_reads = total_blocks / n_buf_blocks;

        //println!("Buffer size: {n_buf_blocks}");

        (n_buf_blocks * BLOCK_SIZE, n_reads)
    }
}