#[macro_export]
/// Implements `FromStr` for a type that forwards to serde.
///
/// ```rust
/// #[macro_use] extern crate serde_derive;
/// #[macro_use] extern crate serde_plain;
/// # fn main() {
///
/// #[derive(Deserialize, Debug)]
/// pub enum MyEnum {
///     VariantA,
///     VariantB,
/// }
///
/// forward_from_str_to_serde!(MyEnum);
/// # }
/// ```
///
/// This automatically implements `FromStr` which will invoke the
/// `from_str` method from this crate.
///
/// Additionally this macro supports a second argument which can be the
/// error type to use.  In that case `From<serde_plain::Error>` needs
/// to be implemented for that error.
macro_rules! forward_from_str_to_serde {
    ($type:ty) => {
        impl ::std::str::FromStr for $type {
            type Err = $crate::Error;
            fn from_str(s: &str) -> Result<$type, Self::Err> {
                $crate::from_str(s)
            }
        }
    };
    ($type:ty, $err_type:ty) => {
        impl ::std::str::FromStr for $type {
            type Err = $err_type;
            fn from_str(s: &str) -> Result<$type, Self::Err> {
                $crate::from_str(s).map_err(|e| e.into())
            }
        }
    };
}

#[macro_export]
/// Implements `fmt::Display` for a type that forwards to serde.
///
/// ```rust
/// #[macro_use] extern crate serde_derive;
/// #[macro_use] extern crate serde_plain;
/// # fn main() {
///
/// #[derive(Serialize, Debug)]
/// pub enum MyEnum {
///     VariantA,
///     VariantB,
/// }
///
/// forward_display_to_serde!(MyEnum);
/// # }
/// ```
///
/// This automatically implements `fmt::Display` which will invoke the
/// `to_string` method from this crate.  In case that fails the method
/// will panic.
macro_rules! forward_display_to_serde {
    ($type:ty) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", $crate::to_string(self).unwrap())
            }
        }
    }
}
