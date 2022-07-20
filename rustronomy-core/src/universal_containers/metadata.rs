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
//! ## reserved keys
//! Specific metadata tags are implemented by reserving keys. You cannot create
//! a generic metadata tag using a reserved key. The keys listed here are reserved
//! by the non-generic tags defined in the core crate.
//! - `"author"` used to indicate the author of a data container
//! - `"date"` specifies when the data was collected
//! - `"last_modified"` specifies when the data was last modified
//! - `"object"` specifies the object(s) observed to produce the data
//! - `"organisation"` specifies the organisation(s) responsible for producing
//! the data
//! - `"telescope"` specifies the telescope used to produce the data
//! - `"instrument"` specifies the instrument used to produce the data.
//! This keyword is to be used in conjunction with the `telescope` keyword
//! - `"reference"` specifies a reference to a publication accompanying the data.
//! It is recommended to use a [DOI](https://doi.org) or
//! [ADS](https://ads.harvard.edu) format.
//! - `"exposure_time"` specifies the exposure time in seconds of the image

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
  /// supplied tag is reserved. If the tag already exists, the old value will
  /// be overridden.
  fn set_generic_tag(&mut self, tag: GenericMetaDataTag<T>) -> Result<(), MetaDataErr> {
    //(1) Check if the key is reserved
    if RESERVED_TAGS.contains(&tag.key.as_str()) {
      return Err(MetaDataErr::ReservedKey(tag.key));
    }

    //(R) return the non-reserved key
    self.set_priv_tag(tag)
  }

  /// returns a copy of the tag associated with the supplied key, if it exists.
  /// reserved tags cannot be changed manually, but can be copied without any
  /// issues. Therefore, you may supply a restricted key to this function.
  fn get_tag(&self, key: &str) -> Option<GenericMetaDataTag<T>> {
    if let Ok(tag) = self.get_priv_tag(key) {
      Some(tag)
    } else {
      None
    }
  }

  /// removes a generic metadata tag from a data container. Returns an error if
  /// the supplied tag is reserved or if the tag does not exist.
  fn remove_generic_tag(&mut self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr> {
    //(1) Check if the key is reserved
    if RESERVED_TAGS.contains(&key) {
      return Err(MetaDataErr::ReservedKey(key.to_string()));
    }

    //(R) remove the non-reserved key
    self.remove_priv_tag(key)
  }

  /*
    Metadata containers should also be able to be tagged with non-generic tags
  */
  fn set_author(&mut self, author: &str) -> Result<(), MetaDataErr> {
    self.set_priv_tag(AuthorTag(author.to_string()))
  }
  fn get_author(&self) -> Option<String> {
    if let Some(tag) = self.get_tag(AUTHOR) {
      Some(tag.value.to_string())
    } else {
      None
    }
  }
  fn remove_author(&mut self) -> Result<(), MetaDataErr> {
    self.remove_priv_tag(AUTHOR)?;
    Ok(())
  }
}

pub(crate) mod priv_hack {
  use std::{
    fmt::{Debug, Display},
    str::FromStr,
  };

  use super::{GenericMetaDataTag, MetaDataErr, MetaDataTag};

  pub trait PrivDataContainer<T>
  where
    T: Display + Sized + Send + Sync + FromStr,
    <T as FromStr>::Err: Debug,
  {
    fn set_priv_tag(&mut self, tag: impl MetaDataTag) -> Result<(), MetaDataErr>;
    fn get_priv_tag(&self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr>;
    fn remove_priv_tag(&mut self, key: &str) -> Result<GenericMetaDataTag<T>, MetaDataErr>;
  }
}

#[derive(Debug)]
#[non_exhaustive]
/// this enum contains various error types that may occur when modifying a container
/// with metadata tags
pub enum MetaDataErr {
  KeyNotFound(String),
  ReservedKey(String),
  KeyExists(String),
}

impl std::error::Error for MetaDataErr {}
impl Display for MetaDataErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    use MetaDataErr::*;
    match self {
      KeyNotFound(key) => write!(f, "could not find key \"{key}\""),
      ReservedKey(key) => write!(f, "cannot modify tag with key \"{key}\" because it is reserved"),
      KeyExists(key) => write!(f, "cannot add tag with key \"{key}\" because it already exists"),
    }
  }
}

/// this is a utility trait implemented by all structs that may be used as a
/// metadata tag. It provides the functions necessary to convert between key-value
/// string pairs and the actual metadata tags. Special care should be taken when
/// converting between tags
pub trait MetaDataTag {
  fn get_key(&self) -> &str;
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
  fn get_key(&self) -> &str {
    &self.key
  }

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
    write!(f, "<GenericTag> \"{}\"={}", self.key, self.value.to_string())
  }
}

/// utility macro to easily create reserved tags from tuple structs
macro_rules! impl_tag {
  ($tag_name:path, $key:ident, $fmt:literal) => {
    impl MetaDataTag for $tag_name {
      fn get_key(&self) -> &str {
        $key
      }
      fn as_string_pair(self) -> (String, String) {
        ($key.to_string(), self.0.to_string())
      }
      fn parse_string_pair(_: String, value: &str) -> Self {
        $tag_name(value.to_string())
      }
    }

    impl Display for $tag_name {
      fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, $fmt, self.0)
      }
    }

    impl From<$tag_name> for GenericMetaDataTag<String> {
      fn from(reserved_tag: $tag_name) -> GenericMetaDataTag<String> {
        GenericMetaDataTag { key: $key.to_string(), value: reserved_tag.0 }
      }
    }

    impl From<String> for $tag_name {
      fn from(inner: String) -> $tag_name {
        $tag_name(inner)
      }
    }
  };
}

/*
  List of all the reserved tags. If you add a reserved tag, make sure to:
    (1) add it to the RESERVED_TAGS array
    (2) run the impl_tag! macro
    (3) specify the reserved key in the struct-level docs
    (4) add the reserved key (with description) to the reserved keys list in
        the module-level docs.
  Thx!
*/
/// these tags are reserved for special use-cases and may not be used as generic tags
pub const RESERVED_TAGS: [&str; 9] = [
  AUTHOR, DATE, LAST_MODIFIED, OBJECT, ORGANISATION, TELESCOPE, INSTRUMENT,
  REFERENCE, EXPOSURE_TIME
];

#[derive(Debug, Clone)]
/// this reserved tag specifies the author(s) of the data contained within
/// the data container. It corresponds to the reserved `author` key.
pub struct AuthorTag(pub String);
pub(crate) const AUTHOR: &str = "author";
impl_tag!(AuthorTag, AUTHOR, "<RESERVED> \"author\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies the ISO date when the data held in the data
/// container was collected. It corresponds to the reserved `date` key.
pub struct DateTag(pub String);
pub(crate) const DATE: &str = "date";
impl_tag!(DateTag, DATE, "<RESERVED> \"data creation date\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies the ISO date when the data container was last
/// modified. It corresponds to the reserved `last_modified` key.
pub struct LastModifiedTag(pub String);
pub(crate) const LAST_MODIFIED: &str = "last_modified";
impl_tag!(LastModifiedTag, LAST_MODIFIED, "<RESERVED> \"last modified\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies the (astronomical) object that was observed to
/// produce the data in this container. It corresponds to the `object` key.
pub struct ObjectTag(pub String);
pub(crate) const OBJECT: &str = "object";
impl_tag!(ObjectTag, OBJECT, "<RESERVED> \"observed object\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies the organisation responsible for producing the
/// data in this container. It corresponds to the reserved `organisation` key.
pub struct OrgTag(pub String);
pub(crate) const ORGANISATION: &str = "organisation";
impl_tag!(OrgTag, ORGANISATION, "<RESERVED> \"organisation\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies the telescope used in producing the data 
/// in this container. It corresponds to the reserved `telescope` key.
pub struct TelescopeTag(pub String);
pub(crate) const TELESCOPE: &str = "telescope";
impl_tag!(TelescopeTag, TELESCOPE, "<RESERVED> \"telescope\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies the instrument used in producing the data 
/// in this container. It corresponds to the reserved `instrument` key.
pub struct InstrumentTag(pub String);
pub(crate) const INSTRUMENT: &str = "instrument";
impl_tag!(InstrumentTag, INSTRUMENT, "<RESERVED> \"instrument\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies a reference to a publication relevant to the 
/// datacontainer. It corresponds to the reserved `reference` key.
pub struct ReferenceTag(pub String);
pub(crate) const REFERENCE: &str = "reference";
impl_tag!(ReferenceTag, REFERENCE, "<RESERVED> \"reference publication\"={}");

#[derive(Debug, Clone)]
/// this reserved tag specifies a reference to a publication relevant to the 
/// datacontainer. It corresponds to the reserved `reference` key.
pub struct ExpTimeTag(pub u64); //<- requires manual implementation :c
pub(crate) const EXPOSURE_TIME: &str = "exposure_time";

impl MetaDataTag for ExpTimeTag {
  fn get_key(&self) -> &str { EXPOSURE_TIME }

  fn as_string_pair(self) -> (String, String) {
    (EXPOSURE_TIME.to_string(), self.0.to_string())
  }

  fn parse_string_pair(_: String, value: &str) -> Self {
    ExpTimeTag(value.parse().unwrap()) //<- panics if value is invalid, too bad
  }
}

impl From<u64> for ExpTimeTag {
  fn from(exposure_time: u64) -> Self {
    ExpTimeTag(exposure_time)
  }
}

impl From<ExpTimeTag> for GenericMetaDataTag<u64> {
  fn from(tag: ExpTimeTag) -> Self {
    GenericMetaDataTag { key: EXPOSURE_TIME.to_string(), value: tag.0 }
  }
}