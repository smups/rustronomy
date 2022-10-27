/*
  Copyright© 2022 Raúl Wolters(1)

  This file is part of rustronomy-core.

  rustronomy is free software: you can redistribute it and/or modify it under
  the terms of the European Union Public License version 1.2 or later, as
  published by the European Commission.

  rustronomy is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE. See the European Union Public License for more details.

  You should have received a copy of the EUPL in an/all official language(s) of
  the European Union along with rustronomy.  If not, see 
  <https://ec.europa.eu/info/european-union-public-licence_en/>.

  (1) Resident of the Kingdom of the Netherlands; agreement between licensor and
  licensee subject to Dutch law as per article 15 of the EUPL.
*/

//! This module specifies a generic array container. In contrast to an Image, an
//! Array may have any number of axes. This comes at some allocation cost, see the
//! ndarray docs for details.

use std::{
  any,
  collections::HashMap,
  fmt::{self, Debug, Display, Formatter},
  mem,
};

use ndarray::{Array, Dimension, IxDyn};
use num_traits::Num;

use super::metadata::{private_container::PrivContainer, MetaDataContainer, PubContainer};

#[derive(Debug, Clone)]
pub struct DataArray<T: Num> {
  pub(crate) data: Array<T, IxDyn>,
  pub(crate) meta: HashMap<String, String>,
}

impl<U: Num> PrivContainer for DataArray<U> {
  fn remove_tag_str(&mut self, key: &str) -> Option<String> {
    self.meta.remove(key)
  }

  fn remove_all_tags(&mut self) -> Vec<(String, String)> {
    self.meta.drain().collect()
  }

  fn insert_tag_str(&mut self, parsed_tag: &str, key: &str) -> Option<String> {
    self.meta.insert(key.to_string(), parsed_tag.to_string())
  }

  fn has_tag_str(&self, key: &str) -> bool {
    self.meta.contains_key(key)
  }

  fn clone_tags(&self) -> Vec<(String, String)> {
    self.meta.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }
}

impl<U: Num> PubContainer for DataArray<U> {}
impl<U: Num> MetaDataContainer for DataArray<U> {}

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

impl<U: Num> From<super::Image<U>> for DataArray<U> {
  fn from(img: super::Image<U>) -> Self {
    DataArray { data: img.data.into_dimensionality().unwrap(), meta: img.meta }
  }
}
