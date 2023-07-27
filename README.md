# Name (derive-name)

[![CI](https://github.com/abineo-ag/derive-name/actions/workflows/ci.yml/badge.svg)](https://github.com/abineo-ag/derive-name/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/derive-name.svg)](https://crates.io/crates/derive-name)
[![Docs](https://docs.rs/derive-name/badge.svg)](https://docs.rs/derive-name)

Derive macro to get the name of a struct or enum.

## Usage

```rust
use derive_name::Name;

#[derive(Name)]
struct Alice;

#[derive(Name)]
enum Bob {}

assert_eq!(Alice::name(), "Alice");
assert_eq!(Bob::name(), "Bob");
```

## Usage with `Named`

```rust
use derive_name::Named;

#[derive(derive_name::Name)]
struct Alice;

#[derive(derive_name::Name)]
enum Bob {
    Variant
}

let her = Alice {};
let his = Bob::Variant;

assert_eq!(her.name(), "Alice");
assert_eq!(his.name(), "Bob");
```

## Contributing

If you think you found a bug: [open a issue](https://github.com/abineo-ag/derive-name/issues).
Feature request are also welcome.

## License

This library is distributed under the terms of the [ISC License](https://github.com/abineo-ag/derive-name/blob/main/LICENSE).  
Find an easy explanation on [choosealicense.com/licenses/isc](https://choosealicense.com/licenses/isc/).
