// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        enabled -> Bool,
    }
}
