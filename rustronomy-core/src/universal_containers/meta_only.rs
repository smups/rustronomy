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

//! This module provides a metadata-only container called `MetaOnly`. It can be
//! used by other libraries to construct additional containers, or to directly
//! pass metadata.

use std::{any::TypeId, collections::HashMap};

use crate::meta::{MetaDataContainer, MetaTag};

#[derive(Clone, Default)]
/// Basic hashmap based metadata container, containing only metadata.
///
/// # Utility traits
/// `MetaOnly` implements `Clone`, `Debug`, `Default`, `Display`, `Send` and `Sync`
pub struct MetaOnly {
  string_tags: HashMap<String, String>,
  typed_tags: HashMap<TypeId, Box<dyn MetaTag>>,
}

impl MetaOnly {
  /// Constructs empty `MetaOnly` container
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns an `Iterator` over all strongly typed metadata tags in the container.
  pub fn iter_typed_tags(&self) -> impl Iterator<Item = &dyn MetaTag> {
    self.typed_tags.iter().map(|(_type_id, opaque)| opaque.as_ref())
  }

  /// Returns an `Iterator` over all string key-value metadata pairs in the container.
  pub fn iter_string_kv(&self) -> impl Iterator<Item = (&str, &str)> {
    self.string_tags.iter().map(|(k, v)| (k as &str, v as &str))
  }

  /// Returns an `Iterator` of nicely formatted string key-value metadata pairs in the container.
  ///
  /// The format looks like `[Key]: "Value"`
  pub fn iter_string_fmt(&self) -> impl Iterator<Item = String> + '_ {
    self.iter_string_kv().map(|(k, v)| format!("[{k}]: \"{v}\""))
  }
}

impl PartialEq for MetaOnly {
  fn eq(&self, other: &Self) -> bool {
    self.iter_string_kv().all(|(key, _value)| other.contains_string_tag(key))
      && self.iter_typed_tags().all(|tag| other.contains_type_id(&tag.type_id()))
  }
}

impl std::fmt::Debug for MetaOnly {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    writeln!(f, "================================<TYPED METADATA>================================")?;
    if self.has_typed_metadata() {
      for strong in self.iter_typed_tags() {
        writeln!(f, "{strong}")?;
      }
    } else {
      #[cfg_attr(rustfmt, rustfmt_skip)]
      writeln!(f, "                                  (not present)                                 ")?;
    }
    #[cfg_attr(rustfmt, rustfmt_skip)]
    writeln!(f, "===============================<UNTYPED METADATA>===============================")?;
    if self.has_string_metadata() {
      for weak in self.iter_string_fmt() {
        writeln!(f, "{weak}")?;
      }
    } else {
      #[cfg_attr(rustfmt, rustfmt_skip)]
      writeln!(f, "                                  (not present)                                 ")?;
    }
    Ok(())
  }
}

impl MetaDataContainer for MetaOnly {
  fn insert_tag<T: MetaTag + Clone>(&mut self, tag: &T) -> Option<T> {
    self.typed_tags.insert(tag.type_id(), Box::new(tag.clone()))?.as_ref().downcast()
  }

  fn contains_tag<T: MetaTag + Clone>(&self) -> bool {
    self.typed_tags.contains_key(&TypeId::of::<T>())
  }

  fn contains_type_id(&self, type_id: &TypeId) -> bool {
    self.typed_tags.contains_key(type_id)
  }

  fn get_tag<T: MetaTag + Clone>(&self) -> Option<&T> {
    self.typed_tags.get(&TypeId::of::<T>())?.downcast_ref()
  }

  fn get_tag_mut<T: MetaTag + Clone>(&mut self) -> Option<&mut T> {
    self.typed_tags.get_mut(&TypeId::of::<T>())?.downcast_mut()
  }

  fn remove_tag<T: MetaTag + Clone>(&mut self) -> Option<T> {
    self.typed_tags.remove(&TypeId::of::<T>())?.downcast()
  }

  fn has_typed_metadata(&self) -> bool {
    !self.typed_tags.is_empty()
  }

  fn insert_string_tag(&mut self, key: &str, value: &str) -> Option<String> {
    self.string_tags.insert(key.to_string(), value.to_string())
  }

  fn contains_string_tag(&self, key: &str) -> bool {
    self.string_tags.contains_key(key)
  }

  fn get_string_tag(&self, key: &str) -> Option<&str> {
    self.string_tags.get(key).and_then(|x| Some(x as &str))
  }

  fn get_string_tag_mut(&mut self, key: &str) -> Option<&mut String> {
    self.string_tags.get_mut(key)
  }

  fn remove_string_tag(&mut self, key: &str) -> Option<String> {
    self.string_tags.remove(key)
  }

  fn has_string_metadata(&self) -> bool {
    !self.string_tags.is_empty()
  }
}

#[test]
fn test_string_meta_insert_remove() {
  let mut meta = MetaOnly::new();
  let (key, value) = ("test", "value");
  meta.insert_string_tag(key, value);
  assert_eq!(value, meta.remove_string_tag(key).unwrap())
}

#[test]
fn test_partialeq_string_meta() {
  let mut meta1 = MetaOnly::new();
  let mut meta2 = MetaOnly::new();
  assert_eq!(&meta1, &meta2);

  let (key, value) = ("test", "value");
  meta1.insert_string_tag(key, value);
  assert_ne!(&meta1, &meta2);

  meta2.insert_string_tag(key, value);
  assert_eq!(&meta1, &meta2);

  meta2.remove_string_tag(key);
  assert_ne!(&meta1, &meta2);

  meta1.remove_string_tag(key);
  assert_eq!(&meta1, &meta2);
}

#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
struct DummyTag(pub String);
impl MetaTag for DummyTag {}
impl std::fmt::Display for DummyTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "[DummyTag]: \"{}\"", self.0)
  }
}

#[test]
fn test_dummy_meta_insert_remove() {
  let mut meta = MetaOnly::new();
  let tag = DummyTag("🇦🇬".to_string());
  meta.insert_tag(&tag);
  assert_eq!(tag, meta.remove_tag().unwrap())
}

#[test]
fn test_partialeq_dummy_meta() {
  let mut meta1 = MetaOnly::new();
  let mut meta2 = MetaOnly::new();
  assert_eq!(&meta1, &meta2);

  let tag = DummyTag("🇦🇬".to_string());
  meta1.insert_tag(&tag);
  assert_ne!(&meta1, &meta2);

  meta2.insert_tag(&tag);
  assert_eq!(&meta1, &meta2);

  meta2.remove_tag::<DummyTag>();
  assert_ne!(&meta1, &meta2);

  meta1.remove_tag::<DummyTag>();
  assert_eq!(&meta1, &meta2);
}
