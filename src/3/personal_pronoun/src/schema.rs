table! {
    FirstPersons (id) {
        id -> Integer,
        value -> Text,
        ruby -> Text,
        comment -> Text,
    }
}

table! {
    SecondPersons (id) {
        id -> Integer,
        value -> Text,
        ruby -> Text,
        comment -> Text,
    }
}

table! {
    ThirdPersons (id) {
        id -> Integer,
        value -> Text,
        ruby -> Text,
        comment -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    FirstPersons,
    SecondPersons,
    ThirdPersons,
);
