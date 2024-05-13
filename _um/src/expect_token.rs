macro_rules! expect_token {
    ($tokens:ident, ident) => {
        match $tokens.next() {
            Some(TokenTree::Ident(ident)) => ident,
            _ => panic!("expected identifier"),
        }
    };
    ($tokens:ident, ident = $expected:literal) => {
        match $tokens.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == $expected => ident,
            _ => panic!("expected `{}`", $expected),
        }
    };
    ($tokens:ident, punct) => {
        match $tokens.next() {
            Some(TokenTree::Punct(punct)) => punct,
            _ => panic!("expected punctuation"),
        }
    };
    ($tokens:ident, punct = $expected:literal) => {
        match $tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == $expected => {}
            _ => panic!("expected `{}`", $expected),
        }
    };
}

pub(crate) use expect_token;
