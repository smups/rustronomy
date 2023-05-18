#  Copyright© 2023 Raúl Wolters(1)
#
#  This file is part of rustronomy-core.
#
#  rustronomy is free software: you can redistribute it and/or modify it under
#  the terms of the European Union Public License version 1.2 or later, as
#  published by the European Commission.
#
#  rustronomy is distributed in the hope that it will be useful, but WITHOUT ANY
#  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
#  A PARTICULAR PURPOSE. See the European Union Public License for more details.
#
#  You should have received a copy of the EUPL in an/all official language(s) of
#  the European Union along with rustronomy.  If not, see 
#  <https://ec.europa.eu/info/european-union-public-licence_en/>.
#
#  (1) Resident of the Kingdom of the Netherlands; agreement between licensor and
#  licensee subject to Dutch law as per article 15 of the EUPL.

import os
import numpy as np

print("hello from the python script")
cwd = os.getcwd()

# input csv file for generating tag structs
tag_input = "/rustronomy-core/resources/tags.csv"
tag_output = "/rustronomy-core/src/meta/auto.rs"

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

use std::fmt::{{Display, Debug, Formatter, Result}};
use super::MetaTag;

""".encode())

  for type_name, inner_type, doc_str, fmt_str in tags:
    out.write(f"""#[derive(Debug, Clone, PartialEq)]
/// {doc_str}
pub struct {type_name}(pub {inner_type});
impl MetaTag for {type_name} {{}}
impl Display for {type_name} {{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {{
    write!(f, "[{fmt_str}]: \\"{{}}\\"", self.0)
  }}
}}

""".encode())