![rustronomy_dark_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_dark.png?raw=true#gh-light-mode-only)
![rustronomy_light_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_light.png#gh-dark-mode-only)
# `rustronomy-core` changelog
_breaking releases are marked with ❗️, feature updates are marked with 🌟 and bug-fix updates are marked with 👾_

## v0.2.0 - universal data containers (❗️🌟)
This update introduces universal data containers which can be used to share data between rustronomy crates. The datacontainers feature support for metadata.
### 🌟Feature updates🌟
added universal data containers:
- added `universal_container` module
- added `Image<T: Num>` container for images
- added `DataArray<T: Num>` container for higher-dimensional arrays
- added `Table` container consisting of named `Col`s of different types

added metadata support for universal containers:
- added `MetaDataTag<T>`. Used to add metadata of type `T` to a datacontainer.
- added reserved metadata tags. Reserved tags are to be encoded in special ways if the file format that the universal data container is encoded into supports reserved metadata tags. The following tags are reserved, as of `v0.2`:
  - `author` specifies author(s) of the datacontainer
  - `date` specifies date the container was last modified

### ❗️Breaking changes❗️
All encoding/decoding related functionality was removed from `rustronomy-core` since it is not needed to ensure interoperability between rustronomy crates.
- removed `data_type_traits` module
- removed `Encode` trait and all implementations
- removed `Decode` trait and all implementations
- removed `EncodeAndConsume` trait

## v0.1.0 - initial release (🌟)
initial release