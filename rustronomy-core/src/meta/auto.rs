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

use std::fmt::{Display, Debug, Formatter, Result};
use super::MetaTag;

#[derive(Debug, Clone, PartialEq)]
/// Author responsible for data container
pub struct Author(pub String);
impl MetaTag for Author {}
impl Display for Author {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Author]: \"{}\"", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
/// Date of data collection
pub struct CreationDate(pub chrono::DateTime<chrono::Utc>);
impl MetaTag for CreationDate {}
impl Display for CreationDate {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Creation date]: \"{}\"", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
/// Date container was last modified
pub struct LastModified(pub chrono::DateTime<chrono::Utc>);
impl MetaTag for LastModified {}
impl Display for LastModified {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Last modified date]: \"{}\"", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
/// Organisation responsible for data container
pub struct Organisation(pub String);
impl MetaTag for Organisation {}
impl Display for Organisation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Organisation]: \"{}\"", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
/// Telescope used to collect data (astronomy-specific)
pub struct Telescope(pub String);
impl MetaTag for Telescope {}
impl Display for Telescope {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Telescope]: \"{}\"", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
/// Instrument used to collect data (astronomy-specific)
pub struct Instrument(pub String);
impl MetaTag for Instrument {}
impl Display for Instrument {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Instrument]: \"{}\"", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
/// Observation target (astronomy-specific)
pub struct Object(pub String);
impl MetaTag for Object {}
impl Display for Object {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Target object]: \"{}\"", self.0)
  }
}

#[derive(Debug, Clone, PartialEq)]
/// Exposure time in ms (astronomy-specific)
pub struct ExposureTime(pub u64);
impl MetaTag for ExposureTime {}
impl Display for ExposureTime {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[Exposure time (ms)]: \"{}\"", self.0)
  }
}

