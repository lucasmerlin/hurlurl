-- Your SQL goes here

create type payment_status as enum ('pending', 'succeeded', 'failed');

alter table links
    add column stripe_session_id text;

alter table links
    add column payment_status payment_status;