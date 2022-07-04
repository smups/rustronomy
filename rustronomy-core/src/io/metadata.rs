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
//!
//! ## Restricted keys
//! Specific metadata tags are implemented by reserving keys. You cannot create
//! a generic metadata tag using a restricted key. The keys listed here are reserved
//! by the non-generic tags defined in the core crate.
//! - `"author"` used to indicate the author of a data container
//! - `"date"` specifies when a data container was last modified

use std::{
  fmt::{self, Debug, Display, Formatter},
  str::FromStr,
};

#[derive(Debug, Clone)]
/// this is the generic metadata tag consisting of a `String key` and a generic
/// value of type `T`. Parsers will encode these generic tags as such: *non-reserved,
/// generic tags*.
///
/// If you want parsers to use special metadata fields (if the file format
/// that is being parts supports such special fields), you should use one of the
/// non-generic tags in this module. In addition, authors of file-format specific
/// crates are encouraged to define non-generic tags specific to that file-format
/// in addition to the general non-generic tags provided by this module by implementing
/// the `MetaDataTag`.
///
/// If you want to define custom tags yourself, this is the struct to use.
///
/// ## Example usage:
/// ```
///     let my_tag: MetaDataTag<i32> = MetaDataTag::new("my_favourite_number", 42);
/// ```
pub struct GenericMetaDataTag<T>
where
  T: Display + Sized + Send + Sync + FromStr,
{
  pub key: String,
  pub value: T,
}

/// this trait is implemented by containers that store metadata tags.
pub trait MetaDataContainer<T>: priv_hack::PrivDataContainer<T>
where
  T: Display + Sized + Send + Sync + FromStr,
  <T as FromStr>::Err: Debug,
{
  /// adds a generic metadata tag to a data container. Returns an error if the
  /// supplied tag is restricted or if it already exists
  fn add_generic_tag(&mut self, tag: GenericMetaDataTag<T>) -> Result<(), MetaDataErr> {
    //(1) Check if the key is reserved
    if RESERVED_TAGS.contains(&tag.key.as_str()) {
      return Err(MetaDataErr::RestrictedKey(tag.key));
    }

    //(R) return the non-restricted key
    self.add_priv_tag(tag)
  }

  /// removes a generic metadata tag from a data container. Returns an error if
  /// the supplied tag is restricted or if the tag does not exist.
  fn remove_generic_tag(&mut self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr> {
    //(1) Check if the key is reserved
    if RESERVED_TAGS.contains(&key) {
      return Err(MetaDataErr::RestrictedKey(key.to_string()));
    }

    //(R) remove the non-restricted key
    self.remove_priv_tag(key)
  }
}

pub(crate) mod priv_hack {
  use std::{
    fmt::{Debug, Display},
    str::FromStr,
  };

  use super::{GenericMetaDataTag, MetaDataErr};

  pub trait PrivDataContainer<T>
  where
    T: Display + Sized + Send + Sync + FromStr,
    <T as FromStr>::Err: Debug,
  {
    fn add_priv_tag(&mut self, tag: GenericMetaDataTag<T>) -> Result<(), MetaDataErr>;
    fn remove_priv_tag(&mut self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr>;
  }
}

#[derive(Debug)]
/// this enum contains various error types that may occur when modifying a container
/// with metadata tags
pub enum MetaDataErr {
  KeyNotFound(String),
  RestrictedKey(String),
  KeyExists(String),
}

impl std::error::Error for MetaDataErr {}
impl Display for MetaDataErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    use MetaDataErr::*;
    match self {
      KeyNotFound(key) => write!(f, "could not find key \"{key}\""),
      RestrictedKey(key) => write!(
        f,
        "cannot modify tag with key \"{key}\" because it is restricted"
      ),
      KeyExists(key) => write!(
        f,
        "cannot add tag with key \"{key}\" because it already exists"
      ),
    }
  }
}

/// this is a utility trait implemented by all structs that may be used as a
/// metadata tag. It provides the functions necessary to convert between key-value
/// string pairs and the actual metadata tags. Special care should be taken when
/// converting between tags
pub trait MetaDataTag {
  fn as_string_pair(self) -> (String, String);
  /// # panics
  /// this function may panic if the `&str` provided in the value field cannot
  /// be parsed to the desired metadata tag. This can occur if an invalid key-value
  /// pair was inserted in the metadata storage container.
  fn parse_string_pair(key: String, value: &str) -> Self;
}

impl<T> MetaDataTag for GenericMetaDataTag<T>
where
  T: Display + Sized + Send + Sync + FromStr,
  <T as FromStr>::Err: Debug,
{
  fn as_string_pair(self) -> (String, String) {
    let key = self.key;
    let value = T::to_string(&self.value);
    (key, value)
  }

  fn parse_string_pair(key: String, value: &str) -> GenericMetaDataTag<T> {
    //This should never panic since the value field may only be filled by
    //calling .to_string() on T
    let value = T::from_str(value)
      .expect("Could not parse tag data. Are you sure you provided the right key?");
    GenericMetaDataTag { key, value }
  }
}

impl<T> Display for GenericMetaDataTag<T>
where
  T: Display + Send + Sync + FromStr,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "<Generic Tag> \"{}\"={}",
      self.key,
      self.value.to_string()
    )
  }
}

/// utility macro to easily create restricted tags from tuple structs
macro_rules! impl_tag {
  ($tag_name:ty, $key:ident, $fmt:literal) => {
    impl MetaDataTag for $tag_name {
      fn as_string_pair(self) -> (String, String) {
        ($key.to_string(), self.0.to_string())
      }
      fn parse_string_pair(_: String, value: &str) -> Self {
        Self(value.to_string())
      }
    }

    impl Display for $tag_name {
      fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, $fmt, self.0)
      }
    }
  };
}

/*
  List of all the restricted tags. If you add a restricted tag, make sure to:
    (1) add it to the RESERVED_TAGS array
    (2) run the impl_tag! macro
    (3) specify the restricted key in the struct-level docs
    (4) add the restricted key (with description) to the restricted keys list in
        the module-level docs.
  Thx!
*/
/// these tags are reserved for special use-cases and may not be used as generic tags
pub const RESERVED_TAGS: [&str; 2] = [AUTHOR, DATE];

#[derive(Debug, Clone)]
/// this restricted tag specifies the author(s) of the data contained within
/// the data container. It corresponds to the restricted `author` key.
pub struct AuthorTag(pub String);
pub(crate) const AUTHOR: &str = "author";
impl_tag!(AuthorTag, AUTHOR, "<Author Tag> \"author\"={}");

#[derive(Debug, Clone)]
/// this restricted tag specifies the ISO date when the data container was last
/// modified. It corresponds to the restricted `date` key.
pub struct DateTag(pub String);
pub(crate) const DATE: &str = "date";
impl_tag!(DateTag, DATE, "<Date Tag> \"last modified\"={}");
