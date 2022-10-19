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
//! `NDArray` of some generic numeric type and a list of metadata tags.

use std::{
  any,
  collections::HashMap,
  fmt::{self, Debug, Display, Formatter},
  mem,
};

use ndarray::Array2;
use num_traits::Num;

use super::metadata::{
  private_container::PrivContainer, MetaDataContainer, PubContainer,
};

#[derive(Debug, Clone)]
/// Image container consisting of a 2D array of type `T` and metadata tags
pub struct Image<T: Num> {
  pub(crate) data: Array2<T>,
  pub(crate) meta: HashMap<String, String>,
}

impl<U: Num> PrivContainer for Image<U> {
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
}

impl<U: Num> PubContainer for Image<U> {}
impl<U: Num> MetaDataContainer for Image<U> {}

impl<U: Num> Image<U> {
  /// constructs new image (without metadata tags) from 2D ndarray
  pub fn new(data: Array2<U>) -> Self {
    Image { data, meta: HashMap::new() }
  }

  /// returns image data throwing away the metadata tags
  pub fn data(self) -> Array2<U> {
    self.data
  }
}

impl<U: Num> Display for Image<U> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(
      f,
      ">============================<|RUSTRONOMY IMAGE 🦀🌌|>============================"
    )?;
    writeln!(f, ">shape: ({}x{})", self.data.shape()[0], self.data.shape()[1])?;
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
