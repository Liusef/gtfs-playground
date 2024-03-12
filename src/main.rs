mod utils;
mod proto;
mod gtfs_static;

use gtfs_static::ingest::ingest;
use proto::gtfsrt_raw;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("\n\n\x1b[34mCWD should be project root \x1b[0m-\x1b[32m Your CWD is \x1b[033m{}\x1b[0m", _cwd());
    // gtfsrt_raw::_build_proto();
    proto::gtfsrt_raw::tripupdate().await?;
    // rt_protobuf::alerts().await?;

    // ingest("http://www.bart.gov/dev/schedules/google_transit.zip").await?;

    Ok(())    
}


