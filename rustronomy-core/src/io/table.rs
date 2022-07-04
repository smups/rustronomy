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

//! This module specifies a table container. Tables consist of named columns
//! that hold a bunch of values of the same type. Tables should not be used to
//! store large amounts of data.
//!
//! Three column types are supported:
//! - `Integer` always a `Vec<i64>`
//! - `Float` always a `Vec<f64>`
//! - `Text` always a `Vec<String>`
