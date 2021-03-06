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
  priv_hack::PrivDataContainer, GenericMetaDataTag, MetaDataContainer, MetaDataErr, MetaDataTag,
};

#[derive(Debug, Clone)]
pub struct DataArray<T: Num> {
  data: Array<T, IxDyn>,
  meta: HashMap<String, String>,
}

impl<U: Num, T> PrivDataContainer<T> for DataArray<U>
where
  T: Display + Sized + Send + Sync + FromStr,
  <T as FromStr>::Err: Debug,
{
  fn set_priv_tag(&mut self, tag: impl MetaDataTag) -> Result<(), MetaDataErr> {
    //(1) Check if the key already exists
    if self.meta.contains_key(tag.get_key()) {
      return Err(MetaDataErr::KeyExists(tag.get_key().to_string()));
    }

    //(2) we're good -> add the key
    let (key, value) = tag.as_string_pair();
    self.meta.insert(key, value);
    Ok(())
  }

  fn get_priv_tag(&self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr> {
    //(1) Check if the key does not exists
    if !self.meta.contains_key(key) {
      return Err(MetaDataErr::KeyNotFound(key.to_string()));
    }

    //(2) we're good -> return a copy of the key
    let value = self.meta.get(key).unwrap();
    Ok(GenericMetaDataTag::<T>::parse_string_pair(key.to_string(), value))
  }

  fn remove_priv_tag(&mut self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr> {
    //(1) Check if the key does not exists
    if !self.meta.contains_key(key) {
      return Err(MetaDataErr::KeyNotFound(key.to_string()));
    }

    //(2) we're good -> remove the key
    let (key, value) = self.meta.remove_entry(key).unwrap();
    Ok(GenericMetaDataTag::<T>::parse_string_pair(key, &value))
  }
}

impl<U: Num + Dimension, T> MetaDataContainer<T> for DataArray<U>
where
  T: Display + Sized + Send + Sync + FromStr,
  <T as FromStr>::Err: Debug,
{
}

impl<U: Num> DataArray<U> {
  /// constructs new array (without metadata tags) from 2D ndarray
  pub fn new(data: Array<U, IxDyn>) -> Self {
    DataArray { data, meta: HashMap::new() }
  }

  #[inline]
  /// returns array data throwing away the metadata tags
  pub fn data(self) -> Array<U, IxDyn> {
    self.data
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
    writeln!(f, ">size: {}", super::fmt_byte_size(self.data.len() * mem::size_of::<U>()))?;
    writeln!(f, ">datatype: {}", any::type_name::<U>())?;
    writeln!(
      f,
      ">----------------------------------<|METADATA|>---------------------------------"
    )?;
    for (tag, val) in self.meta.iter() {
      writeln!(f, ">\"{tag}\": {val}")?;
    }
    writeln!(f, ">===============================================================================")
  }
}
