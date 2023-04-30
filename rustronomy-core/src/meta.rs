/*
  Copyright© 2023 Raúl Wolters(1)

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

//! This module defines all the traits and types used by `rustronomy` to enable
//! cross-format metadata. Crates in the rustronomy can translate these types
//! into format-specific representations. This way, metadata can be transferred
//! from one storage format to another.
//! 
//! ## A quick overview of the `rustronomy` metadata system
//! The rustronomy metadata system is built from two fundamental traits:
//! - `MetaTag` is implemented by types that specify metadata
//! - `MetaContainer` is implemented by types that contain metadata.
//! 
//! In addition to the "typed" metadata constructed from rust types implementing the
//! `MetaTag` trait, rustronomy also supports "untyped" (or "stringly-typed")
//! metadata consisting of simple `String` (key, value) pairs. `MetaContainer`
//! has methods for interacting with both.
//! 
//! ## Strongly-typed metadata vs stringly-typed metadata
//! The most important distinction between the typed metadata and the simple
//! (key, value) string pairs is how rustronomy crates implementing specific
//! storage formats are expected to handle them:
//! - *Strongly typed* metadata tags are to be encoded into a format-specific
//! representation that conveys their meaning in a machine-readable manner, if
//! this is supported by the specific format.
//! - *Stringly typed* metadata key-value pairs are to be interpreted as simple
//! key-value pairs.
//! As an example, the metadata struct `Author` is encoded by `rustronomy-fits`
//! using the FITS `AUTHOR` keyword, whereas a string metadata entry with the
//! key `"Author"` wouldn't be parsed in any special way.
//! 
//! Furthermore, if a key-value string pair were to conflict with a strongly typed
//! metadata entry, storage format implementations may ignore the conflicting
//! string entry: strongly-typed entries take precedence over stringly-typed ones.
//! 
//! ## built-in metadata tags
//! In addition to these two traits, a bunch of pre-made types that implement the
//! `MetaTag` trait can be found in the `tags` module, all of which are
//! transferable between data storage formats.
//! 
//! The strongly-typed metadata system is *in principle* user-extendable. You can
//! implement `MetaTag` for you own custom metadata tags, but there is no
//! guarantee that these tags will be respected or parsed properly by
//! implementations (since they are most likely not aware of your custom data type). 

//Types that already implement MetaTag
mod auto;
mod custom;

/// This module contains all pre-defined strongly typed tags supported by rustronomy.
pub mod tags {
  pub use super::auto::*;
  pub use super::custom::*;
}

// Re-export the impl_meta_container macro defined in this module to this module's
// path!
pub use crate::impl_meta_container;

use std::{
  any::*,
  fmt::{Debug, Display},
};

/*
  A lot of the following code took inspiration/was copied from the anymap crate.

  Creds to them for coming up with most of the workarounds presented here.
*/

/// Core trait that must be implemented by all structs representing metadata.
///
/// This trait requires implementers to also implement `MetaTagClone`, which is
/// required to enable cloning of `Box<dyn MetaTag>` objects. The `Display` impl
/// of the metadata type is used by rustronomy when printing the metadata contents
/// of a container.
///
/// # Methods
/// Although the `MetaTag` trait itself doesn't contain any methods, there *are*
/// a couple methods implemented for `dyn MetaTag`. Most of these are copies from
/// methods implemented in `std` for `dyn Any`.
pub trait MetaTag: Any + Debug + Display + MetaTagClone + Send + Sync {}

/// This trait is a hack to enable cloning `Box<dyn MetaTag>` objects.
///
/// Trait objs.are DST's and therefore cannot impl `Sized`, which is a bound for
/// the `Clone` trait. This is annoying, because `Box<dyn Trait>` *is* sized and
/// can definitely be cloned if the type implementing `Trait` implements `Clone`.
/// But, since we cannot use `Clone` as a bound on `MetaTag` directly, we cannot
/// simply `impl Clone for Box<dyn MetaTag>`. Instead, we have to use this silly
/// sub trait to express the `Clone` bound separately.
pub trait MetaTagClone {
  fn clone_hack(&self) -> Box<dyn MetaTag>;
}

impl<T: MetaTag + Clone> MetaTagClone for T {
  #[inline(always)]
  fn clone_hack(&self) -> Box<dyn MetaTag> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn MetaTag> {
  #[inline(always)]
  fn clone(&self) -> Self {
    self.clone_hack()
  }
}

impl dyn MetaTag {
  /// Returns `true` if `self` is of type `T`, `false` otherwise
  pub fn is<T: Any>(&self) -> bool {
    TypeId::of::<T>() == self.type_id()
  }

  /// Casts `&self` to `T` by cloning `&T`. This requires `T` to impl `Clone`.
  /// If `self` does not have type `T`, this method returns `None`.
  pub fn downcast<T: Any + Clone>(&self) -> Option<T> {
    Some(self.downcast_ref::<T>()?.clone())
  }

  /// Casts `&self` to `&T`. If `self` does not have type `T`, this method returns `None`.
  pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
    //Safety: these ptr casts are valid because self *is* of type T
    if self.is::<T>() {
      Some(unsafe { &*(self as *const dyn MetaTag as *const T) })
    } else {
      None
    }
  }

  /// Casts `&mut self` to `&mut T`. If `self` does not have type `T`, this method returns `None`.
  pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
    if self.is::<T>() {
      //Safety: these ptr casts are valid because self *is* of type T
      Some(unsafe { &mut *(self as *mut dyn MetaTag as *mut T) })
    } else {
      None
    }
  }
}

/// Core trait that is implemented by all containers of rustronomy metadata types.
///
/// `MetaDataContainer`s consist of two different types of metadata:
///   - strongly typed metadata: these are rust data structures that implement
///     the `MetaTag` trait. These metadata may be accessed using the `*operation*_tag`
///     methods.
///   - stringly typed metadata: these are just string key-value pairs. These
///     metadata may be accessed using the `*operation*_string_tag` methods.
///
/// # For data storage format implementers
/// Strongly typed metadata are the key to how rustronomy can transfer metadata
/// between incompatible data storage formats. Implementers of data storage
/// formats should take special care in storing this kind of metadata.
pub trait MetaContainer: Clone + Debug {
  /// Inserts strongly typed tag with type `T` into the container. If the
  /// container already had this type of metadata, its previous value will be
  /// returned.
  fn insert_tag<T: MetaTag + Clone>(&mut self, tag: &T) -> Option<T>;
  /// Returns `true` if container contains a tag of type `T`, `false` otherwise
  fn contains_tag<T: MetaTag + Clone>(&self) -> bool;
  /// Returns `true` if container contains a tag with a type that matches the
  /// provided `TypeId`, `false` otherwise
  fn contains_type_id(&self, type_id: &TypeId) -> bool;
  /// Returns a reference to the strongly typed tag of type `T`, if it exists.
  fn get_tag<T: MetaTag + Clone>(&self) -> Option<&T>;
  /// Returns a mutable reference to the strongly typed tag of type `T`, if it exists.
  fn get_tag_mut<T: MetaTag + Clone>(&mut self) -> Option<&mut T>;
  /// Removes the strongly typed tag with type `T` from the container if it
  /// is present. Returns the value of the removed entry.
  fn remove_tag<T: MetaTag + Clone>(&mut self) -> Option<T>;
  /// Returns `true` if this container has typed metadata, `false` otherwise
  fn has_typed_metadata(&self) -> bool;

  /// Insert string tag with key `key` into the container. If the container already
  /// contained an entry with this key, its previous value will be returned.
  fn insert_string_tag(&mut self, key: &str, value: &str) -> Option<String>;
  /// Returns `true` if container contains key `key`, `false` otherwise
  fn contains_string_tag(&self, key: &str) -> bool;
  /// Returns a `&str` reference to the string tag with key `key`, if it exists.
  fn get_string_tag(&self, key: &str) -> Option<&str>;
  /// Returns a `&mut String` mutable reference to the string tag with key `key`,
  /// if it exists.
  fn get_string_tag_mut(&mut self, key: &str) -> Option<&mut String>;
  /// Removes tag with key `key` from the container, if it is present. Returns
  /// the value of the removed entry.
  fn remove_string_tag(&mut self, key: &str) -> Option<String>;
  /// Returns `true` if this container has string metadata, `false` otherwise
  fn has_string_metadata(&self) -> bool;
}

#[macro_export]
macro_rules! impl_meta_container {
  () => {
    fn insert_tag<T: MetaTag + Clone>(&mut self, tag: &T) -> Option<T> {
      self.meta.insert_tag(tag)
    }

    fn contains_tag<T: MetaTag + Clone>(&self) -> bool {
      self.meta.contains_tag::<T>()
    }

    fn contains_type_id(&self, type_id: &std::any::TypeId) -> bool {
      self.meta.contains_type_id(type_id)
    }

    fn get_tag<T: MetaTag + Clone>(&self) -> Option<&T> {
      self.meta.get_tag()
    }

    fn get_tag_mut<T: MetaTag + Clone>(&mut self) -> Option<&mut T> {
      self.meta.get_tag_mut()
    }

    fn remove_tag<T: MetaTag + Clone>(&mut self) -> Option<T> {
      self.meta.remove_tag()
    }

    fn has_typed_metadata(&self) -> bool {
      self.meta.has_typed_metadata()
    }

    fn insert_string_tag(&mut self, key: &str, value: &str) -> Option<String> {
      self.meta.insert_string_tag(key, value)
    }

    fn contains_string_tag(&self, key: &str) -> bool {
      self.meta.contains_string_tag(key)
    }

    fn get_string_tag(&self, key: &str) -> Option<&str> {
      self.meta.get_string_tag(key)
    }

    fn get_string_tag_mut(&mut self, key: &str) -> Option<&mut String> {
      self.meta.get_string_tag_mut(key)
    }

    fn remove_string_tag(&mut self, key: &str) -> Option<String> {
      self.meta.remove_string_tag(key)
    }

    fn has_string_metadata(&self) -> bool {
      self.meta.has_string_metadata()
    }
  };
}
