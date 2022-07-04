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
  fmt::{Debug, Display},
  str::FromStr,
};

use super::metadata::{
  priv_hack::PrivDataContainer, GenericMetaDataTag, MetaDataContainer, MetaDataErr, MetaDataTag,
};

#[derive(Debug, Clone)]
pub struct Table {
  data: Vec<Col>,
  lookup_tbl: HashMap<String, usize>,
  meta: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum Col {
  Integer(Vec<i64>),
  Float(Vec<f64>),
  Text(String),
}

impl<T> PrivDataContainer<T> for Table
where
  T: Display + Sized + Send + Sync + FromStr,
  <T as FromStr>::Err: Debug,
{
  fn add_priv_tag(&mut self, tag: GenericMetaDataTag<T>) -> Result<(), MetaDataErr> {
    //(1) Check if the key already exists
    if self.meta.contains_key(&tag.key) {
      return Err(MetaDataErr::KeyExists(tag.key));
    }

    //(2) we're good -> add the key
    let (key, value) = tag.as_string_pair();
    self.meta.insert(key, value);
    Ok(())
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
