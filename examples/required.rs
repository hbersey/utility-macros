use utility_macros::{HasRequired as _, Required};

fn main() {
    let details = Details {
        id: 0,
        name: Some("Henry".to_string()),
        age: Some(100),
        password: None,
    };

    let all = details.required().unwrap();
    let also_all: AllDetails = details.clone().try_into().unwrap();
    let also_details = also_all.type_();

    assert_eq!(details, all);
    assert_eq!(all, also_all);
    assert_eq!(details, also_details);
}

#[derive(Required, Debug, Clone, PartialEq)]
#[required(name = AllDetails, derive(PartialEq, Debug))]
pub struct Details {
    #[required(name = details_id)]
    pub id: u32,
    pub name: Option<String>,
    pub age: Option<u8>,
    #[required(skip)]
    pub password: Option<String>,
}
