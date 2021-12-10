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

//Imports
use super::*;

pub struct Complex<T> where
    T:Encode,
    T:Decode
{
    pub val1: T,
    pub val2: T
}

pub trait ComplexFuncs<T> where
    T:Encode,
    T:Decode
{
    fn from_parts(val1: T, val2: T) -> Complex<T> {
        Complex {val1:val1, val2:val2}
    }
    fn from_euler(arg: f64, abs: f64) -> Complex<T> {
        let val1 = ( f64::sin(arg) * abs ) as T;
        let val2 = ( f64::cos(arg) * abs ) as T;
        Complex { val1: val1, val2: val2 }
    }
    fn arg(&self) -> f64;
    fn abs(&self) -> f64;
}

impl Decode for Complex<i64>{
    fn from_bytes(data: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Decode for Complex<f64>{
    fn from_bytes(data: &Vec<u8>) -> Self {
        todo!()
    }
}

impl Encode for Complex<i64>{
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Encode for Complex<f64>{
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl ComplexFuncs<i64> for Complex<i64> {
    fn arg(&self) -> f64 {
        todo!()
    }

    fn abs(&self) -> i64 {
        todo!()
    }
}