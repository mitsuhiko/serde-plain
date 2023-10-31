use serde_derive::{Deserialize, Serialize};
use serde_plain::{FromStrDeserialize, SerializeDisplay};

#[derive(SerializeDisplay, FromStrDeserialize, Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Test {
    FooBarBaz,
    BlahBlah,
}

#[test]
fn test_forward_basics() {
    assert_eq!(Test::FooBarBaz.to_string(), "foo_bar_baz");
    assert_eq!("foo_bar_baz".parse::<Test>().unwrap(), Test::FooBarBaz);
}
