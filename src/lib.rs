//! # Name
//!
//! Derive macro to get name of type (struct/enum) as String.
//!
//! ## Usage
//!
//! ```
//! use derive_name::Name;
//!
//! #[derive(Name)]
//! struct MyStruct;
//!
//! assert_eq!(MyStruct::name(), "MyStruct");
//! ```
//!
//! ### Manual implementation
//!
//! ```
//! use derive_name::Name;
//!
//! struct MyStruct;
//!
//! impl Name for MyStruct {
//!     fn name() -> &'static str {
//!         "Banana"
//!     }
//! }
//!
//! assert_eq!(MyStruct::name(), "Banana");
//! ```
//!

pub use derive_name_macros::Name;

pub trait Name {
    fn name() -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::Name;

    struct StructManual;

    impl Name for StructManual {
        fn name() -> &'static str {
            "success"
        }
    }

    enum EnumManual {}

    impl Name for EnumManual {
        fn name() -> &'static str {
            "success"
        }
    }

    #[test]
    fn manual() {
        assert_eq!(StructManual::name(), "success");
        assert_eq!(EnumManual::name(), "success");
    }

    #[derive(Name)]
    struct Struct;

    #[derive(Name)]
    enum Enum {}

    #[test]
    fn derived() {
        assert_eq!(Struct::name(), "Struct");
        assert_eq!(Enum::name(), "Enum");
    }
}
