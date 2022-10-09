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
//! The currently restricted tags are:
//! | restricted tag string | restricted tag struct | description |
//! | --: | :--: | :-- |
//! | `"author"` | `Author(String)` | author |
//! | `"date"` | `Date(chrono::DateTime<chrono::Utc>)` | data creation date |
//! | `"last_modified"` | `LastModified(chrono::DateTime<chrono::Utc>)` | date file was last modified |
//! | `"organisation"` | `Organisation(String)` | data was created by |
//! | `"reference"` | `Reference(String)` | reference publication for data |
//! | `"reference_doi"` | `ReferenceDOI(String)` | doi of reference publication |
//! | `"telescope"` | `Telescope(String)` | telescope |
//! | `"instrument"` | `Instrument(String)` | instrument |
//! | `"object"` | `Object(String)` | observed object |
//! | `"exposure_time"` | `ExposureTime(u64)` | exposure time in ms |

use super::metadata::{MetaDataTag, TagError, PubContainer, private_container::PrivContainer};
use std::str::FromStr;

pub const RESTRICTED_TAGS: [&str;10] = [
  "author",
  "date",
  "last_modified",
  "organisation",
  "reference",
  "reference_doi",
  "telescope",
  "instrument",
  "object",
  "exposure_time"
];

#[derive(Debug)]
pub struct Author(String);

impl From<String> for Author {
  fn from(author: String) -> Self {Author(author)}
}

impl MetaDataTag for Author {
  const KEY: &'static str = "author";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(author) => Ok(author.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct Date(chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for Date {
  fn from(date: chrono::DateTime<chrono::Utc>) -> Self {Date(date)}
}

impl MetaDataTag for Date {
  const KEY: &'static str = "date";
  type ValueType = chrono::DateTime<chrono::Utc>;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<chrono::DateTime<chrono::Utc>>() {
      Ok(date) => Ok(date.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct LastModified(chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for LastModified {
  fn from(last_modified: chrono::DateTime<chrono::Utc>) -> Self {LastModified(last_modified)}
}

impl MetaDataTag for LastModified {
  const KEY: &'static str = "last_modified";
  type ValueType = chrono::DateTime<chrono::Utc>;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<chrono::DateTime<chrono::Utc>>() {
      Ok(last_modified) => Ok(last_modified.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct Organisation(String);

impl From<String> for Organisation {
  fn from(organisation: String) -> Self {Organisation(organisation)}
}

impl MetaDataTag for Organisation {
  const KEY: &'static str = "organisation";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(organisation) => Ok(organisation.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct Reference(String);

impl From<String> for Reference {
  fn from(reference: String) -> Self {Reference(reference)}
}

impl MetaDataTag for Reference {
  const KEY: &'static str = "reference";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(reference) => Ok(reference.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct ReferenceDOI(String);

impl From<String> for ReferenceDOI {
  fn from(reference_doi: String) -> Self {ReferenceDOI(reference_doi)}
}

impl MetaDataTag for ReferenceDOI {
  const KEY: &'static str = "reference_doi";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(reference_doi) => Ok(reference_doi.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct Telescope(String);

impl From<String> for Telescope {
  fn from(telescope: String) -> Self {Telescope(telescope)}
}

impl MetaDataTag for Telescope {
  const KEY: &'static str = "telescope";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(telescope) => Ok(telescope.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct Instrument(String);

impl From<String> for Instrument {
  fn from(instrument: String) -> Self {Instrument(instrument)}
}

impl MetaDataTag for Instrument {
  const KEY: &'static str = "instrument";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(instrument) => Ok(instrument.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct Object(String);

impl From<String> for Object {
  fn from(object: String) -> Self {Object(object)}
}

impl MetaDataTag for Object {
  const KEY: &'static str = "object";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(object) => Ok(object.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

#[derive(Debug)]
pub struct ExposureTime(u64);

impl From<u64> for ExposureTime {
  fn from(exposure_time: u64) -> Self {ExposureTime(exposure_time)}
}

impl MetaDataTag for ExposureTime {
  const KEY: &'static str = "exposure_time";
  type ValueType = u64;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<u64>() {
      Ok(exposure_time) => Ok(exposure_time.into()),
      Err(err) => Err(err)
    }
  }
  
  fn to_string(&self) -> String { format!("{}", self.0)}
}

pub trait MetaDataContainer: PrivContainer + PubContainer {
fn remove_author(&mut self, key: &str) -> Result<Author, TagError> { self.remove_tag(key) }
fn insert_author(&mut self, author: String) -> Result<Option<Author>, TagError> { self.insert_tag(author.into()) }

fn remove_date(&mut self, key: &str) -> Result<Date, TagError> { self.remove_tag(key) }
fn insert_date(&mut self, date: chrono::DateTime<chrono::Utc>) -> Result<Option<Date>, TagError> { self.insert_tag(date.into()) }

fn remove_last_modified(&mut self, key: &str) -> Result<LastModified, TagError> { self.remove_tag(key) }
fn insert_last_modified(&mut self, last_modified: chrono::DateTime<chrono::Utc>) -> Result<Option<LastModified>, TagError> { self.insert_tag(last_modified.into()) }

fn remove_organisation(&mut self, key: &str) -> Result<Organisation, TagError> { self.remove_tag(key) }
fn insert_organisation(&mut self, organisation: String) -> Result<Option<Organisation>, TagError> { self.insert_tag(organisation.into()) }

fn remove_reference(&mut self, key: &str) -> Result<Reference, TagError> { self.remove_tag(key) }
fn insert_reference(&mut self, reference: String) -> Result<Option<Reference>, TagError> { self.insert_tag(reference.into()) }

fn remove_reference_doi(&mut self, key: &str) -> Result<ReferenceDOI, TagError> { self.remove_tag(key) }
fn insert_reference_doi(&mut self, reference_doi: String) -> Result<Option<ReferenceDOI>, TagError> { self.insert_tag(reference_doi.into()) }

fn remove_telescope(&mut self, key: &str) -> Result<Telescope, TagError> { self.remove_tag(key) }
fn insert_telescope(&mut self, telescope: String) -> Result<Option<Telescope>, TagError> { self.insert_tag(telescope.into()) }

fn remove_instrument(&mut self, key: &str) -> Result<Instrument, TagError> { self.remove_tag(key) }
fn insert_instrument(&mut self, instrument: String) -> Result<Option<Instrument>, TagError> { self.insert_tag(instrument.into()) }

fn remove_object(&mut self, key: &str) -> Result<Object, TagError> { self.remove_tag(key) }
fn insert_object(&mut self, object: String) -> Result<Option<Object>, TagError> { self.insert_tag(object.into()) }

fn remove_exposure_time(&mut self, key: &str) -> Result<ExposureTime, TagError> { self.remove_tag(key) }
fn insert_exposure_time(&mut self, exposure_time: u64) -> Result<Option<ExposureTime>, TagError> { self.insert_tag(exposure_time.into()) }

}