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

//! this module provides a generic `MetaDataTag<T>` used to add metadata to data
//! containers. Some file formats have special metadata fields. In this case,
//! you may want to use one of the specific metadata tags provided down below.
//!
//! ## Tag storage
//! Tags are meant to be stored in a `HashMap<String, String>`-like format. This
//! means that no two metadata tags may have the same key. Objects that store
//! metadata should therefore not allow the user to store the same key twice. To
//! facilitate easy conversion between tags and their string representations, all
//! metadata tags should implement `MetaDataTag` trait.

use std::{fmt::Debug, str::FromStr};

pub use super::tags::MetaDataContainer;

pub trait MetaDataTag: Sized + From<Self::ValueType>
where
  Self::ValueType: FromStr,
  <Self::ValueType as FromStr>::Err: std::fmt::Debug,
{
  const KEY: &'static str;
  type ValueType;

  fn new(val: Self::ValueType) -> Self {
    val.into()
  }
  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err>;
  fn to_string(&self) -> String;
}

#[derive(Debug)]
pub enum TagError {
  TagParseError(String),
  TagNotFoundError(String),
  RestrictedTagError(String),
}

pub(crate) mod private_container {
  use super::{MetaDataTag, TagError};
  use std::str::FromStr;

  pub trait PrivContainer {
    fn remove_tag_str(&mut self, key: &str) -> Option<String>;

    fn remove_tag<T>(&mut self, key: &str) -> Result<T, TagError>
    where
      T: MetaDataTag,
      <<T as MetaDataTag>::ValueType as FromStr>::Err: std::fmt::Debug,
    {
      match self.remove_tag_str(key) {
        Some(string) => match string.parse::<T::ValueType>() {
          Ok(value) => Ok(value.into()),
          Err(err) => Err(TagError::TagParseError(format!("{err:?}"))),
        },
        None => Err(TagError::TagNotFoundError(key.to_string())),
      }
    }

    fn insert_tag_str(&mut self, parsed_tag: &str, key: &str) -> Option<String>;

    fn insert_tag<T>(&mut self, tag: T) -> Result<Option<T>, TagError>
    where
      T: MetaDataTag,
      <<T as MetaDataTag>::ValueType as FromStr>::Err: std::fmt::Debug,
    {
      match self.insert_tag_str(&tag.to_string(), T::KEY) {
        Some(string) => match string.parse::<T::ValueType>() {
          Ok(tag) => Ok(Some(tag.into())),
          Err(err) => Err(TagError::TagParseError(format!("{err:?}"))),
        },
        None => Ok(None),
      }
    }
  }
}

pub trait PubContainer: self::private_container::PrivContainer {

  fn remove_generic_tag<T>(&mut self, key: &str) -> Result<T, TagError>
  where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
  {
    if super::tags::RESTRICTED_TAGS.contains(&key) {
      return Err(TagError::RestrictedTagError(key.to_string()))
    }
    match self.remove_tag_str(key) {
      Some(string) => match string.parse::<T>() {
        Ok(value) => Ok(value),
        Err(err) => Err(TagError::TagParseError(format!("{err:?}"))),
      },
      None => Err(TagError::TagNotFoundError(key.to_string())),
    }
  }

  fn insert_generic_tag<T>(&mut self, key: &str, val: T) -> Result<Option<T>, TagError>
  where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
  {
    if super::tags::RESTRICTED_TAGS.contains(&key) {
      return Err(TagError::RestrictedTagError(key.to_string()))
    }
    match self.insert_tag_str(&key, &format!("{val:?}")) {
      Some(string) => match string.parse::<T>() {
        Ok(value) => Ok(Some(value)),
        Err(err) => Err(TagError::TagParseError(format!("{err:?}"))),
      },
      None => Err(TagError::TagNotFoundError(key.to_string())),
    }
  }
}