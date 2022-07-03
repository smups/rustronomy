/*
    Copyright (C) 2022 Raúl Wolters
    
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

//! This module specifies a generic Image container. An Image consists of a 2D
//! `NDArray` and a list of metadata tags.

use std::{
    collections::HashMap,
    fmt::{self, Formatter, Debug, Display},
    str::FromStr, mem, any
};

use ndarray::Array2;
use num_traits::Num;

use super::metadata::{
    MetaDataErr, MetaDataContainer,
    MetaDataTag, GenericMetaDataTag,
    RESERVED_TAGS, AUTHOR
};

pub struct Image<T: Num> {
    data: Array2<T>,
    meta: HashMap<String, String>
}

impl<U: Num, T> MetaDataContainer<T> for Image<U> 
where T: Display + Sized + Send + Sync + FromStr, <T as FromStr>::Err: Debug
{
    fn add_generic_tag(&mut self, tag: GenericMetaDataTag<T>)
        -> Result<(), MetaDataErr>
    {
        //(1) Check if the key is reserved
        if RESERVED_TAGS.contains(&tag.key.as_str()) {
            return Err(MetaDataErr::RestrictedKey(tag.key));
        }

        //(2) Check if the key already exists
        if self.meta.contains_key(&tag.key) {
            return Err(MetaDataErr::KeyExists(tag.key))
        }

        //(3) we're good -> add the key
        let (key, value) = tag.as_string_pair();
        self.meta.insert(key, value);
        Ok(())
    }

    fn remove_generic_tag(&mut self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr> {
        //(1) Check if the key is reserved
        if RESERVED_TAGS.contains(&key) {
            return Err(MetaDataErr::RestrictedKey(key.to_string()));
        }

        //(2) Check if the key does not exists
        if !self.meta.contains_key(key) {
            return Err(MetaDataErr::KeyNotFound(key.to_string()))
        }

        //(3) we're good -> remove the key
        let (key, value) = self.meta.remove_entry(key).unwrap();
        Ok(GenericMetaDataTag::<T>::parse_string_pair(key, &value))
    }
}

impl<U: Num> Image<U> {

    /// constructs new image (without metadata tags) from 2D ndarray
    pub fn new(data: Array2<U>) -> Self {
        Image { data, meta: HashMap::new() }
    }

    /// returns image data throwing away the metadata tags
    pub fn data(self) -> Array2<U> { self.data }

    /// set the author of the image to the supplied value
    pub fn set_author(&mut self, author: &str) {
        self.meta.insert(AUTHOR.to_string(), author.to_string());
    }

    /// returns the author of the image, if one was specified
    pub fn get_author(&self) -> Option<&String> {
        self.meta.get(AUTHOR)
    }

}

impl<U: Num> Display for Image<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, ">============================<|RUSTRONOMY IMAGE 🦀🌌|>============================")?;
        writeln!(f, ">shape: ({}x{})", self.data.shape()[0], self.data.shape()[1])?;
        writeln!(f, ">size: {}", {
            let byte_size = self.data.len() * mem::size_of::<U>();
            if byte_size <= 1000 {
                format!("{}B", byte_size)
            } else if byte_size >= 1_000_000 {
                format!("{}kB", byte_size / 1000)
            } else if byte_size >= 1_000_000_000 {
                format!("{}MB", byte_size / 1_000_000)
            } else {
                format!("{}GB", byte_size / 1_000_000_000)
            }
        })?;
        writeln!(f, ">datatype: {}", any::type_name::<U>())?;
        writeln!(f, ">----------------------------------<|METADATA|>---------------------------------")?;
        for (tag, val) in self.meta.iter() {
            writeln!(f, ">\"{tag}\": {val}")?;
        }
        writeln!(f, ">===============================================================================")
    }
}