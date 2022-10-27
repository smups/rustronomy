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
//! Metadata may be added to a data container in the form of key-value pairs.
//! To provide interoperability between storage formats, certain keys are restricted
//! and may only be added via specialised functions implemented by the metadata
//! container. All relevant types and traits for metadata tags and containers are
//! implemented in this module.
//!
//! A list of restricted keys for metadata tags can be found in the documentation
//! of the `MetaDataContainer` trait.

use std::{fmt::Debug, str::FromStr};

use self::private_container::PrivContainer;

pub use super::tags::MetaDataContainer;

/// The `MetaDataTag` trait specifies methods that must be implemented by a type
/// to be used as a metadata tag. This trait may be implemented by the user to
/// create custom metadata tags.
///
/// The `MetaDataTag` trait specifies the key corresponding to the metadata tag,
/// and the datatype of the value contained by it. In additon, the type implementing
/// `MetaDataTag` must be freely transformable from and into its inner type AND
/// a string.
pub trait MetaDataTag: Sized + From<Self::ValueType> + Into<Self::ValueType>
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
#[non_exhaustive]
/// This error type describes all errors that may occur when parsing entries in
/// the metadata container to `impl MetaDataTag`s. It is marked `#[non_exhaustive]`.
pub enum TagError {
  TagParseError(String),
  TagNotFoundError(String),
  RestrictedTagError(String),
}
impl std::fmt::Display for TagError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use TagError::*;
    match self {
      TagParseError(parse_error) => {
        write!(f, "could not parse data found as a metadata tag. Reason:\"{parse_error}\"")
      }
      TagNotFoundError(tag) => {
        write!(f, "the tag \"{tag}\" is not specified by this metadata container")
      }
      RestrictedTagError(tag) => {
        write!(f, "the tag\"{tag}\" is restricted and cannot be accessed directly")
      }
    }
  }
}
impl std::error::Error for TagError {}

pub(crate) mod private_container {
  use super::{MetaDataTag, TagError};
  use std::str::FromStr;

  pub trait PrivContainer {
    //(1) funcs to remove tags
    fn remove_all_tags(&mut self) -> Vec<(String, String)>;
    fn remove_tag_str(&mut self, key: &str) -> Option<String>;
    fn remove_tag<T>(&mut self) -> Result<T, TagError>
    where
      T: MetaDataTag,
      <<T as MetaDataTag>::ValueType as FromStr>::Err: std::fmt::Debug,
    {
      match self.remove_tag_str(T::KEY) {
        Some(string) => match string.parse::<T::ValueType>() {
          Ok(value) => Ok(value.into()),
          Err(err) => Err(TagError::TagParseError(format!("{err:?}"))),
        },
        None => Err(TagError::TagNotFoundError(T::KEY.to_string())),
      }
    }

    //(2) funcs to insert tags
    fn insert_all_tags(&mut self, tags: &[(String, String)]) {
      tags.iter().for_each(|(key, value)| {
        self.insert_tag_str(value, key);
      })
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

    fn has_tag_str(&self, key: &str) -> bool;
    fn has_tag<T>(&self) -> bool
    where
      T: MetaDataTag,
      <<T as MetaDataTag>::ValueType as FromStr>::Err: std::fmt::Debug,
    {
      return self.has_tag_str(T::KEY);
    }

    //(3) funcs to clone tags
    fn clone_tags(&self) -> Vec<(String, String)>;
  }
}

/// The `PubContainer` trait contains methods to modify and parse generic
/// (non-reserved) metadata tags. These methods will return an error if the key
/// specified in the function call is one of the reserved keys. See the
/// `MetaDataContainer` type for a list of reserved tags.
pub trait PubContainer: self::private_container::PrivContainer {
  fn clone_metadata(&self) -> super::meta_only::MetaOnly {
    let mut meta = super::meta_only::MetaOnly::new();
    meta.insert_all_tags(&self.clone_tags());
    return meta;
  }

  fn has_generic_tag(&self, key: &str) -> bool {
    self.has_tag_str(key)
  }

  fn remove_generic_tag<T>(&mut self, key: &str) -> Result<T, TagError>
  where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
  {
    if super::tags::RESTRICTED_TAGS.contains(&key) {
      return Err(TagError::RestrictedTagError(key.to_string()));
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
      return Err(TagError::RestrictedTagError(key.to_string()));
    }
    match self.insert_tag_str(&format!("{val:?}"), &key) {
      Some(string) => match string.parse::<T>() {
        Ok(value) => Ok(Some(value)),
        Err(err) => Err(TagError::TagParseError(format!("{err:?}"))),
      },
      None => Ok(None),
    }
  }
}
