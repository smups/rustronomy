use std::{collections::HashMap, fmt::Debug};

trait MetaTag: anymap::any::CloneAny + Sized + Debug {
  const KEY: &'static str;
  type Inner;

  fn from_inner(inner: Self::Inner) -> Self;
  fn to_inner(self) -> Self::Inner;
  fn as_ref_inner(&self) -> &Self::Inner;
  fn as_mut_inner(&mut self) -> &mut Self::Inner;
}

trait MetaDataContainer {
  fn insert_tag<T: MetaTag>(&mut self, tag: T) -> Option<T::Inner>;
  fn get_tag<T: MetaTag>(&self) -> Option<&T::Inner>;
  fn get_tag_mut<T: MetaTag>(&mut self) -> Option<&mut T::Inner>;
  fn remove_tag<T: MetaTag>(&mut self) -> Option<T::Inner>;

  fn insert_string_tag(&mut self, key: &str, value: &str) -> Option<String>;
  fn get_string_tag(&self, key: &str) -> Option<&str>;
  fn get_string_tag_mut(&mut self, key: &str) -> Option<&mut String>;
  fn remove_string_tag(&mut self, key: &str) -> Option<String>;
}

#[derive(Debug, Clone)]
struct MetaOnly {
  string_tags: HashMap<String, String>,
  typed_tags: anymap::Map<dyn anymap::any::CloneAny>
}

impl MetaOnly {
  pub fn new() -> Self {
    MetaOnly { string_tags: HashMap::new(), typed_tags: anymap::Map::new() }
  }
}

impl MetaDataContainer for MetaOnly {
  fn insert_tag<T: MetaTag>(&mut self, tag: T) -> Option<T::Inner> {
    self.typed_tags.insert(tag).and_then(|x| Some(x.to_inner()))
  }

  fn get_tag<T: MetaTag>(&self) -> Option<&T::Inner> {
    self.typed_tags.get::<T>().and_then(|x| Some(x.as_ref_inner()))
  }

  fn get_tag_mut<T: MetaTag>(&mut self) -> Option<&mut T::Inner> {
    self.typed_tags.get_mut::<T>().and_then(|x| Some(x.as_mut_inner()))
  }

  fn remove_tag<T: MetaTag>(&mut self) -> Option<T::Inner> {
    self.typed_tags.remove::<T>().and_then(|x| Some(x.to_inner()))
  }

  fn insert_string_tag(&mut self, key: &str, value: &str) -> Option<String> {
    self.string_tags.insert(key.to_string(), value.to_string())
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
}

#[derive(Debug, Clone)]
struct AuthorTag(pub String);

impl MetaTag for AuthorTag {
  const KEY: &'static str = "Author";
  type Inner = String;

  #[inline(always)]
  fn from_inner(inner: Self::Inner) -> Self { AuthorTag(inner) }

  #[inline(always)]
  fn to_inner(self) -> Self::Inner { self.0 }

  #[inline(always)]
  fn as_ref_inner(&self) -> &Self::Inner { &self.0 }

  #[inline(always)]
  fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[test]
fn test() {
  let mut meta = MetaOnly::new();
  meta.insert_tag(AuthorTag("me".to_string()));
  dbg!(meta.get_tag::<AuthorTag>());
  dbg!(meta);
}