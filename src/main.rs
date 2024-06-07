mod utils;
mod proto;
mod gtfs_static;
mod sfbart_metrics;

use gtfs_static::ingest::ingest;
use proto::gtfsrt_raw;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("\n\n\x1b[34mCWD should be project root \x1b[0m-\x1b[32m Your CWD is \x1b[033m{}\x1b[0m", _cwd());
    // gtfsrt_raw::_build_proto();
    // proto::gtfsrt_raw::tripupdate_json().await?;
    // rt_protobuf::alerts().await?;

    // proto::gtfsrt_raw::alerts_json().await?;

    // ingest("http://www.bart.gov/dev/schedules/google_transit.zip").await?;

    sfbart_metrics::list_curr_trips().await?;
    loop {
        sfbart_metrics::track_trip("1509120").await?;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    // sfbart_metrics::legacy_get_arr("BERY").await?;

    Ok(())    
}


