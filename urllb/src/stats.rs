use cached::lazy_static::lazy_static;
use diesel::dsl::sum;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;

use cached::{Cached, TimedCache};
use shared::TotalStats;

use crate::db::Connection;
use crate::schema::links::dsl::*;
use crate::schema::targets::dsl::targets;

use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<TimedCache<usize, TotalStats>> =
        Mutex::new(TimedCache::with_lifespan(120));
}

pub async fn total_stats<'a>(connection: &mut Connection<'a>) -> anyhow::Result<TotalStats> {
    if let Some(stats) = CACHE.lock().unwrap().cache_get(&0) {
        return Ok(stats.clone());
    }

    let link_count = links.count().first(connection).await?;

    let target_count = targets.count().first(connection).await?;

    let redirect_count: Option<i64> = links.select(sum(redirects)).first(connection).await?;

    let stats = TotalStats {
        links: link_count,
        targets: target_count,
        redirects: redirect_count.unwrap_or(0),
    };

    CACHE.lock().unwrap().cache_set(0, stats.clone());

    Ok(stats)
}
