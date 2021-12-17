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

use crate::data_types::DataType;

#[test]
fn match_test() {
    //Loop throuhg all possible u16 values to check
    for code in 0..=u16::MAX {
        //Encode the u16
        let bytes = code.to_bytes();

        //Turn the bytes into a DataType
        let data_type = DataType::from_bytes(&bytes);

        if data_type != DataType::Malformed {
            //May be commented out in the future
            println!("[{:#0000X}] - {:?}", code, data_type);

            //Turn the DataType back into a code and assert they are equal!
            let code_enc = u16::from_bytes(&data_type.to_bytes());
            assert!(code_enc == code);
        }
    }
}