/*
    Copyright (C) 2021 Raúl Wolters
    
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

use crate::io::{complex::Complex, Encode, Decode};

/*
    This file contains 
*/

pub enum KeyWordVal where
    KeyWordVal:Encode,
    KeyWordVal:Decode
{
    Blank,
    CharacterString(String),
    ContinuedString(String),
    Logical(bool),
    Integer(i64),
    Float(f64),
    ComplexInt(Complex<i64>),
    ComplexFloat(Complex<f64>)
}

impl Encode for KeyWordVal {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Blank => Vec::new(),
            Self::CharacterString(val) => val.to_bytes(),
            Self::ContinuedString(val) => val.to_bytes(),
            Self::Logical(val) => val.to_bytes(),
            Self::Integer(val) => val.to_bytes(),
            Self::Float(val) => val.to_bytes(),
            Self::ComplexInt(val) => val.to_bytes(),
            Self::ComplexFloat(val) => val.to_bytes()
        }
    }
}

impl Decode for KeyWordVal {
    fn from_bytes(data: &Vec<u8>) -> Self {
        todo!()
    }
}