/*
    Copyright (C) 2021 Raúl Wolters
    
    This file is part of rustronomy-sadf.
    
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

//Imports
use rustronomy_core::data_type_traits::io_utils::{
    Encode,
    Decode
};

#[derive(PartialEq)]
pub enum DataType {
    //Raw types
    Raw,
    //Bool Types
    Logic,
    //Unsigned Integers
    Usig8,
    Usig16,
    Usig32,
    Usig64,
    //Signed Integers
    Sig16,
    Sig32,
    Sig64,
    //Floating point
    Fp32,
    Fp64,
    //Strings
    Utf8,
    Utf16,
    //Pointer
    Ptr,
    //Complex numbers
    Xusig16,
    Xusig32,
    Xusig64,
    Xsig16,
    Xsig32,
    Xsig64,
    Xfp32,
    Xfp64,
    //User Defined
    USR(u16)
}

impl Encode for DataType {
    fn to_bytes(&self) -> Vec<u8> {
        let code: u16 = match self {
            //Raw
            DataType::Raw => 0x0000,
            //Bools
            DataType::Logic => 0x0001,
            //Unsigned ints
            DataType::Usig8 => 0x0008,
            DataType::Usig16 => 0x0016,
            DataType::Usig32 => 0x0032,
            DataType::Usig64 => 0x0064,
            //Signed ints
            DataType::Sig16 => 0x0010,
            DataType::Sig32 => 0x0020,
            DataType::Sig64 => 0x0040,
            //Floating point
            DataType::Fp32 => 0x0f20,
            DataType::Fp64 => 0x0f40,
            //Strings
            DataType::Utf8 => 0xca08,
            DataType::Utf16 => 0xca16,
            //Pointer
            DataType::Ptr => 0xa064,
            //Complex numbers
            DataType::Xusig16 => 0xc016,
            DataType::Xusig32 => 0xc032,
            DataType::Xusig64 => 0xc064,
            DataType::Xsig16 => 0xc010,
            DataType::Xsig32 => 0xc020,
            DataType::Xsig64 => 0xc040,
            DataType::Xfp32 => 0xcf32,
            DataType::Xfp64 => 0xcf64,
            //User-defined
            DataType::USR(code) => *code
        };
        return code.to_bytes();
    }
}

impl Decode for DataType {
    fn from_bytes(data: &Vec<u8>) -> Self {
        assert!(data.len() == 2); //Must be 2 bytes long
        match u16::from_bytes(&data) {
            //Raw
            0x0000 => DataType::Raw,
            //Bools
            0x0001 => DataType::Logic,
            //Unsigned Integers
            0x0008 => DataType::Usig8,
            0x0016 => DataType::Usig16,
            0x0032 => DataType::Usig32,
            0x0064 => DataType::Usig64,
            //Signed Integers
            0x0010 => DataType::Sig16,
            0x0020 => DataType::Sig32,
            0x0040 => DataType::Sig64,
            //Floating Point
            0x0f20 => DataType::Fp32,
            0x0f40 => DataType::Fp64,
            //Strings
            0xca08 => DataType::Utf8,
            0xca16 => DataType::Utf16,
            //Pointer
            0xa064 => DataType::Ptr,
            //Complex Numbers
            0xc016 => DataType::Xusig16,
            0xc032 => DataType::Xusig32,
            0xc064 => DataType::Xusig64,
            0xc010 => DataType::Xsig16,
            0xc020 => DataType::Xsig32,
            0xc040 => DataType::Xsig64,
            0xcf20 => DataType::Xfp32,
            0xcf40 => DataType::Xfp64,
            //User-defined
            code @ 0xb000..=0xbfff => DataType::USR(code),
            //Undefined!
            _ => panic!("Invalid DataType found")
        }
    }
}