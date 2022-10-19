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

use crate::universal_containers::{metadata::PubContainer, Image, MetaDataContainer, Table};

#[test]
fn metadata_test() {
  //Create a table and remove some tags
  let mut tbl = Table::new();
  let my_tag_name = "jan misli";
  let my_tag_value: usize = 1234;
  tbl.insert_generic_tag(my_tag_name, my_tag_value).unwrap();
  assert!(tbl.has_generic_tag(my_tag_name));
  assert!(tbl.remove_generic_tag::<usize>(my_tag_name).unwrap() == my_tag_value);
  assert!(!tbl.has_generic_tag(my_tag_name));

  //try more complex date-time tag
  let now = chrono::Utc::now();
  tbl.insert_date(now).unwrap();
  assert_eq!(tbl.remove_date().unwrap(), now);

  //let's make sure we get an error when we try to access a restricted tag
  assert!(tbl.insert_generic_tag("date", -12).is_err());
}

#[test]
fn image_test() {
  use ndarray as nd;
  type M = nd::Array2<f32>;

  //Create an ndarray with ones on the diagonal
  let data = M::eye(100);
  let copy = data.clone();
  let mut image = Image::new(data);
  print!("{image}");

  //Add some metadata tags
  image.insert_author("me".to_string()).unwrap();
  image.insert_exposure_time(1200).unwrap();
  print!("{image}");

  //deconstruct image
  assert_eq!(copy, image.data());
}
