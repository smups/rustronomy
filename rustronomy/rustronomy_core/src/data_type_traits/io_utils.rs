/*
    Copyright (C) 2021 Raúl Wolters
    
    This file is part of rustronomy-core.
    
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

pub trait Encode {
    fn to_bytes(&self) -> Vec<u8>;
    fn fill_buf(&self, buf: &mut Vec<u8>) {
        buf.append(&mut self.to_bytes());
    }
}

pub trait EncodeAndConsume {
    fn fill_buf(self, buf: &mut Vec<u8>);
    fn to_bytes(self) -> Vec<u8>;
}

pub trait Decode {
    fn from_bytes(data: &Vec<u8>) -> Self;
}