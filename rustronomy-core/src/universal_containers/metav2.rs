use std::{
  any::*,
  collections::HashMap,
  fmt::{Debug, Display},
};

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
  fn is<T: Any>(&self) -> bool {
    TypeId::of::<T>() == self.type_id()
  }

  /// Casts `&self` to `T` by cloning `&T`. This requires `T` to impl `Clone`.
  /// If `self` does not have type `T`, this method returns `None`.
  fn downcast<T: Any + Clone>(&self) -> Option<T> {
    Some(self.downcast_ref::<T>()?.clone())
  }

  /// Casts `&self` to `&T`. If `self` does not have type `T`, this method returns `None`.
  fn downcast_ref<T: Any>(&self) -> Option<&T> {
    //Safety: these ptr casts are valid because self *is* of type T
    if self.is::<T>() {
      Some(unsafe { &*(self as *const dyn MetaTag as *const T) })
    } else {
      None
    }
  }

  /// Casts `&mut self` to `&mut T`. If `self` does not have type `T`, this method returns `None`.
  fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
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
trait MetaDataContainer: Clone + Debug {
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

#[derive(Clone, Default)]
/// Basic hashmap based metadata container, containing only metadata.
///
/// # Utility traits
/// `MetaOnly` implements `Clone`, `Debug`, `Default`, `Display`, `Send` and `Sync`
struct MetaOnly {
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
