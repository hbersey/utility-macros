use convert_case::{Case, Casing as _};
use utility_macros_internals::utils::CaseExt as _;

#[test]
pub fn test_from_string() {
    assert_eq!(Case::from_string("upper".to_string()).unwrap(), Case::Upper);
    assert!(Case::from_string("invalid".to_string()).is_err());

}

#[test]
pub fn test_from_str() {
    assert_eq!(Case::from_str("upper").unwrap(), Case::Upper);
    assert_eq!(Case::from_str("UPPER").unwrap(), Case::Upper);
    assert_eq!(Case::from_str("lower").unwrap(), Case::Lower);
    assert_eq!(Case::from_str("title").unwrap(), Case::Title);
    assert_eq!(Case::from_str("Title").unwrap(), Case::Title);
    assert_eq!(Case::from_str("toggle").unwrap(), Case::Toggle);
    assert_eq!(Case::from_str("ToGgLe").unwrap(), Case::Toggle);
    assert_eq!(Case::from_str("camel").unwrap(), Case::Camel);
    assert_eq!(Case::from_str("pascal").unwrap(), Case::Pascal);
    assert_eq!(Case::from_str("Pascal").unwrap(), Case::Pascal);
    assert_eq!(Case::from_str("snake").unwrap(), Case::Snake);
    assert_eq!(Case::from_str("upper_snake").unwrap(), Case::UpperSnake);
    assert_eq!(Case::from_str("UPPER_SNAKE").unwrap(), Case::UpperSnake);
    assert_eq!(
        Case::from_str("screaming_snake").unwrap(),
        Case::ScreamingSnake
    );
    assert_eq!(
        Case::from_str("SCREAMING_SNAKE").unwrap(),
        Case::ScreamingSnake
    );
    assert_eq!(Case::from_str("kebab").unwrap(), Case::Kebab);
    assert_eq!(Case::from_str("cobol").unwrap(), Case::Cobol);
    assert_eq!(Case::from_str("train").unwrap(), Case::Train);
    assert_eq!(Case::from_str("Train").unwrap(), Case::Train);
    assert_eq!(Case::from_str("flat").unwrap(), Case::Flat);
    assert_eq!(Case::from_str("upper_flat").unwrap(), Case::UpperFlat);
    assert_eq!(Case::from_str("UPPER_FLAT").unwrap(), Case::UpperFlat);
    assert_eq!(Case::from_str("alternating").unwrap(), Case::Alternating);
    assert_eq!(Case::from_str("aLtErNaTiNg").unwrap(), Case::Alternating);
    assert!(Case::from_str("invalid").is_err());
}
