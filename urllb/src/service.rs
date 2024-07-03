use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::Result;
use diesel::associations::HasTable;
use diesel::expression_methods::ExpressionMethods;
use diesel::internal::operators_macro::FieldAliasMapper;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use ipnet::IpNet;

use shared::{schema, PaymentStatus};

use crate::db::Connection;
use crate::models::{CreateLinkDto, Link, NewLink, NewTarget, Target};
use crate::schema::links::dsl::*;
use crate::schema::links::url;
use crate::schema::targets::dsl::targets;
use crate::schema::targets::link_id;

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

pub async fn set_link_payment_status<'c>(
    connection: &mut Connection<'c>,
    link: &str,
    status: PaymentStatus,
) -> Result<()> {
    diesel::update(links.filter(url.eq(link)))
        .set(schema::links::payment_status.eq(status))
        .execute(connection)
        .await?;

    Ok(())
}

pub async fn create_link<'c>(
    connection: &mut Connection<'c>,
    create: &CreateLinkDto,
    path: &str,
    user_ip: IpNet,
    stripe_session: Option<String>,
) -> Result<(Link, Vec<Target>)> {
    let link = NewLink {
        url: path,
        permanent_redirect: create.permanent_redirect,
        created_by_ip: Some(anonymize_ip(user_ip)),
        payment_status: stripe_session.as_ref().map(|_| PaymentStatus::Pending),
        stripe_session_id: stripe_session.as_deref(),
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

/// Truncates some bits of the IP address to anonymize it.
pub fn anonymize_ip(ip: IpNet) -> IpNet {
    match ip {
        IpNet::V4(v4) => {
            let octets = v4.addr().octets();
            let new_ip =
                ipnet::Ipv4Net::new(Ipv4Addr::new(octets[0], octets[1], octets[2], 0), 24).unwrap();
            IpNet::V4(new_ip)
        }
        IpNet::V6(v6) => {
            let octets = v6.addr().segments();
            let new_ip = ipnet::Ipv6Net::new(
                Ipv6Addr::new(octets[0], octets[1], octets[2], octets[3], 0, 0, 0, 0),
                120,
            )
            .unwrap();
            IpNet::V6(new_ip)
        }
    }
}
