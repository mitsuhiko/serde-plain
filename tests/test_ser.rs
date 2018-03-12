#[macro_use]
extern crate serde_derive;
extern crate serde_plain;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Test {
    FooBarBaz,
    BlahBlah,
}

#[derive(Serialize)]
pub struct NewInt(i32);

#[test]
fn test_basics() {
    assert_eq!(serde_plain::to_string(&42).unwrap(), "42");
    assert_eq!(serde_plain::to_string(&"blafasel").unwrap(), "blafasel");
    assert_eq!(
        serde_plain::to_string(&Test::FooBarBaz).unwrap(),
        "foo_bar_baz"
    );
    assert_eq!(
        serde_plain::to_string(&Test::BlahBlah).unwrap(),
        "blah_blah"
    );
    assert_eq!(serde_plain::to_string(&NewInt(42)).unwrap(), "42");
    assert_eq!(serde_plain::to_string(&Some(true)).unwrap(), "true");
    assert_eq!(serde_plain::to_string(&None::<()>).unwrap(), "");
    assert_eq!(serde_plain::to_string(&()).unwrap(), "");
}
