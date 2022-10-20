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

use super::metadata::{private_container::PrivContainer, MetaDataContainer, PubContainer};

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

impl<U: Num> TryFrom<super::DataArray<U>> for Image<U> {
  type Error = ConversionErr;

  fn try_from(array: super::DataArray<U>) -> Result<Self, Self::Error> {
    use ndarray::Dimension;

    //First we check if the array is indeed 2d
    if (&array.data).dim().size() != 2 {
      return Err(ConversionErr((&array.data).dim().size()));
    }

    //We're ok, so we can just return the new image
    Ok(Image { data: array.data.into_dimensionality::<ndarray::Ix2>().unwrap(), meta: array.meta })
  }
}

#[derive(Debug)]
pub struct ConversionErr(usize);
impl std::fmt::Display for ConversionErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "cannot convert DataArray with {} axes to an Image. Number of axes must be exactly two.",
      self.0
    )
  }
}
impl std::error::Error for ConversionErr {}
