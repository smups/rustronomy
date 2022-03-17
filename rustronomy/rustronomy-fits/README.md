![rustronomy_dark_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy-fits_github_banner_dark.png?raw=true#gh-light-mode-only)
![rustronomy_light_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy-fits_github_banner_light.png#gh-dark-mode-only)

# Rustronomy-fits - a Rustronomy tool for FITS file I/O
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

_This crate is part of the Rustronomy Project_
Rustronomy-fits provides I/O tools for reading, writing and parsing FITS files. It is currently still under heavy development.

### Features/Roadmap
Rustronomy-fits will support the following features (features marked with ✔️ have already been implemented):

_Reading Fits Files_
- ✔️ Parsing FITS headers and keyword records
- Parsing data in the primary HDU
  - ✔️ Images
  - ❌ Tables
  - ❌ Random Groups
- Parsing Extensions
  - ✔️ Images
  - ❌ Tables
 
_Writing Fits Files_
- ✔️ Writing existing FITS files to disk
- Constructing a HDU (including auto header) from
  - ❌ an ndarray (to an Image)
  - ❌ a table (to a table)

# Quickstart
To use Rustronomy-fits in a cargo project, add the rustronomy-fits crate as a dependency to your `Cargo.toml` file:
```toml
[dependencies]
rustronomy-fits = "0.1.0"
```
To use Rustronomy-fits in a Jupyter notebook, execute a cell containing the following code:
```rust
:dep rustronomy-fits = {git = "https://github.com/smups/rustronomy"}
```
### Short example: Importing a FITS image as a ndarray
In this example, we create a Fits struct using the `open()` method, which takes
the path to the file as an argument. Next, we get a reference to the second
header-data-unit (HDU) in the file, wich contains an `f64` encoded Image
extension. We can get the Image from the HDU by matching the data contained in
the HDU with the `Extension::Image` variant and then calling `as_f64_array()` on
the unwrapped Image.
```rust
use rustronomy_fits::prelude::*;

let fits = Fits::open(&Path::from("somefile.fits"))?;
let data_array = match fits.get_hdu(1).unwrap().get_data() {
  Extension::Image(img) => img.as_f64_array()?,
  _ => panic!()
}; 
```
>**Check out the examples folder in the repo root for Jupyter notebooks with more
in-depth explanations and examples!**

# Contributing
### Notes on testing
If you want to contribute to this module, please keep in mind the following points regarding testing:
- all I/O tests make use of the `resources/tests/` folder, located in the root of the `rustronomy_fits/` folder.
- rustronomy_fits uses the python package astropy as a reference to validate FITS files written during tests. Make sure to setup a python virtual environment with astropy and numpy installed to run these tests.
- to test the reading functionalities of rustronomy_fits, a number of test FITS files are used during testing. These are too large to be uploaded to github. You can download them using this link: [TODO]. Be sure to place them in the `resources/tests/` folder.

# License
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

_Rustronomy-fits is part of the Rustronomy project and inherits the gpl license
from the overarching Rustronomy project_

>**Rustronomy is explicitly not licensed under the dual
Apache/MIT license common to the Rust ecosystem. Instead it is licensed under
the terms of the [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.html)**.

Rustronomy is a science project and embraces the values of open science and free
and open software. Closed and paid scientific software suites hinder the
development of new technologies and research methods, as well as diverting much-
needed public funds away from researchers to large publishing and software
companies.

>Rustronomy-fits is free software.
It is licensed under the GNU GPL version 3 or later.
That means you are free to use this program for any purpose;
free to study and modify this program to suit your needs;
and free to share this program or your modifications with anyone.
If you share this program or your modifications
you must grant the recipients the same freedoms.
To be more specific: you must share the source code under the same license. For details see https://www.gnu.org/licenses/gpl-3.0.html or the LICENSE file in this
github repository.
