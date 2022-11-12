use diesel::QueryDsl;
use diesel::dsl::sum;
use diesel_async::RunQueryDsl;

use cached::proc_macro::cached;
use shared::TotalStats;

use crate::db::db;
use crate::schema::links::dsl::*;
use crate::schema::targets::dsl::targets;

#[cached(time=60, result = true)]
pub async fn total_stats() -> anyhow::Result<TotalStats> {
    let mut db = db().await;

    let link_count = links.count().first(&mut db).await?;

    let target_count = targets.count().first(&mut db).await?;

    let redirect_count: Option<i64> = links.select(sum(redirects)).first(&mut db).await?;

    let stats = TotalStats {
        links: link_count,
        targets: target_count,
        redirects: redirect_count.unwrap_or(0),
    };

    Ok(stats)
}
