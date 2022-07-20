![rustronomy_dark_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_dark.png?raw=true#gh-light-mode-only)
![rustronomy_light_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_light.png#gh-dark-mode-only)
# `rustronomy-core` changelog
_breaking releases are marked with ❗️, feature updates are marked with 🌟 and
bug-fix updates are marked with 👾_

## v0.3.0 array bugfix and additional restricted metadata tags (❗️👾)
This update introduces a small but crucial bug fix in the array container
- Fixed typo in the `DataArray` container: dimensionality and type parameters
were swapped (❗️breaking change❗️)
In addition, the following restricted keywords were added/modified (keywords
are chosen to mirror those specified in the FITS standard):
- `date` now specifies the date the data was collected
- `last_modified` now specifies the date the container was last modified
- `object` specifies the object(s) observed to produce the data
- `organisation` specifies the organisation responsible for producing the data
- `telescope` specifies the telescope used to produce the data
- `instrument` specifies the instrument used to produce the data. This keyword is
to be used in conjunction with the `telescope` keyword
- `reference` specifies a reference to a publication accompanying the data. It is
recommended to use a [DOI](https://doi.org) or [ADS](https://ads.harvard.edu)
format.
- `exposure_time` specifies the exposure time in seconds of the image

Other time and position related keywords as specified by the FITS standard may
be added in the future. 

## v0.2.1 - visibility fix (👾)
This tiny update adds a re-export of some universal data containers to the 
`universal_containers` module.

## v0.2.0 - universal data containers (❗️🌟)
This update introduces universal data containers which can be used to share data
between rustronomy crates. The datacontainers feature support for metadata.
### 🌟Feature updates🌟
added universal data containers:
- added `universal_container` module
- added `Image<T: Num>` container for images
- added `DataArray<T: Num>` container for higher-dimensional arrays
- added `Table` container consisting of named `Col`s of different types

added metadata support for universal containers:
- added `MetaDataTag<T>`. Used to add metadata of type `T` to a datacontainer.
- added reserved metadata tags. Reserved tags are to be encoded in special ways
if the file format that the universal data container is encoded into supports
reserved metadata tags. The following tags are reserved, as of `v0.2`:
  - `author` specifies author(s) of the datacontainer
  - `date` specifies date the container was last modified

### ❗️Breaking changes❗️
All encoding/decoding related functionality was removed from `rustronomy-core`
since it is not needed to ensure interoperability between rustronomy crates.
- removed `data_type_traits` module
- removed `Encode` trait and all implementations
- removed `Decode` trait and all implementations
- removed `EncodeAndConsume` trait

## v0.1.0 - initial release (🌟)
initial release