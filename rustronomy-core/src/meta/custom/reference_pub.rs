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

use crate::meta::MetaTag;
use chrono::Datelike;

use getset::{Getters, Setters, MutGetters};

#[derive(Debug, Clone, PartialEq, Default, Getters, Setters, MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
/// Struct representing a reference to an (academic) publication describing the
/// data inside the container. To be used as a rustronomy `MetaTag`.
/// 
/// # Optional fields
/// This struct contains a lot of optional data which may be accessed using the
/// `self.xyz()`, `self.xyz_mut()` and `self.set_xyz()` methods. Currently,
/// the following optional metadata is supported (more may be added in the future):
/// - `affiliation` affiliation of the authors
/// - `journal`, `number`, `volume`, `pages` to direct users to the publication 
/// - `doi` [Digital Object Identifier](https://www.doi.org/)
/// - `url` url to publication (e.g. arXiv or publisher site)
/// - `date` date of publishing
/// - `email` contact info for author handling correspondence
/// 
/// *Note: these struct fields have to be private because changing the number of
/// public struct fields is currently a breaking change in Rust. This is why getters
/// and setters are used here.*
pub struct ReferencePublication {
  title: String,
  authors: String,
  affiliation: Option<String>,

  journal: Option<String>,
  number: Option<u16>,
  volume: Option<u16>,
  pages: Option<(u16, u16)>,
  doi: Option<String>,
  url: Option<String>,
  date: Option<chrono::NaiveDate>,

  email: Option<String>,
}

impl ReferencePublication {
  /// Constructs a `ReferencePublication` with all optional fields set to `None`
  pub fn new(title: &str, authors: &str) -> Self {
    let mut out = ReferencePublication::default();
    out.title = title.to_string();
    out.authors = authors.to_string();
    out
  }
}

impl MetaTag for ReferencePublication {}
impl std::fmt::Display for ReferencePublication {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "[Reference Publication]: ")?;
    writeln!(f, ">\"{}\"", self.title)?;
    writeln!(f, ">Authors: {}", self.authors)?;
    if let Some(af) = &self.affiliation {
      writeln!(f, ">Affiliation: {af}")?
    };
    if let Some(journal) = &self.journal {
      write!(f, ">In \"{journal}\", ")?
    };
    if let Some(volume) = self.volume {
      write!(f, "Vol.{volume}, ")?
    };
    if let Some(number) = self.number {
      write!(f, "№{number} ")?
    };
    if let Some((p1, p2)) = self.pages {
      write!(f, "pages {p1}-{p2} ")?
    };
    if let Some(date) = self.date {
      write!(f, "({})", date.year())?
    };
    writeln!(f, "")?;
    if let Some(url) = &self.url {
      writeln!(f, ">URL: {url}")?
    };
    if let Some(doi) = &self.doi {
      writeln!(f, ">DOI: {doi}")?
    };
    if let Some(email) = &self.email {
      writeln!(f, ">Contact: {email}")?
    };
    Ok(())
  }
}
