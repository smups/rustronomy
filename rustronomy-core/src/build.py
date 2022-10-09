# Copyright (C) 2022 Raúl Wolters
#
# This file is part of rustronomy.
#
# rustronomy is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# rustronomy is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with rustronomy.  If not, see <http://www.gnu.org/licenses/>.
import os
import numpy as np

print("hello from the python script")
cwd = os.getcwd()

# input csv file for generating tag structs
tag_input = "/rustronomy-core/resources/tags.csv"
tag_output = "/rustronomy-core/src/universal_containers/tags.rs"

#read input csv file:
# col0: tag string
# col1: type name
# col2: description string
# col3: tag inner type
tags = np.loadtxt(cwd + tag_input, comments="#", dtype=str, delimiter=',')

#turn tags into rust code
with open(cwd + tag_output, "wb") as out:
  #start by printing the header:
  out.write(f"""/*
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
*/""".encode())
  
  #now we add the doc comments
  out.write(f"""
//! The currently restricted tags are:
//! | restricted tag string | restricted tag struct | description |
//! | --: | :--: | :-- |
""".encode())
  #fill the table with info
  for tag_string, type_name, description, inner_type in tags:
    out.write(f"""//! | `"{tag_string}"` | `{type_name}({inner_type})` | {description} |\n""".encode())
  out.write(b"""
use super::metadata::{MetaDataTag, TagError, PubContainer, private_container::PrivContainer};
use std::str::FromStr;\n
""")
  
  #next we add them to the restricted tags list
  num_tags = len(tags)
  out.write(f"pub const RESTRICTED_TAGS: [&str;{num_tags}] = [\n".encode())
  for tag_string,_,_,_ in tags:
    out.write(f"""  "{tag_string}",\n""".encode())
  out.seek(-2, 2)
  out.write(b"\n];\n")
  
  #finally we define the actual structs
  for tag_data in tags:
    tag_string, type_name, _, inner_type = tag_data[0], tag_data[1], tag_data[2], tag_data[3]
    out.write(f"""
#[derive(Debug)]
struct {type_name}({inner_type});

impl From<{inner_type}> for {type_name} {{
  fn from({tag_string}: {inner_type}) -> Self {{{type_name}({tag_string})}}
}}

impl From<{type_name}> for {inner_type} {{
  fn from({tag_string}: {type_name}) -> Self {{{tag_string}.0}}
}}

impl MetaDataTag for {type_name} {{
  const KEY: &'static str = "{tag_string}";
  type ValueType = {inner_type};

  fn parse_str(text: &str) -> Result<Self, <Self::ValueType as FromStr>::Err> {{
    match text.parse::<{inner_type}>() {{
      Ok({tag_string}) => Ok({tag_string}.into()),
      Err(err) => Err(err)
    }}
  }}
  
  fn to_string(&self) -> String {{ format!("{{}}", self.0)}}
}}\n""".encode())
  
  #Now we generate the MetaDataContainer trait
  out.write(f"""
pub trait MetaDataContainer: PrivContainer + PubContainer {{""".encode());
  for tag_string, type_name, _, inner_type in tags:
    out.write(f"""
fn remove_{tag_string}(&mut self) -> Result<{inner_type}, TagError> {{
  match self.remove_tag::<{type_name}>() {{
    Ok(tag) => Ok(tag.into()),
    Err(err) => Err(err)
  }}
}}
fn insert_{tag_string}(&mut self, {tag_string}: {inner_type}) -> Result<Option<{inner_type}>, TagError> {{
  match self.insert_tag::<{type_name}>({tag_string}.into()) {{
    Ok(Some(tag)) => Ok(Some(tag.into())),
    Ok(None) => Ok(None),
    Err(err) => Err(err)
  }}
}}
fn has_{tag_string}(&self) -> bool {{
  self.has_tag::<{type_name}>()
}}
""".encode())
  out.write(b"\n}")