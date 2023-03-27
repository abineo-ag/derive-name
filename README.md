# Name (derive-name)

[![CI](https://github.com/abineo-ag/derive-name/actions/workflows/ci.yml/badge.svg)](https://github.com/abineo-ag/derive-name/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/derive-name.svg)](https://crates.io/crates/derive-name)
[![Docs](https://docs.rs/derive-name/badge.svg)](https://docs.rs/derive-name)

Derive macro to get name of type (struct/enum) as String.

## Usage

```rust
use derive_name::Name;

#[derive(Name)]
struct MyStruct;

assert_eq!(MyStruct::name(), "MyStruct");
```

### Manual implementation

```rust
use derive_name::Name;

struct MyStruct;

impl Name for MyStruct {
    fn name() -> &'static str {
        "Banana"
    }
}

assert_eq!(MyStruct::name(), "Banana");
```

## Contributing

If you think you found a bug: [open a issue](https://github.com/abineo-ag/derive-name/issues).
Feature request are also welcome.

## License

This library is distributed under the terms of the [ISC License](https://github.com/abineo-ag/derive-name/blob/main/LICENSE).  
Find an easy explanation on [choosealicense.com/licenses/isc](https://choosealicense.com/licenses/isc/).
