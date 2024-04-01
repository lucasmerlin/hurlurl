-- Your SQL goes here

alter table links
    add column fraud boolean not null default false;
alter table links
    add column fraud_reason text;

--- we collect the (anonymized, truncated) IP address of the creator, so we can easier track down fraudsters
alter table links
    add column created_by_ip inet;
