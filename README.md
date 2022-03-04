![](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_dark.png?raw=true#gh-light-mode-only)
![](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_light.png?raw=true#gh-dark-mode-only)
# Rustronomy - an astronomy toolkit written in rust
Rustronomy aims to provide easy-to-use modular libraries for working with astronomical datasets in Rust. These include I/O libraries for commonly used data storage formats like FITS, as well as image processing tools for radio astronomy and high-contrast imageing (WIP).

### ndarray and jupyter notebook integration
Rustronomy represents image and table-like data types using ![ndarray](https://crates.io/crates/ndarray), which promises to provide similar functionality for rust programs as is currently avaliable in python using numpy.

With the introduction of a ![rust jupyter kernel](https://github.com/google/evcxr/tree/main/evcxr_jupyter) by Google, the combination of rustronomy, ndarray and a plottling libary (like ![plotters](https://github.com/38/plotters)) promises to provide a high-performance replacement for current python tools.

### Current features
Rustronomy features are packaged in seperate crates: (most of these are still WIP)
- **rustronomy-core**: provides shared core functionality (dependency)
- **rustronomy-fits**: provides I/O functionality for FITS files.
- **rustronomy-sadf**: provides I/O functionality for the Simple Astronomical Data Format (SADF)

### Planned features
Planned features have not been divided into crates yet (all of these are WIP):
- mutli-dim fast fourier transforms for image processing
- radio astronomy data processing tools
- high contrast imageing processing tools
- spectroscopy tools (fitting line profiles and so-on)
- star-field analysis tools
