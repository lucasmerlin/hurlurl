-- Your SQL goes here

create table links
(
    id serial primary key,
    url VARCHAR unique not null,
    redirects integer not null default 0,

    permanent_redirect boolean not null default false
);

create table targets
(
    id serial primary key,
    link_id integer references links(id) not null,
    target_url VARCHAR not null,
    redirects integer not null default 0
);
