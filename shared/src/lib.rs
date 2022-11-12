#[cfg(feature = "diesel")]
pub mod schema;

#[cfg(feature = "diesel")]
use diesel::{Queryable, Identifiable};
#[cfg(feature = "diesel")]
use crate::schema::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[cfg_attr(feature = "diesel", derive(Queryable, Identifiable))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub redirects: i32,

    pub permanent_redirect: bool,
}

#[cfg_attr(feature = "diesel", derive(Queryable, Identifiable))]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub id: i32,
    pub link_id: i32,
    pub target_url: String,
    pub redirects: i32,
}

#[derive(Serialize, Deserialize, Validate, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTargetDto {
    #[validate(url)]
    pub target_url: String,
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateLinkDto {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub permanent_redirect: bool,
    #[validate(length(min = 1))]
    #[validate]
    pub targets: Vec<CreateTargetDto>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkDto {
    #[serde(flatten)]
    pub link: Link,

    pub targets: Vec<Target>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TotalStats {
    pub links: i64,
    pub redirects: i64,
    pub targets: i64,
}
