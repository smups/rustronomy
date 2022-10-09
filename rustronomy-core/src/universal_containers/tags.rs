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
pub const RESTRICTED_TAGS: [&str; 10] = [
  "author",
  "date",
  "last_modified",
  "organisation",
  "reference",
  "reference_doi",
  "telescope",
  "instrument",
  "object",
  "exposure_time",
];

#[derive(Debug)]
struct Author(String);

impl From<String> for Author {
  fn from(author: String) -> Self {
    Author(author)
  }
}

impl From<Author> for String {
  fn from(author: Author) -> Self {
    author.0
  }
}

impl MetaDataTag for Author {
  const KEY: &'static str = "author";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(author) => Ok(author.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct Date(chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for Date {
  fn from(date: chrono::DateTime<chrono::Utc>) -> Self {
    Date(date)
  }
}

impl From<Date> for chrono::DateTime<chrono::Utc> {
  fn from(date: Date) -> Self {
    date.0
  }
}

impl MetaDataTag for Date {
  const KEY: &'static str = "date";
  type ValueType = chrono::DateTime<chrono::Utc>;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<chrono::DateTime<chrono::Utc>>() {
      Ok(date) => Ok(date.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct LastModified(chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for LastModified {
  fn from(last_modified: chrono::DateTime<chrono::Utc>) -> Self {
    LastModified(last_modified)
  }
}

impl From<LastModified> for chrono::DateTime<chrono::Utc> {
  fn from(last_modified: LastModified) -> Self {
    last_modified.0
  }
}

impl MetaDataTag for LastModified {
  const KEY: &'static str = "last_modified";
  type ValueType = chrono::DateTime<chrono::Utc>;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<chrono::DateTime<chrono::Utc>>() {
      Ok(last_modified) => Ok(last_modified.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct Organisation(String);

impl From<String> for Organisation {
  fn from(organisation: String) -> Self {
    Organisation(organisation)
  }
}

impl From<Organisation> for String {
  fn from(organisation: Organisation) -> Self {
    organisation.0
  }
}

impl MetaDataTag for Organisation {
  const KEY: &'static str = "organisation";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(organisation) => Ok(organisation.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct Reference(String);

impl From<String> for Reference {
  fn from(reference: String) -> Self {
    Reference(reference)
  }
}

impl From<Reference> for String {
  fn from(reference: Reference) -> Self {
    reference.0
  }
}

impl MetaDataTag for Reference {
  const KEY: &'static str = "reference";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(reference) => Ok(reference.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct ReferenceDOI(String);

impl From<String> for ReferenceDOI {
  fn from(reference_doi: String) -> Self {
    ReferenceDOI(reference_doi)
  }
}

impl From<ReferenceDOI> for String {
  fn from(reference_doi: ReferenceDOI) -> Self {
    reference_doi.0
  }
}

impl MetaDataTag for ReferenceDOI {
  const KEY: &'static str = "reference_doi";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(reference_doi) => Ok(reference_doi.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct Telescope(String);

impl From<String> for Telescope {
  fn from(telescope: String) -> Self {
    Telescope(telescope)
  }
}

impl From<Telescope> for String {
  fn from(telescope: Telescope) -> Self {
    telescope.0
  }
}

impl MetaDataTag for Telescope {
  const KEY: &'static str = "telescope";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(telescope) => Ok(telescope.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct Instrument(String);

impl From<String> for Instrument {
  fn from(instrument: String) -> Self {
    Instrument(instrument)
  }
}

impl From<Instrument> for String {
  fn from(instrument: Instrument) -> Self {
    instrument.0
  }
}

impl MetaDataTag for Instrument {
  const KEY: &'static str = "instrument";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(instrument) => Ok(instrument.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct Object(String);

impl From<String> for Object {
  fn from(object: String) -> Self {
    Object(object)
  }
}

impl From<Object> for String {
  fn from(object: Object) -> Self {
    object.0
  }
}

impl MetaDataTag for Object {
  const KEY: &'static str = "object";
  type ValueType = String;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<String>() {
      Ok(object) => Ok(object.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

#[derive(Debug)]
struct ExposureTime(u64);

impl From<u64> for ExposureTime {
  fn from(exposure_time: u64) -> Self {
    ExposureTime(exposure_time)
  }
}

impl From<ExposureTime> for u64 {
  fn from(exposure_time: ExposureTime) -> Self {
    exposure_time.0
  }
}

impl MetaDataTag for ExposureTime {
  const KEY: &'static str = "exposure_time";
  type ValueType = u64;

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {
    match text.parse::<u64>() {
      Ok(exposure_time) => Ok(exposure_time.into()),
      Err(err) => Err(err),
    }
  }

  fn to_string(&self) -> String {
    format!("{}", self.0)
  }
}

use super::metadata::{private_container::PrivContainer, MetaDataTag, PubContainer, TagError};
use std::str::FromStr;

/// The `MetaDataContainer` trait specifies all the methods that a metadata
/// container must implement. This includes special methods for accessing all
/// restricted metadata keys. The restricted metadata tags are listed in the trait
/// level documentation down below.  
///
/// The currently restricted tags are:
///
/// | restricted tag string | restricted tag type | description |
/// | --: | :--: | :-- |
/// | `"author"` | `Author(String)` | author |
/// | `"date"` | `Date(chrono::DateTime<chrono::Utc>)` | data creation date |
/// | `"last_modified"` | `LastModified(chrono::DateTime<chrono::Utc>)` | date file was last modified |
/// | `"organisation"` | `Organisation(String)` | data was created by |
/// | `"reference"` | `Reference(String)` | reference publication for data |
/// | `"reference_doi"` | `ReferenceDOI(String)` | doi of reference publication |
/// | `"telescope"` | `Telescope(String)` | telescope |
/// | `"instrument"` | `Instrument(String)` | instrument |
/// | `"object"` | `Object(String)` | observed object |
/// | `"exposure_time"` | `ExposureTime(u64)` | exposure time in ms |
pub trait MetaDataContainer: PrivContainer + PubContainer {
  fn remove_author(&mut self) -> Result<String, TagError> {
    match self.remove_tag::<Author>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_author(&mut self, author: String) -> Result<Option<String>, TagError> {
    match self.insert_tag::<Author>(author.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_author(&self) -> bool {
    self.has_tag::<Author>()
  }

  fn remove_date(&mut self) -> Result<chrono::DateTime<chrono::Utc>, TagError> {
    match self.remove_tag::<Date>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_date(
    &mut self,
    date: chrono::DateTime<chrono::Utc>,
  ) -> Result<Option<chrono::DateTime<chrono::Utc>>, TagError> {
    match self.insert_tag::<Date>(date.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_date(&self) -> bool {
    self.has_tag::<Date>()
  }

  fn remove_last_modified(&mut self) -> Result<chrono::DateTime<chrono::Utc>, TagError> {
    match self.remove_tag::<LastModified>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_last_modified(
    &mut self,
    last_modified: chrono::DateTime<chrono::Utc>,
  ) -> Result<Option<chrono::DateTime<chrono::Utc>>, TagError> {
    match self.insert_tag::<LastModified>(last_modified.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_last_modified(&self) -> bool {
    self.has_tag::<LastModified>()
  }

  fn remove_organisation(&mut self) -> Result<String, TagError> {
    match self.remove_tag::<Organisation>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_organisation(&mut self, organisation: String) -> Result<Option<String>, TagError> {
    match self.insert_tag::<Organisation>(organisation.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_organisation(&self) -> bool {
    self.has_tag::<Organisation>()
  }

  fn remove_reference(&mut self) -> Result<String, TagError> {
    match self.remove_tag::<Reference>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_reference(&mut self, reference: String) -> Result<Option<String>, TagError> {
    match self.insert_tag::<Reference>(reference.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_reference(&self) -> bool {
    self.has_tag::<Reference>()
  }

  fn remove_reference_doi(&mut self) -> Result<String, TagError> {
    match self.remove_tag::<ReferenceDOI>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_reference_doi(&mut self, reference_doi: String) -> Result<Option<String>, TagError> {
    match self.insert_tag::<ReferenceDOI>(reference_doi.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_reference_doi(&self) -> bool {
    self.has_tag::<ReferenceDOI>()
  }

  fn remove_telescope(&mut self) -> Result<String, TagError> {
    match self.remove_tag::<Telescope>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_telescope(&mut self, telescope: String) -> Result<Option<String>, TagError> {
    match self.insert_tag::<Telescope>(telescope.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_telescope(&self) -> bool {
    self.has_tag::<Telescope>()
  }

  fn remove_instrument(&mut self) -> Result<String, TagError> {
    match self.remove_tag::<Instrument>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_instrument(&mut self, instrument: String) -> Result<Option<String>, TagError> {
    match self.insert_tag::<Instrument>(instrument.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_instrument(&self) -> bool {
    self.has_tag::<Instrument>()
  }

  fn remove_object(&mut self) -> Result<String, TagError> {
    match self.remove_tag::<Object>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_object(&mut self, object: String) -> Result<Option<String>, TagError> {
    match self.insert_tag::<Object>(object.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_object(&self) -> bool {
    self.has_tag::<Object>()
  }

  fn remove_exposure_time(&mut self) -> Result<u64, TagError> {
    match self.remove_tag::<ExposureTime>() {
      Ok(tag) => Ok(tag.into()),
      Err(err) => Err(err),
    }
  }
  fn insert_exposure_time(&mut self, exposure_time: u64) -> Result<Option<u64>, TagError> {
    match self.insert_tag::<ExposureTime>(exposure_time.into()) {
      Ok(Some(tag)) => Ok(Some(tag.into())),
      Ok(None) => Ok(None),
      Err(err) => Err(err),
    }
  }
  fn has_exposure_time(&self) -> bool {
    self.has_tag::<ExposureTime>()
  }
}
