![rustronomy_dark_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_dark.png?raw=true#gh-light-mode-only)
![rustronomy_light_banner](https://github.com/smups/rustronomy/blob/main/logos/Rustronomy_github_banner_light.png#gh-dark-mode-only)
# The Rustronomy Project - an astrophysics toolkit written in Rust
Rustronomy is an astrophysics Rust project aimed at providing astrophysicists with the tools needed to perform lightning-fast statistical analysis and computational modelling, powered by the Rust programming language. Rust makes low-level programming accessible to non-computer scientists. Over the last few years more and more scientific computing tools written in rust have popped up: we can now make very nice plots with [plotters](https://github.com/38/plotters), perform lightning-fast array calculations using [ndarray](https://github.com/rust-ndarray/ndarray) and even write Rust code in Jupyter notebooks since the introduction of a [rust kernel by google](https://github.com/google/evcxr).

Rustronomy crates build upon the currently existing, frequently used crates in the Rust ecosystem to provide a standardised toolset for astronomy. This way, all crates in the Rustronomy ecosystem are interoperable with each other and with external crates using the same dependencies as the Rustronomy project. Another key goal of Rustronomy is interoperability of Rustronomy crates with Python and its extensive existing scientific computing ecosystem. It is already quite easy to create Python bindings for Rust crates, and Rustronomy crates with unique capabilities are encouraged to also include Python bindings.

### Organisation
Rustronomy is a collection of interoperable crates from many different fields. Any crate that makes use of other Rustronomy crates and follows the [guidelines]() can be added to the Rustronomy repository. Each Rustronomy crate may have its own maintainers.

### Current Features
Rustronomy is currently divided in three crates, each providing the following features:
- **rustronomy-core**: common dependency,
- **rustronomy-fits**: utility for reading and writing FITS files,
- **rustronomy-asdf**: utility for reading and writing ASDF (Advanced Scientific Data Format) files
Note: some of these crates are not finished yet, see their respective pages for more info on their progress.

### Planned Features
Currently, the crates listed above are not finished yet, so any work on future expansions is postponed until those are finished (barred from the asdf crate since not that many people use asdf files yet). If anyone wishes to contribute code for these projects, feel free to do so if you are prepared to maintain your own Rustronomy component.
Sugessted Features:
- spectroscopy tools
- high-contrast imaging tools
- star-field tools (calculating star fluxes from a field etc...)
- stellar evolution tools / bindings to MESA
- ...and many more crates
