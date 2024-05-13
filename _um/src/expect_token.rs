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
    ($tokens:ident, group) => {
        match $tokens.next() {
            Some(TokenTree::Group(group)) => group,
            _ => panic!("expected group"),
        }
    };
    ($tokens:ident, group, delimiter = $expected:path) => {
        match $tokens.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == $expected => group,
            _ => panic!("expected group with delimiter `{:?}`", $expected),
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

macro_rules! peek_token {
    ($tokens:ident, punct = $what:literal) => {
        match $tokens.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == $what => Some(punct),
            _ => None,
        }
    };
}

pub(crate) use {expect_token, peek_token};
