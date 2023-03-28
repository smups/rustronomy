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

//! This module specifies a generic array container. In contrast to an Image, an
//! Array may have any number of axes. This comes at some allocation cost, see the
//! ndarray docs for details.

use std::{
  fmt::{self, Debug, Display, Formatter}
};

use ndarray as nd;

use crate::meta::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DataArray<T> {
  pub(crate) data: nd::Array<T, nd::IxDyn>,
  pub(crate) meta: super::MetaOnly,
}

impl<R> MetaDataContainer for DataArray<R> where R: Debug + Clone {
  impl_meta_container!{}
}