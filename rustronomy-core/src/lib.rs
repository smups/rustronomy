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

#![doc(
  html_logo_url = "https://raw.githubusercontent.com/smups/rustronomy/main/logos/Rustronomy_ferris.png?raw=true"
)]
//! This crate defines the shared API for all rustronomy-project crates.
//! Its purpose is to provide the necessary traits and types to ensure
//! interoperability within the rustronomy project.
//!
//! See the module docs down below for more details regarding specific API's.

//Module Strucutre (module description is given in the module definition files)
pub mod universal_containers;

#[cfg(test)]
mod test;
