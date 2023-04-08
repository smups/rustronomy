![rustronomy_dark_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_dark.png?raw=true#gh-light-mode-only)
![rustronomy_light_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_light.png#gh-dark-mode-only)
# The Rustronomy Project - an astrophysics toolkit written in Rust
[![License: EUPL v1.2](https://img.shields.io/badge/License-EUPLv1.2-blue.svg)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
[![Crates.io](https://img.shields.io/crates/v/rustronomy-core)](https://crates.io/crates/rustronomy-core)
![](https://img.shields.io/crates/d/rustronomy-core)
### _this is the homepage of rustronomy-core, the shared core dependency for all rustronomy crates_
The `rustronomy-core` crate provides types and traits that ensure interoperability between rustronomy crates. This crate should be updated very infrequently since all other rustronomy crates depend on its public API and breaking changes would require a large rewrite. Therefore, the functionality provided by `rustronomy-core` should be kept to a minimum. 

# Functionality [(read the docs)](https://docs.rs/rustronomy-core/)
`rustronomy-core` currently provides traits and structs for working with
cross-storage-format metadata, as well as an unstable data container to be used
for representing table-like (textual) data.

See module docs for more info.

# License
[![License: EUPL v1.2](https://img.shields.io/badge/License-EUPLv1.2-blue.svg)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)

All crates in the Rustronomy ecosystem are licensed under the EUPLv1.2 (or higher)
license.
>**Rustronomy is explicitly not licensed under the dual
Apache/MIT license common to the Rust ecosystem. Instead it is licensed under
the terms of the [European Union Public License v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)**.

Rustronomy is a science project and embraces the values of open science and free
and open software. Closed and paid scientific software suites hinder the
development of new technologies and research methods, as well as diverting much-
needed public funds away from researchers to large publishing and software
companies.

See the [LICENSE.md](../LICENSE.md) file for the EUPL text in all 22 official
languages of the EU, and [LICENSE-EN.txt](../LICENSE-EN.txt) for a plain text
English version of the license.