// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        url -> Varchar,
        redirects -> Int4,
        permanent_redirect -> Bool,
    }
}

diesel::table! {
    targets (id) {
        id -> Int4,
        link_id -> Int4,
        target_url -> Varchar,
        redirects -> Int4,
    }
}

diesel::joinable!(targets -> links (link_id));

diesel::allow_tables_to_appear_in_same_query!(links, targets,);
