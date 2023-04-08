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

#[derive(Debug, Clone, PartialEq, Default)]
struct ReferencePublication {
  pub title: String,
  pub authors: String,
  pub affiliation: Option<String>,

  pub journal: Option<String>,
  pub number: Option<u16>,
  pub volume: Option<u16>,
  pub pages: Option<(u16, u16)>,
  pub doi: Option<String>,
  pub url: Option<String>,
  pub date: Option<chrono::NaiveDate>,

  pub email: Option<String>,
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
