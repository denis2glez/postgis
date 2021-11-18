# postgis

## About

A collection of demos, tests, and benchmarks for exploring Rust's support of geospatial data
processing.

## Prerequisites

You need to have the following dependencies installed

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download) 
- [PostGIS](https://postgis.net/install) extension.
- SQLx's associated command-line utility [`sqlx-cli`](https://crates.io/crates/sqlx-cli).

## Installing

<details open>
<summary><b>Arch Linux</b></summary>

If you are using Arch Linux or a derivative, you could install all the required dependencies by
running the following commands.
```sh
sudo pacman -S rust postgresql postgis
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```
</details>

<details open>
<summary><b>Debian</b></summary>

If you are using Debian or a derivative (e.g. Ubuntu, Linux Mint), it is recommended to install Rust
using the standard installation script. You could install all the development dependencies by running
the following commands.
```sh
# sqlx-cli needs libssl-dev
sudo apt install postgresql postgis curl libssl-dev
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```
</details>

<details open>
<summary><b>macOS</b></summary>

If you are using macOS you could install all the development dependencies using [Homebrew](https://brew.sh)
by running the following commands.
```sh
brew install curl postgresql postgis
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```
</details>

<details open>
<summary><b>Windows</b></summary>

If you are using Windows, you could install all the required dependencies using the
[`winget`](https://docs.microsoft.com/en-us/windows/package-manager/winget/#production-recommended)
CLI tool by running the following commands.

```sh
winget install --id Rustlang.Rust.MSVC
winget install --id PostgreSQL.PostgreSQL
# Install sqlx-cli only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```
</details>

## Usage

TODO:

## License

This project is licensed under the [MIT](LICENSE) license.
