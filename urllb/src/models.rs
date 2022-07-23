use crate::{Deserialize, Serialize};
use super::schema::links;
use super::schema::targets;

#[derive(Queryable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub redirects: i32,

    pub permanent_redirect: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub id: i32,
    pub link_id: i32,
    pub target_url: String,
    pub redirects: i32,
}

#[derive(Insertable)]
#[table_name="links"]
pub struct NewLink<'a> {
    pub url: &'a str,
    pub permanent_redirect: bool,
}

#[derive(Insertable)]
#[table_name="targets"]
pub struct NewTarget<'a> {
    pub link_id: i32,
    pub target_url: &'a str,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLinkDto {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub permanent_redirect: bool,
    pub targets: Vec<String>,
}

#[derive(Serialize)]
pub struct LinkDto {
    #[serde(flatten)]
    pub link: Link,

    pub targets: Vec<Target>,
}
