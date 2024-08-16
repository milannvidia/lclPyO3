# LclPy

LclPy is a localsearch library implemented in python and Rust.

- [Getting started](#getting-started)
  - [pip](#pip)
  - [Just build](#build-code)
  - [Install locally](#build-and-install-locally)
- [Testing](#testing)
  - [Rust code](#rust-code)
- [Authors](#authors)
- [Acknowledgments](#acknowledgments)

## Getting started

To install this package you have 3 options. When installed if you want to use the native python functionalities you can import them as normally. However if you want to use the Rust version of these functionalities you'll need to import as follows.
Make sure to use the "as" attribute to mitigate confusion later.

```
from lclPyO3 import lclPyO3 as lcl_rust
```

### pip

### Build code

To build the code **but not install it locally in your virtual environment**. You will need to install maturin. (The release flag is the same as for Rust).

```
pip install maturin
maturin build --release
```

The generated code will be in the folder target/release

### Build and install locally

To install the code locally in your virtual environment you will need:

- To create a virtual environment
- To install maturin.
- To develop.

```
python m .env
//activate virtual environment
pip install maturin
maturin develop --release
```

The generated code will be in the folder target/release and will automatically be added as a package in your .env/Lib/site-packages.

## Testing

### Rust code

To test the Rust code

```
cargo test
//if doc tests didn't automatically run
cargo test --doc
```

## Authors

- **Daan Thijs** - _Design and implementation Python_ - [@nobody1570](https://github.com/nobody1570)

- **Milan Schollier** - _Design and implementation Rust and Pyo3 Integration_ - [@milannvidia](https://github.com/milannvidia)

## Acknowledgments

- **Tony Wauters** - _Master's Thesis Coordinator_ - [KuLeuven](https://www.kuleuven.be/wieiswie/nl/person/00069609)
