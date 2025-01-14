use utility_macros::partial;

#[derive(Debug, PartialEq)]
#[partial(PartialUser, derive(Debug))]
pub struct User<T> {
    pub name: String,
    pub email: Option<String>,
    pub data: T,
    pub maybe_data: Option<T>,
}

// pub struct PartialUser<T> {
//     pub name: Option<String>,
//     pub email: Option<String>,
//     pub data: Option<T>,
//     pub maybe_data: Option<iT>,
// }

fn main() {
    let user = User {
        name: "Alice".to_string(),
        email: Some("alice@gmail.com".to_string()),
        data: 42,
        maybe_data: Some(42),
    };

    let partial_user = PartialUser {
        name: Some("Alice".to_string()),
        email: Some("alice@gmail.com".to_string()),
        data: Some(42),
        maybe_data: Some(42),
    };

    assert_eq!(partial_user, user);
}
