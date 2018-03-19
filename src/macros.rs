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
///
/// A third form with a conversion function as second argument is supported.
/// The closure needs to be in the form `|err| -> ErrType { ... }`:
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
/// #[derive(Debug)]
/// pub struct MyError(String);
///
/// forward_from_str_to_serde!(MyEnum, |err| -> MyError { MyError(err.to_string()) });
/// # }
/// ```
macro_rules! forward_from_str_to_serde {
    ($type:ty) => {
        impl ::std::str::FromStr for $type {
            type Err = $crate::Error;
            fn from_str(s: &str) -> ::std::result::Result<$type, Self::Err> {
                $crate::from_str(s)
            }
        }
    };
    ($type:ty, |$var:ident| -> $err_type:ty { $err_conv:expr }) => {
        impl ::std::str::FromStr for $type {
            type Err = $err_type;
            fn from_str(s: &str) -> ::std::result::Result<$type, Self::Err> {
                $crate::from_str(s).map_err(|$var| ($err_conv))
            }
        }
    };
    ($type:ty, $err_type:ty) => {
        impl ::std::str::FromStr for $type {
            type Err = $err_type;
            fn from_str(s: &str) -> ::std::result::Result<$type, Self::Err> {
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

/// Derives `serde::Deserialize` a type that implements `FromStr`.
///
/// ```rust
/// use std::str::FromStr;
/// use std::num::ParseIntError;
/// #[macro_use] extern crate serde;
/// #[macro_use] extern crate serde_plain;
/// # fn main() {
///
/// pub struct MyStruct(u32);
///
/// impl FromStr for MyStruct {
///     type Err = ParseIntError;
///     fn from_str(value: &str) -> Result<MyStruct, Self::Err> {
///         Ok(MyStruct(value.parse()?))
///     }
/// }
///
/// derive_deserialize_from_str!(MyStruct, "valid positive number");
/// # }
/// ```
///
/// This automatically implements `fmt::Serialize` which will invoke the
/// `from_str` function on the target type internally. First argument is
/// the name of the type, the second is a message for the expectation
/// error (human readable type effectively).
#[macro_export]
macro_rules! derive_deserialize_from_str {
    ($type:ty, $expectation:expr) => {
        impl<'de> ::serde::de::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                struct V;

                impl<'de> ::serde::de::Visitor<'de> for V {
                    type Value = $type;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str($expectation)
                    }

                    fn visit_str<E>(self, value: &str) -> ::std::result::Result<$type, E>
                    where
                        E: ::serde::de::Error,
                    {
                        value
                            .parse()
                            .map_err(|_| ::serde::de::Error::invalid_value(
                                ::serde::de::Unexpected::Str(value), &self))
                    }
                }

                deserializer.deserialize_str(V)
            }
        }
    }
}

/// Derives `serde::Serialize` a type that implements `fmt::Display`.
///
/// ```rust
/// use std::fmt;
/// #[macro_use] extern crate serde;
/// #[macro_use] extern crate serde_plain;
/// # fn main() {
///
/// pub struct MyStruct(u32);
///
/// impl fmt::Display for MyStruct {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "{}", self.0)
///     }
/// }
///
/// derive_serialize_from_display!(MyStruct);
/// # }
/// ```
///
/// This automatically implements `fmt::Serialize` which will invoke the
/// `to_string` method on the target.
#[macro_export]
macro_rules! derive_serialize_from_display {
    ($type:ty) => {
        impl ::serde::ser::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
    }
}
