# postgis

## Table of Contents
- [postgis](#postgis)
  - [Table of Contents](#table-of-contents)
  - [About](#about)
  - [Prerequisites](#prerequisites)
  - [Installing](#installing)
  - [Setup](#setup)
  - [Quick check](#quick-check)
  - [Build](#build)
  - [Run tests](#run-tests)
  - [Run a binary](#run-a-binary)
  - [Run an example](#run-an-example)
  - [Run a benchmark](#run-a-benchmark)
  - [License](#license)

## About

A collection of demos, tests, and benchmarks for exploring Rust’s support of geospatial data processing, combined
with Computer Vision and Machine Learning.

## Prerequisites

You need to have the following dependencies installed

- [Rust](https://www.rust-lang.org/tools/install).
- [PostgreSQL](https://www.postgresql.org/download).
- [PostGIS](https://postgis.net/install) extension.
- SQLx's associated command-line utility [`sqlx-cli`](https://crates.io/crates/sqlx-cli).
- [GDAL](https://gdal.org/download.html).
- [OpenCV](https://opencv.org/releases).
- [TensorFlow](https://www.tensorflow.org/install) with Python support.

## Installing

> The dependencies specified after `opencv` (e.g., `clang` or `llvm`) are only required to generate
> the Rust bindings to the installed OpenCV library.

> Note that TensorFlow's recommended installation is via Python's pip package manager.

<details open>
<summary><b>Arch Linux</b></summary>

If you are using Arch Linux or a derivative, you could install all the required dependencies by
running the following commands.
```sh
sudo pacman -S rust postgresql postgis gdal python-tensorflow opencv clang
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```
> If you have an NVIDIA GPU, instead of `python-tensorflow` you can choose the `python-tensorflow-cuda`
> package.

</details>

<details open>
<summary><b>Debian</b></summary>

If you are using Debian or a derivative (e.g. Ubuntu, Linux Mint), it is recommended to install Rust
using the standard installation script. You could install all the development dependencies by running
the following commands.
```sh
# sqlx-cli needs libssl-dev
sudo apt install postgresql postgis curl libssl-dev libopencv-dev libgdal-dev clang libclang-dev
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
# Install TensorFlow
pip install --upgrade pip
pip install tensorflow
```
</details>

<details open>
<summary><b>macOS</b></summary>

If you are using macOS you could install all the development dependencies using [Homebrew](https://brew.sh)
by running the following commands.
```sh
brew install curl postgresql postgis gdal python opencv llvm
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
# Install TensorFlow
pip install --upgrade pip
pip install tensorflow
```
</details>

<details open>
<summary><b>Windows</b></summary>

If you are using Windows, you could install most of the required dependencies using
[Chocolatey](https://chocolatey.org) by running the following commands.

```sh
choco install rust-ms postgresql python opencv llvm
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
# Install TensorFlow
pip install --upgrade pip
pip install tensorflow
```

After installing PostgreSQL, remember to run the "StackBuilder" utility to install the PostGIS add-on.

> GDAL on Windows requires manual installation or to use a tool like `vcpkg`.

</details>

## Setup

Download the zip file containing all the test data by running the script

```sh
scripts/download_data.sh
```

or if you prefer manually from the following [link](https://drive.google.com/file/d/1Vimn78opM6jMIdWoR3Hznqu2RbzrmOY5/view?usp=sharing").

## Quick check

Quickly check the package and all of its dependencies for possible errors.
```sh
cargo check
```

## Build

To build the application on your host machine use

```sh
cargo build
```

## Run tests

Now you can run all the default tests

```sh
cargo test
```
or just a specific group of tests, by adding `-- <pattern>` to filter. For instance,

```sh
cargo test -- parse_wkt
```

## Run a binary

To execute a specific binary you can use

```sh
cargo run --bin <bin_name>
```
For instance,

```sh
cargo run --bin geo_centroid
```

## Run an example

To run an example, you could run

```sh
cargo run --example <example_name>
```
For instance,

```sh
cargo run --example opencv_text_detection
```

## Run a benchmark

To execute a micro-benchmark use

```sh
cargo bench --bench <bench_name>
```
For example,

```sh
cargo bench --bench geo_contains
```

## License

This project is licensed under the [MIT](LICENSE) license.
