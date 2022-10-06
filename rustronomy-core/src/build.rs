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
/*
  This build-script constructs all required
*/

use std::path::Path;
use std::fs;
use std::env;

use csv;

const RESERVED_TAGS_IN: &str = "resources/tags.csv";
const RESERVED_TAGS_OUT: &str = "src/universal_containers/tags.rs";

fn main() {
  println!("Generating reserved tag structs...");
  let out_dir = env::var_os("OUT_DIR").unwrap();
  let in_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
  let source_path = Path::new(&in_dir).join(RESERVED_TAGS_IN);
  let dest_path = Path::new(&out_dir).join(RESERVED_TAGS_OUT);

  println!("{dest_path:?}");

  //open csv file with all reserved tags
  let mut csv_reader = csv::ReaderBuilder::new()
    .comment(Some(b'#'))
    .from_path(source_path)
    .unwrap();
  
  //create struct for each reserved tags, and add them to the reserved tags list
  for row in csv_reader.records() {
    let row = row.unwrap();
    let (tag, struct_name, descr, tag_ty) = (&row[0], &row[1], &row[2], &row[3]);
    fs::write(
      &dest_path,
      format!("
      #[derive(Debug)]
      pub struct {struct_name}({tag_ty});
      impl super::metadata::MetaDataContainer
      ")
    ).unwrap();
  }

  //re-run requirements
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed={RESERVED_TAGS_IN}");
}