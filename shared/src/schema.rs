// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_status"))]
    pub struct PaymentStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentStatus;

    links (id) {
        id -> Int4,
        url -> Varchar,
        redirects -> Int4,
        permanent_redirect -> Bool,
        fraud -> Bool,
        fraud_reason -> Nullable<Text>,
        created_by_ip -> Nullable<Inet>,
        stripe_session_id -> Nullable<Text>,
        payment_status -> Nullable<PaymentStatus>,
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
