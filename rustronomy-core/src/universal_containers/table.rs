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

//! This module specifies a table container. Tables consist of named columns
//! that hold a bunch of values of the same type. Tables should not be used to
//! store large amounts of data.
//!
//! Three column types are supported:
//! - `Integer` always a `Vec<i64>`
//! - `Float` always a `Vec<f64>`
//! - `Text` always a `Vec<String>`
//!
//! Columns can be accessed either through their index or through their name, if
//! one has been supplied.

use std::{
  collections::HashMap,
  fmt::{self, Debug, Display, Formatter},
  mem,
  str::FromStr,
};

use indexmap::IndexMap;

use super::metadata::{
  priv_hack::PrivDataContainer, GenericMetaDataTag, MetaDataContainer, MetaDataErr, MetaDataTag,
};

#[derive(Debug, Clone)]
/// the table data container. Consists of named columns and metadata tags. See
/// the module-level documentation for more details.
pub struct Table {
  //Indexmap to provide easy iteration
  data: IndexMap<String, Col>,
  meta: HashMap<String, String>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
/// columns are the constituent parts of tables. They consist of vectors holding
/// elements of the same type. Types are differentiated via the variants of this
/// enum.
/// 
/// Right now, 3 variants are supported:
/// - `Integer` always a `Vec<i64>`
/// - `Float` always a `Vec<f64>`
/// - `Text` always a `Vec<String>`
/// In the future, more variants may be added as necessary. As such, this enum
/// is marked as `#[non_exhaustive]`.
pub enum Col {
  Integer(Vec<i64>),
  Float(Vec<f64>),
  Text(Vec<String>),
}

impl Col {
  #[inline]
  /// returns the number of elements in the column
  pub fn len(&self) -> usize {
    use Col::*;
    match self {
      Integer(vec) => vec.len(),
      Float(vec) => vec.len(),
      Text(vec) => vec.len(),
    }
  }

  /// returns the total size of the column in bytes
  pub fn size(&self) -> usize {
    //total size = discriminant + usize + the size of the underlying vector
    mem::size_of::<Self>()
      + self.len()
        * match self {
          Col::Integer(_) => mem::size_of::<i64>(),
          Col::Float(_) => mem::size_of::<f64>(),
          Col::Text(vec) => vec.iter().map(|string| string.bytes().len()).sum(),
        }
  }
}

impl<T> PrivDataContainer<T> for Table
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

impl<T> MetaDataContainer<T> for Table
where
  T: Display + Sized + Send + Sync + FromStr,
  <T as FromStr>::Err: Debug,
{
}

impl Table {
  /// creates an empty table without metadata
  pub fn new() -> Table {
    Table { data: IndexMap::new(), meta: HashMap::new() }
  }

  #[inline]
  /// adds column to the table and gives it a name, if specified. If a column
  /// with the same name already exists, it will be overridden.
  pub fn set_col(&mut self, col_name: &str, col: Col) {
    self.data.insert(col_name.to_string(), col);
  }

  #[inline]
  /// returns reference to the column with the name `col_name` if one exists,
  /// `None` otherwise.
  pub fn get_col(&self, col_name: &str) -> Option<&Col> {
    self.data.get(col_name)
  }

  #[inline]
  /// returns mutable reference to the column with the name `col_name` if one
  /// exists, `None` otherwise.
  pub fn get_col_mut(&mut self, col_name: &str) -> Option<&mut Col> {
    self.data.get_mut(col_name)
  }

  #[inline]
  /// removes column with the name `col_name` if one exists and returns it.
  /// Returns `None` if no column named `col_name` exists.
  pub fn remove_col(&mut self, col_name: &str) -> Option<Col> {
    self.data.remove(col_name)
  }

  #[inline]
  /// returns vec of (column-name, column) pairs, discarding the metadata tags
  pub fn data(self) -> Vec<(String, Col)> {
    self.data.into_iter().collect()
  }

  #[inline]
  /// returns vec of columns discarding column names and metadata tags
  pub fn data_unnamed(self) -> Vec<Col> {
    self.data.into_values().collect()
  }
}

impl Display for Table {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(
      f,
      ">=============================<|RUSTRONOMY TABLE 🦀🌌|>============================"
    )?;
    writeln!(f, ">number of colums: {}", self.data.len())?;
    write!(f, ">shape: (")?;
    for (_name, val) in self.data.iter() {
      write!(f, "{},", val.len())?;
    }
    write!(f, "\u{0008})\n")?;
    write!(
      f,
      ">total size: {}",
      super::fmt_byte_size(self.data.iter().map(|(_name, col)| col.size()).sum())
    )?;
    writeln!(
      f,
      ">-----------------------------------<|COLUMNS|>----------------------------------"
    )?;
    for (col_name, col) in self.data.iter() {
      writeln!(f, ">[{}]", col_name)?;
      writeln!(f, ">    number of elements: {}", col.len())?;
      writeln!(f, ">    size: {}", super::fmt_byte_size(col.size()))?;
    }
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
