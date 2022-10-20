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
