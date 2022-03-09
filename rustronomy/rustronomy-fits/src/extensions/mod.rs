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

use std::fmt::Display;

use crate::raw::BlockSized;

use self::image::TypedImage;

//FITS standard-conforming extensions
pub mod image;

#[derive(Debug)]
pub enum Extension {
    Corrupted,
    Image(TypedImage)
}

impl BlockSized for Extension {
    fn get_block_len(&self) -> usize {
        match &self {
            Self::Corrupted => 0, //corrupted data is disregarded
            Self::Image(img) => img.get_block_len()
        }
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Corrupted => write!(f, "(CORRUPTED_DATA)"),
            Self::Image(img) => write!(f, "(IMAGE) {img}")
        }
    }
}