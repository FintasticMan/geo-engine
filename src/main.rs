use std::net::Ipv6Addr;

use anyhow::{bail, Result};
use pnet::{datalink, ipnetwork::IpNetwork};

fn ipv6_is_eligible(ip: Ipv6Addr) -> bool {
    !ip.is_unspecified()
        && !ip.is_loopback()
        && !ip.is_unique_local()
        && !ip.is_unicast_link_local()
}

fn main() -> Result<()> {
    let ips: Vec<IpNetwork> = datalink::interfaces()
        .into_iter()
        .filter(|i| i.is_up() && !i.is_loopback() && !i.ips.is_empty())
        .flat_map(|i| {
            i.ips.into_iter().filter(|ip| match ip {
                IpNetwork::V6(ip) if ipv6_is_eligible(ip.ip()) => true,
                _ => false,
            })
        })
        .collect();

    if ips.is_empty() {
        bail!("no IP addresses");
    }

    for ip in ips.into_iter() {
        println!("{ip} 1;");
    }

    Ok(())
}
