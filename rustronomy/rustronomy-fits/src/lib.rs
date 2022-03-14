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

//Module structure
mod header;
mod bitpix;
mod fits;
mod extensions;
mod raw;
mod header_data_unit;

//Constants defined by the FITS standard
pub const BLOCK_SIZE: usize = 2880;

//Public api re-exports
pub use fits::Fits as Fits;
pub use header::Header as Header;
pub use header_data_unit::HeaderDataUnit as HeaderDataUnit;
pub use extensions::Extension as Extension;

//prelude (kinda pointless rn but whatev)
pub mod prelude {
    pub use crate::fits::Fits as Fits;
    pub use crate::header::Header as Header;
    pub use crate::header_data_unit::HeaderDataUnit as HeaderDataUnit;
    pub use crate::extensions::Extension as Extension;
}