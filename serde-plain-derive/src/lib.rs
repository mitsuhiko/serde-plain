use proc_macro::TokenStream;

fn get_ident(input: TokenStream) -> String {
    let input = input.to_string();
    
    let mut x = 9;
    let mut start_i = input
        .find(" pub enum");
    if start_i.is_none() {
        x = 5;
        start_i = input.find(" enum")
    }
    let start_i = start_i.unwrap() + x;

    let end_i = input[start_i..]
        .find("{")
        .unwrap();

    input[start_i..start_i + end_i].trim().to_string()
}

/// Implements [`FromStr`](std::str::FromStr) for a type that forwards to [`Deserialize`](serde::Deserialize).
///
/// ```rust
/// # #[macro_use] extern crate serde_derive;
/// use serde::Deserialize;
/// use serde_plain::DeserializeString;
/// # fn main() {
///
/// #[derive(DeserializeString, Deserialize, Debug)]
/// pub enum MyEnum {
///     VariantA,
///     VariantB,
/// }
/// # }
/// ```
/// This automatically implements [`FromStr`](std::str::FromStr) which will
/// invoke the [`from_str`](crate::from_str) method from this crate.
/// 
/// Note: Custom error types are not supported. Use [`derive_display_from_serialize`](crate::derive_display_from_serialize) instead.
#[proc_macro_derive(DeserializeString)]
pub fn derive_deserialize_string(input: TokenStream) -> TokenStream {
    let ident = get_ident(input);
    format!(r#"
        impl ::std::str::FromStr for {0} {{
            type Err = ::serde_plain::Error;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {{
                ::serde_plain::from_str(s)
            }}
        }}
    "#, ident).parse().unwrap()
}

/// Implements [`Display`](std::fmt::Display) for a type that forwards to [`Serialize`](serde::Serialize).
///
/// ```rust
/// # #[macro_use] extern crate serde_derive;
/// use serde::Serialize;
/// use serde_plain::SerializeDisplay;
/// # fn main() {
///
/// #[derive(SerializeDisplay, Serialize, Debug)]
/// pub enum MyEnum {
///     VariantA,
///     VariantB,
/// }
/// # }
/// ```
///
/// This automatically implements [`Display`](std::fmt::Display) which will
/// invoke the [`to_string`](crate::to_string) method from this crate. In case
/// that fails the method will panic.
#[proc_macro_derive(SerializeDisplay)]
pub fn derive_serialize_display(input: TokenStream) -> TokenStream {
    let ident = get_ident(input);
    format!(r#"
        impl ::std::fmt::Display for {0} {{
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{
                write!(f, "{{}}", ::serde_plain::to_string(self).unwrap())
            }}
        }}
    "#, ident).parse().unwrap()
}