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

//SADF version number
pub static SADF_VERSION: u16 = 211;
pub static RUSTRONOMY_SADF_VERSION: &str = "v0.1";

//Module structure
mod header;

//tests
#[cfg(test)]
mod test;