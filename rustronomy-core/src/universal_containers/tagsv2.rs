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

#[derive(Debug, Clone)]
struct Author(pub String);

impl From<String> for Author {
  fn from(x: String) -> Self { Author(x) }
}

impl From<Author> for String {
  fn from(x: Author) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[author]: {}", self.0)
  }
}

impl MetaTag for Author {
  type Inner = String;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { Author(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct Date(pub chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for Date {
  fn from(x: chrono::DateTime<chrono::Utc>) -> Self { Date(x) }
}

impl From<Date> for chrono::DateTime<chrono::Utc> {
  fn from(x: Date) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[date]: {}", self.0)
  }
}

impl MetaTag for Date {
  type Inner = chrono::DateTime<chrono::Utc>;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { Date(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct LastModified(pub chrono::DateTime<chrono::Utc>);

impl From<chrono::DateTime<chrono::Utc>> for LastModified {
  fn from(x: chrono::DateTime<chrono::Utc>) -> Self { LastModified(x) }
}

impl From<LastModified> for chrono::DateTime<chrono::Utc> {
  fn from(x: LastModified) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[last_modified]: {}", self.0)
  }
}

impl MetaTag for LastModified {
  type Inner = chrono::DateTime<chrono::Utc>;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { LastModified(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct Organisation(pub String);

impl From<String> for Organisation {
  fn from(x: String) -> Self { Organisation(x) }
}

impl From<Organisation> for String {
  fn from(x: Organisation) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[organisation]: {}", self.0)
  }
}

impl MetaTag for Organisation {
  type Inner = String;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { Organisation(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct Reference(pub String);

impl From<String> for Reference {
  fn from(x: String) -> Self { Reference(x) }
}

impl From<Reference> for String {
  fn from(x: Reference) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[reference]: {}", self.0)
  }
}

impl MetaTag for Reference {
  type Inner = String;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { Reference(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct ReferenceDOI(pub String);

impl From<String> for ReferenceDOI {
  fn from(x: String) -> Self { ReferenceDOI(x) }
}

impl From<ReferenceDOI> for String {
  fn from(x: ReferenceDOI) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[reference_doi]: {}", self.0)
  }
}

impl MetaTag for ReferenceDOI {
  type Inner = String;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { ReferenceDOI(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct Telescope(pub String);

impl From<String> for Telescope {
  fn from(x: String) -> Self { Telescope(x) }
}

impl From<Telescope> for String {
  fn from(x: Telescope) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[telescope]: {}", self.0)
  }
}

impl MetaTag for Telescope {
  type Inner = String;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { Telescope(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct Instrument(pub String);

impl From<String> for Instrument {
  fn from(x: String) -> Self { Instrument(x) }
}

impl From<Instrument> for String {
  fn from(x: Instrument) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[instrument]: {}", self.0)
  }
}

impl MetaTag for Instrument {
  type Inner = String;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { Instrument(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct Object(pub String);

impl From<String> for Object {
  fn from(x: String) -> Self { Object(x) }
}

impl From<Object> for String {
  fn from(x: Object) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[object]: {}", self.0)
  }
}

impl MetaTag for Object {
  type Inner = String;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { Object(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

#[derive(Debug, Clone)]
struct ExposureTime(pub u64);

impl From<u64> for ExposureTime {
  fn from(x: u64) -> Self { ExposureTime(x) }
}

impl From<ExposureTime> for u64 {
  fn from(x: ExposureTime) -> Self { x.0 }
}

impl std::fmt::Display for AuthorTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[exposure_time]: {}", self.0)
  }
}

impl MetaTag for ExposureTime {
  type Inner = u64;
  
  #[inline] fn from_inner(inner: Self::Inner) -> Self { ExposureTime(inner) }
  #[inline] fn to_inner(self) -> Self::Inner { self.0 }
  #[inline] fn as_ref_inner(&self) -> &Self::Inner { &self.0 }
  #[inline] fn as_mut_inner(&mut self) -> &mut Self::Inner { &mut self.0 }
}

use super::metadata::{MetaDataTag, TagError, PubContainer, private_container::PrivContainer};
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
    Err(err) => Err(err)
  }
}
fn insert_author(&mut self, author: String) -> Result<Option<String>, TagError> {
  match self.insert_tag::<Author>(author.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_author(&self) -> bool {
  self.has_tag::<Author>()
}

fn remove_date(&mut self) -> Result<chrono::DateTime<chrono::Utc>, TagError> {
  match self.remove_tag::<Date>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_date(&mut self, date: chrono::DateTime<chrono::Utc>) -> Result<Option<chrono::DateTime<chrono::Utc>>, TagError> {
  match self.insert_tag::<Date>(date.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_date(&self) -> bool {
  self.has_tag::<Date>()
}

fn remove_last_modified(&mut self) -> Result<chrono::DateTime<chrono::Utc>, TagError> {
  match self.remove_tag::<LastModified>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_last_modified(&mut self, last_modified: chrono::DateTime<chrono::Utc>) -> Result<Option<chrono::DateTime<chrono::Utc>>, TagError> {
  match self.insert_tag::<LastModified>(last_modified.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_last_modified(&self) -> bool {
  self.has_tag::<LastModified>()
}

fn remove_organisation(&mut self) -> Result<String, TagError> {
  match self.remove_tag::<Organisation>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_organisation(&mut self, organisation: String) -> Result<Option<String>, TagError> {
  match self.insert_tag::<Organisation>(organisation.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_organisation(&self) -> bool {
  self.has_tag::<Organisation>()
}

fn remove_reference(&mut self) -> Result<String, TagError> {
  match self.remove_tag::<Reference>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_reference(&mut self, reference: String) -> Result<Option<String>, TagError> {
  match self.insert_tag::<Reference>(reference.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_reference(&self) -> bool {
  self.has_tag::<Reference>()
}

fn remove_reference_doi(&mut self) -> Result<String, TagError> {
  match self.remove_tag::<ReferenceDOI>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_reference_doi(&mut self, reference_doi: String) -> Result<Option<String>, TagError> {
  match self.insert_tag::<ReferenceDOI>(reference_doi.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_reference_doi(&self) -> bool {
  self.has_tag::<ReferenceDOI>()
}

fn remove_telescope(&mut self) -> Result<String, TagError> {
  match self.remove_tag::<Telescope>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_telescope(&mut self, telescope: String) -> Result<Option<String>, TagError> {
  match self.insert_tag::<Telescope>(telescope.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_telescope(&self) -> bool {
  self.has_tag::<Telescope>()
}

fn remove_instrument(&mut self) -> Result<String, TagError> {
  match self.remove_tag::<Instrument>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_instrument(&mut self, instrument: String) -> Result<Option<String>, TagError> {
  match self.insert_tag::<Instrument>(instrument.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_instrument(&self) -> bool {
  self.has_tag::<Instrument>()
}

fn remove_object(&mut self) -> Result<String, TagError> {
  match self.remove_tag::<Object>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_object(&mut self, object: String) -> Result<Option<String>, TagError> {
  match self.insert_tag::<Object>(object.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_object(&self) -> bool {
  self.has_tag::<Object>()
}

fn remove_exposure_time(&mut self) -> Result<u64, TagError> {
  match self.remove_tag::<ExposureTime>() {
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }
}
fn insert_exposure_time(&mut self, exposure_time: u64) -> Result<Option<u64>, TagError> {
  match self.insert_tag::<ExposureTime>(exposure_time.into()) {
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }
}
fn has_exposure_time(&self) -> bool {
  self.has_tag::<ExposureTime>()
}

}