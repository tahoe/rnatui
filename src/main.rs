//! Example rust app using the NetActuate API rust library
//!
//! This app is mainly for testing and is just an example
//!
//! # Usage
//! ### Set ENVs
//!
//! ```bash
//! export API_KEY='<your api key>'
//! export API_ADDRESS='https://vapi2.netactuate.com/api/cloud'
//! ```
//!
//! ### Install example client
//! ```rust
//! cargo install rnaapi
//! ```
//!
//! ### There are two forms of output, all server info or a single server's info
//!
//! #### All servers info
//! `rnaapi`
//!
//! ## A single servers info
//! `rnaapi -m <mbpkgid>`
//!
//! That's it.
//!
// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use anyhow::Result;
use clap::Parser;
use rnaapi::config::Settings;
use rnaapi::NaClient;

#[tokio::main]
async fn main() -> Result<()> {
    //! Test/Example "main" function, right now it just takes
    //! one argument, `-m <mbpkgid>` if not given, returns all the servers you own

    // Get settings from config
    let settings = Settings::new()?;

    // Defaults
    let mut mbpkgid: u32 = 0;
    let mut zoneid: u32 = 0;

    // parse our args into args
    let args = SimpleArgs::parse();

    if args.mbpkgid >= 1 {
        mbpkgid = args.mbpkgid;
    }

    if args.zoneid >= 1 {
        zoneid = args.zoneid;
    }

    // playing with new constructor for client
    // let na_client = NaClient::new(API_KEY.to_owned(), API_ADDRESS.to_owned()).await;
    let na_client = NaClient::new(settings.api_key, settings.api_url).await;

    if mbpkgid > 0 {
        // submit jobs to the tokio async runtime
        // this automatically awaits so no need for .await
        let (srv, jobs, ipv4s, ipv6s, stat) = tokio::join!(
            na_client.get_server(mbpkgid),
            na_client.get_jobs(mbpkgid),
            na_client.get_ipv4(mbpkgid),
            na_client.get_ipv6(mbpkgid),
            na_client.get_status(mbpkgid),
        );

        // print basic server info
        println!(
            "Package: {}, fqdn: {}, mbpkgid: {}",
            srv.clone().unwrap().domu_package,
            srv.clone().unwrap().fqdn,
            srv.clone().unwrap().mbpkgid
        );

        println!();
        // print the job data
        for job in jobs.unwrap() {
            println!(
                "Inserted: {}, Status: {}, command: {}",
                job.ts_insert, job.status, job.command
            );
        }

        println!();
        // print IPv4 Addresses
        for ipv4 in ipv4s.unwrap() {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv4.reverse, ipv4.ip, ipv4.gateway
            );
        }

        println!();
        // print IPv6 Addresses
        for ipv6 in ipv6s.unwrap() {
            println!(
                "Reverse: {}, IP: {}, Gateway: {}",
                ipv6.reverse, ipv6.ip, ipv6.gateway
            );
        }

        println!();
        // print server status, very unverbose
        println!("Status: {}", stat.unwrap().status);
    } else if zoneid > 0 {
        println!();
        // // print out the zone name
        let zone = na_client.get_zone(zoneid).await?;
        println!("Zone: {}", zone.name);

        // print out the SOA for the zone
        let soa = zone.soa.unwrap();
        println!("SOA: {}", soa.primary);

        // print out the first record
        let recs = zone.records.unwrap();
        println!("1st Record: {}", recs[0].name);

        // print out the first NS record
        let nsrecs = zone.ns.unwrap();
        println!("1st NS: {}", nsrecs[0])
    } else {
        // submit jobs to the tokio async runtime
        // this automatically awaits so no need for .await
        let (srvrs, locs, pkgs, imgs, zones, ssh_keys, deets, invoices) = tokio::join!(
            na_client.get_servers(),
            na_client.get_locations(),
            na_client.get_packages(),
            na_client.get_images(),
            na_client.get_zones(),
            na_client.get_ssh_keys(),
            na_client.get_acct_details(),
            na_client.get_acct_invoices()
        );

        for srvr in srvrs.unwrap() {
            println!("fqdn: {}, mbpkgid: {}", srvr.fqdn, srvr.mbpkgid);
        }

        println!();
        // list locations
        for loc in locs.unwrap() {
            println!("Name: {}, Continent: {}", loc.name, loc.continent);
        }

        println!();
        // list packages
        for pkg in pkgs.unwrap() {
            println!("Name: {}, Continent: {}", pkg.name, pkg.city);
        }

        println!();
        // list images
        for img in imgs.unwrap() {
            println!(
                "ID: {}, Size: {}, Name: {}",
                img.id,
                img.size.unwrap_or("null".to_owned()),
                img.os.unwrap_or("null".to_owned())
            );
        }

        println!();
        // list dns zones
        for zone in zones.unwrap() {
            println!(
                "ID: {}, Size: {}, Name: {}",
                zone.id, zone.name, zone.zone_type
            );
        }

        println!();
        // print some ssh keys
        for sshkey in ssh_keys.unwrap() {
            println!("Key: {}, Fingerprint: {}", sshkey.name, sshkey.fingerprint);
        }

        println!();
        // print some account deets
        println!(
            "FullName: {:?}, Address: {:?}, {:?} {:?} {:?}",
            deets.clone().unwrap().fullname,
            deets.clone().unwrap().address1,
            deets.clone().unwrap().city,
            deets.clone().unwrap().state,
            deets.clone().unwrap().postcode
        );

        println!();
        // print some of the invoices, say 3?
        for invoice in invoices.unwrap().iter().take(3) {
            println!("ID: {}, Status: {}", invoice.id, invoice.status);
        }
    }

    Ok(())
}

///
/// This is the SimpleArgs struct
///
#[derive(Parser, Debug)]
#[command(version, about)]
struct SimpleArgs {
    // -m argument for picking an mbpkgid
    #[arg(short, long, default_value_t = 0)]
    mbpkgid: u32,

    // -z argument for picking a dns zone
    #[arg(short, long, conflicts_with = "mbpkgid", default_value_t = 0)]
    zoneid: u32,
}
