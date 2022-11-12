use diesel::associations::HasTable;
use diesel::expression_methods::ExpressionMethods;
use diesel::QueryDsl;

use diesel_async::RunQueryDsl;

use nanoid::nanoid;

use crate::db::Connection;
use crate::models::{CreateLinkDto, Link, NewLink, NewTarget, Target};
use crate::schema::links::dsl::*;
use crate::schema::links::url;
use crate::schema::targets::dsl::targets;
use crate::schema::targets::link_id;

use anyhow::Result;

use shared::schema;

pub async fn get_link_and_targets<'c>(
    connection: &mut Connection<'c>,
    link: &str,
) -> Result<(Link, Vec<Target>)> {
    let link = links.filter(url.eq(link)).first::<Link>(connection).await?;

    let target_results: Vec<Target> = targets
        .filter(link_id.eq(link.id))
        .load::<Target>(connection)
        .await?;

    Ok((link, target_results))
}

pub async fn increase_redirect_count<'c>(
    connection: &mut Connection<'c>,
    link: &Link,
    target: &Target,
) -> Result<()> {
    diesel::update(link)
        .set(schema::links::redirects.eq(schema::links::redirects + 1))
        .execute(connection)
        .await?;

    diesel::update(target)
        .set(schema::targets::redirects.eq(schema::targets::redirects + 1))
        .execute(connection)
        .await?;

    Ok(())
}

pub async fn create_link<'c>(
    connection: &mut Connection<'c>,
    create: &CreateLinkDto,
) -> Result<(Link, Vec<Target>)> {
    let link = NewLink {
        // TODO: Add some way to add custom url
        url: &nanoid!(5),
        permanent_redirect: create.permanent_redirect,
    };

    let link = diesel::insert_into(links::table())
        .values(link)
        .get_result::<Link>(connection)
        .await?;

    let target_results = diesel::insert_into(targets::table())
        .values(
            &create
                .targets
                .iter()
                .map(|target| NewTarget {
                    link_id: link.id,
                    target_url: &target.target_url,
                })
                .collect::<Vec<_>>(),
        )
        .get_results::<Target>(connection)
        .await?;

    Ok((link, target_results))
}
