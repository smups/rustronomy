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

//! This module specifies a generic array container. In contrast to an Image, an
//! Array may have any number of axes. This comes at some allocation cost, see the
//! ndarray docs for details.

use std::{
    any,
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
    mem,
    str::FromStr,
};

use ndarray::{Array, Dimension, IxDyn};
use num_traits::Num;

use super::metadata::{
    GenericMetaDataTag, MetaDataContainer, MetaDataErr, MetaDataTag, AUTHOR, RESERVED_TAGS,
};

pub struct DataArray<T: Num> {
    data: Array<IxDyn, T>,
    meta: HashMap<String, String>,
}

impl<U: Num, T> MetaDataContainer<T> for DataArray<U>
where
    T: Display + Sized + Send + Sync + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn add_generic_tag(&mut self, tag: GenericMetaDataTag<T>) -> Result<(), MetaDataErr> {
        //(1) Check if the key is reserved
        if RESERVED_TAGS.contains(&tag.key.as_str()) {
            return Err(MetaDataErr::RestrictedKey(tag.key));
        }

        //(2) Check if the key already exists
        if self.meta.contains_key(&tag.key) {
            return Err(MetaDataErr::KeyExists(tag.key));
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
            return Err(MetaDataErr::KeyNotFound(key.to_string()));
        }

        //(3) we're good -> remove the key
        let (key, value) = self.meta.remove_entry(key).unwrap();
        Ok(GenericMetaDataTag::<T>::parse_string_pair(key, &value))
    }
}

impl<U: Num> DataArray<U> {
    /// constructs new array (without metadata tags) from 2D ndarray
    pub fn new(data: Array<IxDyn, U>) -> Self {
        DataArray {
            data,
            meta: HashMap::new(),
        }
    }

    /// returns array data throwing away the metadata tags
    pub fn data(self) -> Array<IxDyn, U> {
        self.data
    }

    /// set the author of the array to the supplied value
    pub fn set_author(&mut self, author: &str) {
        self.meta.insert(AUTHOR.to_string(), author.to_string());
    }

    /// returns the author of the array, if one was specified
    pub fn get_author(&self) -> Option<&String> {
        self.meta.get(AUTHOR)
    }
}

impl<U: Num + Dimension> Display for DataArray<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            ">============================<|RUSTRONOMY ARRAY 🦀🌌|>============================"
        )?;
        writeln!(f, ">dimensions: {}", self.data.shape().len())?;
        write!(f, ">shape: (")?;
        for dim in self.data.shape().iter() {
            write!(f, "{dim},")?;
        }
        write!(f, "\u{0008})\n")?;
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
        writeln!(
            f,
            ">----------------------------------<|METADATA|>---------------------------------"
        )?;
        for (tag, val) in self.meta.iter() {
            writeln!(f, ">\"{tag}\": {val}")?;
        }
        writeln!(
            f,
            ">==============================================================================="
        )
    }
}
