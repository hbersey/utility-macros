use utility_macros_internals::utils::ResultExt as _;

#[test]
pub fn test_or_panic() {
    let ok: Result<(), ()> = Ok(());
    assert_eq!(ok.or_panic(), ());
}

#[test]
#[should_panic(expected = "This should panic")]
pub fn test_or_panic_panic() {
    let err: Result<(), &'static str> = Err("This should panic");
    err.or_panic();
}
