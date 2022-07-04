/*
    Copyright (C) 2021 Raúl Wolters

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
