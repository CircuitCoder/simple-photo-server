mod store;

use clap::Parser;
use std::{net::IpAddr, path::PathBuf};
use log::debug;
use serde::Deserialize;
use rocket::tokio::sync::RwLock;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value="0.0.0.0")]
    host: IpAddr,

    #[arg(short, long, default_value="14429")]
    port: u16,

    #[arg(short, long)]
    listings_file: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Sorting {
    FilenameAsc,
    DateDsc,
    DateAsc,
}

#[derive(Deserialize, Debug)]
pub struct Listing {
    name: String,
    root: PathBuf,
    default_order: Sorting,
}

#[derive(Deserialize, Debug)]
struct Listings {
    listings: Vec<Listing>
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    Ok(())
}
