#[macro_use]
extern crate serde_derive;
extern crate serde_plain;

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Test {
    FooBarBaz,
    BlahBlah,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct NewInt(pub i32);

#[test]
fn test_basics() {
    assert_eq!(serde_plain::from_str::<&str>("aha").unwrap(), "aha");
    assert_eq!(serde_plain::from_str::<i32>("42").unwrap(), 42);
    assert_eq!(serde_plain::from_str::<bool>("true").unwrap(), true);
    assert_eq!(serde_plain::from_str::<bool>("false").unwrap(), false);
    assert_eq!(serde_plain::from_str::<()>("").unwrap(), ());
    assert_eq!(serde_plain::from_str::<Option<()>>("").unwrap(), None);
    assert_eq!(
        serde_plain::from_str::<Option<String>>("42").unwrap(),
        Some("42".into())
    );
    assert_eq!(
        serde_plain::from_str::<Option<&str>>("42").unwrap(),
        Some("42")
    );
    assert_eq!(
        serde_plain::from_str::<Test>("blah_blah").unwrap(),
        Test::BlahBlah
    );
    assert_eq!(serde_plain::from_str::<NewInt>("42").unwrap(), NewInt(42));
}
