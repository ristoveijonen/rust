# Packages and crates
Rust packages:
* Can have two types of crates.
  * library
  * binary
* Must have atleas one crate.
* Cannot have more than 1 library crate.
* Has a Cargo.toml-file which instructs how to build the crates included in it.

Rust crates are separate files inside a package.

Crates have root files for both library (lib.rs) and binary (main.rs) crates. If a package has both inside the src/lib directory, both will be ran automatically by rust.