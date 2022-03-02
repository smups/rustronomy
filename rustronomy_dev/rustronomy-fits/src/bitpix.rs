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

use std::error::Error;

use simple_error::SimpleError;

#[derive(Debug)]
pub enum Bitpix {
    Char(u8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Spf(f32),
    Dpf(f64)
}

impl Bitpix {

    pub fn bitpix(val: isize) -> Result<Bitpix, Box<dyn Error>>{
        match val {
            val @ 8 => Ok(Bitpix::Char(val as u8)),
            val @ 16 => Ok(Bitpix::Short(val as i16)),
            val @ 32 => Ok(Bitpix::Integer(val as i32)),
            val @ 64 => Ok(Bitpix::Long(val as i64)),
            val @ -32 => Ok(Bitpix::Spf(val as f32)),
            val @ -64 => Ok(Bitpix::Dpf(val as f64)),
            _ => Err(Box::new(SimpleError::new(
                "Invalid bitpix value found!"
            )))
        }
    }
    
    pub fn get_code(&self) -> isize {
        match self {
            &Bitpix::Char(_) => 8,
            &Bitpix::Short(_) => 16,
            &Bitpix::Integer(_) => 32,
            &Bitpix::Long(_) => 64,
            &Bitpix::Spf(_) => -32,
            &Bitpix::Dpf(_) => -64
        }
    }
}