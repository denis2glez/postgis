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

## Usage

TODO:

## License

This project is licensed under the [MIT](LICENSE) license.
