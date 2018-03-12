# Serde Plain

This crate implements a plain text serializer and deserializer. It can only
serialize and deserialize primitives and derivatives thereof (like basic enums
or newtypes). It internally uses the FromStr and Display trait to convert
objects around.
