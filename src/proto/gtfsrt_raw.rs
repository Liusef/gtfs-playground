use prost::Message;

mod transit_realtime {
    include!("transit_realtime.rs");
}

use crate::utils::*;

pub fn _build_proto() -> () {
    let mut cfg = prost_build::Config::new();
    cfg.type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .out_dir("./src/")
        .compile_well_known_types()
        .compile_protos(&["./src/proto/gtfs-rt.proto"], &["./src/"])
        .expect("Compile Proto brokie, why brokie :(");
}

pub async fn tripupdate() -> Result<transit_realtime::FeedMessage, Box<dyn std::error::Error>> {
    let wbuf = reqwest::get("https://api.bart.gov/gtfsrt/tripupdate.aspx").await?.bytes().await?;
    return Ok(transit_realtime::FeedMessage::decode(wbuf)?);
}

pub async fn tripupdate_json() -> Result<(), Box<dyn std::error::Error>> {
    let buf = tripupdate().await?;
    let json = serde_json::to_string(&buf).expect("why are you brokie, serialization brokie");
    let jsonp = serde_json::to_string_pretty(&buf).expect("pretty json brokie, why are you brokie");
    println!("{jsonp}");
    write_to_file("./export/tripupdate.json", &json);
    write_to_file("./export/tripupdate.p.json", &jsonp);
    return Ok(());
}

pub async fn alerts() -> Result<transit_realtime::FeedMessage, Box<dyn std::error::Error>> {
    let wbuf = reqwest::get("https://api.bart.gov/gtfsrt/alerts.aspx").await?.bytes().await?;
    return Ok(transit_realtime::FeedMessage::decode(wbuf)?);
}

pub async fn alerts_json() -> Result<(), Box<dyn std::error::Error>> {
    let buf = alerts().await?;
    let json = serde_json::to_string(&buf).expect("why are you brokie, serialization brokie");
    let jsonp = serde_json::to_string_pretty(&buf).expect("pretty json brokie, why are you brokie");
    // println!("{json}");
    write_to_file("./export/alerts.json", &json);
    write_to_file("./export/alerts.p.json", &jsonp);
    return Ok(());
}
