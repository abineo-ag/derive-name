//! # Name
//!
//! Derive macro to get the name of a struct or enum.
//!
//! ## Usage
//!
//! ```
//! use derive_name::Name;
//!
//! #[derive(Name)]
//! struct Alice;
//!
//! #[derive(Name)]
//! enum Bob {}
//!
//! assert_eq!(Alice::name(), "Alice");
//! assert_eq!(Bob::name(), "Bob");
//! ```
//!
//! ## Usage with [`Named`]
//!
//! ```
//! use derive_name::Named;
//!
//! #[derive(derive_name::Name)]
//! struct Alice;
//!
//! #[derive(derive_name::Name)]
//! enum Bob {
//!     Variant
//! }
//!
//! let her = Alice {};
//! let his = Bob::Variant;
//!
//! assert_eq!(her.name(), "Alice");
//! assert_eq!(his.name(), "Bob");
//! ```

pub use derive_name_macros::Name;

pub trait Name {
    fn name() -> &'static str;
}

pub trait Named {
    fn name(&self) -> &'static str;
}

impl<T: Name> Named for T {
    fn name(&self) -> &'static str {
        T::name()
    }
}

#[cfg(test)]
mod as_function {
    use super::Name;
    use crate as derive_name;

    #[derive(Name)]
    struct Struct;

    #[derive(Name)]
    enum Enum {}

    #[test]
    fn test() {
        assert_eq!(Struct::name(), "Struct");
        assert_eq!(Enum::name(), "Enum");
    }
}

#[cfg(test)]
mod as_method {
    use super::Named;
    use crate as derive_name;

    #[derive(derive_name::Name)]
    struct Struct;

    #[derive(derive_name::Name)]
    enum Enum {
        A
    }

    #[test]
    fn test() {
        assert_eq!(Struct.name(), "Struct");
        assert_eq!(Enum::A.name(), "Enum");
    }
}
