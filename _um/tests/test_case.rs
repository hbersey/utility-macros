use convert_case::Case;
use utility_macros_internals::case::parse_case;

#[test]
fn test_parse_case() {
    assert_eq!(parse_case("upper"), Ok(Case::Upper));
    assert_eq!(parse_case("UPPER"), Ok(Case::Upper));

    assert_eq!(parse_case("lower"), Ok(Case::Lower));

    assert_eq!(parse_case("title"), Ok(Case::Title));
    assert_eq!(parse_case("Title"), Ok(Case::Title));

    assert_eq!(parse_case("toggle"), Ok(Case::Toggle));
    assert_eq!(parse_case("ToGgLe"), Ok(Case::Toggle));

    assert_eq!(parse_case("camel"), Ok(Case::Camel));

    assert_eq!(parse_case("pascal"), Ok(Case::Pascal));
    assert_eq!(parse_case("Pascal"), Ok(Case::Pascal));

    assert_eq!(parse_case("snake"), Ok(Case::Snake));

    assert_eq!(parse_case("upper_snake"), Ok(Case::UpperSnake));
    assert_eq!(parse_case("UPPER_SNAKE"), Ok(Case::UpperSnake));

    assert_eq!(parse_case("screaming_snake"), Ok(Case::ScreamingSnake));
    assert_eq!(parse_case("SCREAMING_SNAKE"), Ok(Case::ScreamingSnake));

    assert_eq!(parse_case("kebab"), Ok(Case::Kebab));

    assert_eq!(parse_case("cobol"), Ok(Case::Cobol));

    assert_eq!(parse_case("train"), Ok(Case::Train));
    assert_eq!(parse_case("Train"), Ok(Case::Train));

    assert_eq!(parse_case("flat"), Ok(Case::Flat));

    assert_eq!(parse_case("upper_flat"), Ok(Case::UpperFlat));
    assert_eq!(parse_case("UPPER_FLAT"), Ok(Case::UpperFlat));

    assert_eq!(parse_case("alternating"), Ok(Case::Alternating));
    assert_eq!(parse_case("aLtErNaTiNg"), Ok(Case::Alternating));

    assert_eq!(parse_case("invalid"), Err(()));

}

