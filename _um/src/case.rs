use convert_case::Case;

pub fn parse_case(s: &str) -> Result<Case, ()> {
    match s.to_string().as_str() {
        "upper" | "UPPER" => Ok(Case::Upper),
        "lower" => Ok(Case::Lower),
        "title" | "Title" => Ok(Case::Title),
        "toggle" | "ToGgLe" => Ok(Case::Toggle),
        "camel" => Ok(Case::Camel),
        "pascal" | "Pascal" => Ok(Case::Pascal),
        "snake" => Ok(Case::Snake),
        "upper_snake" | "UPPER_SNAKE" => Ok(Case::UpperSnake),
        "screaming_snake" | "SCREAMING_SNAKE" => Ok(Case::ScreamingSnake),
        "kebab" => Ok(Case::Kebab),
        "cobol" => Ok(Case::Cobol),
        "train" | "Train" => Ok(Case::Train),
        "flat" => Ok(Case::Flat),
        "upper_flat" | "UPPER_FLAT" => Ok(Case::UpperFlat),
        "alternating" | "aLtErNaTiNg" => Ok(Case::Alternating),
        _ => Err(()),
    }
}
