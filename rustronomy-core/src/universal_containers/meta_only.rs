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

use std::{
  collections::HashMap,
  fmt::{self, Debug, Display, Formatter},
};

use super::metadata::{private_container::PrivContainer, MetaDataContainer, PubContainer};

#[derive(Debug, Clone)]
pub struct MetaOnly(HashMap<String, String>);

impl MetaOnly {
  pub fn new() -> Self {
    MetaOnly(HashMap::new())
  }
}

impl PrivContainer for MetaOnly {
  fn remove_all_tags(&mut self) -> Vec<(String, String)> {
    self.0.drain().collect()
  }

  fn remove_tag_str(&mut self, key: &str) -> Option<String> {
    self.0.remove(key)
  }

  fn insert_tag_str(&mut self, parsed_tag: &str, key: &str) -> Option<String> {
    self.0.insert(key.to_string(), parsed_tag.to_string())
  }

  fn has_tag_str(&self, key: &str) -> bool {
    self.0.contains_key(key)
  }

  fn clone_tags(&self) -> Vec<(String, String)> {
    self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }
}

impl PubContainer for MetaOnly {
  fn clone_metadata(&self) -> super::meta_only::MetaOnly {
    self.clone()
  }
}
impl MetaDataContainer for MetaOnly {}

impl Display for MetaOnly {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(
      f,
      ">----------------------------------<|METADATA|>---------------------------------"
    )?;
    for (tag, val) in self.0.iter() {
      writeln!(f, ">\"{tag}\": {val}")?;
    }
    writeln!(f, ">===============================================================================")
  }
}
