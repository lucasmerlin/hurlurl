use super::schema::links;
use super::schema::targets;

pub use shared::*;

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink<'a> {
    pub url: &'a str,
    pub permanent_redirect: bool,
    pub created_by_ip: Option<ipnet::IpNet>,
}

#[derive(Insertable)]
#[table_name = "targets"]
pub struct NewTarget<'a> {
    pub link_id: i32,
    pub target_url: &'a str,
}
