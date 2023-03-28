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

//! This module provides high-level API's for commonly used storage objects
//! in astronomy, such as Images and Tables. Crates in the rustronomy ecosystem
//! responsible for encoding/decoding specific file formats (such as [rustronomy-fits](https://github.com/smups/rustronomy-fits))
//! use the containers defined in this trait to facilitate easy data access and
//! conversion between storage formats.
//!
//! ## Metadata
//! All storage objects defined in this module may contain metadata represented
//! as `MetaDataTag`s, which are simple key-value pairs. Some metadata tags are
//! so commonly used in astronomy (such as a tag for the telescope or author)
//! that they have already been implemented here. In addition, users can add
//! their own metadata tags. For more info, see the metadata module.

/* (Module structure)
  note: module-level documentation is supplied in the modules themselves.
*/
mod meta_only;
mod array;
mod table;

// Handy re-exports (v0.2.1 and higher)
pub use meta_only::MetaOnly;
pub use array::DataArray;
pub use table::{Col, Table};

// utility for formatting byte sizes
pub(crate) fn fmt_byte_size(byte_size: usize) -> String {
  if byte_size <= 1000 {
    format!("{}B", byte_size)
  } else if byte_size <= 1_000_000 {
    format!("{}kB", byte_size / 1000)
  } else if byte_size <= 1_000_000_000 {
    format!("{}MB", byte_size / 1_000_000)
  } else if byte_size <= 1_000_000_000_000 {
    format!("{}GB", byte_size / 1_000_000_000)
  } else {
    format!("{}TB", byte_size / 1_000_000_000_000)
  }
}
