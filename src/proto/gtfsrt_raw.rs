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

pub async fn tripupdate() -> Result<(), Box<dyn std::error::Error>> {
    let wbuf = reqwest::get("https://api.bart.gov/gtfsrt/tripupdate.aspx").await?.bytes().await?;
    let buf = transit_realtime::FeedMessage::decode(wbuf).expect("Why are you brokie, protobuf brokie");
    let json = serde_json::to_string(&buf).expect("why are you brokie, serialization brokie");
    let jsonp = serde_json::to_string_pretty(&buf).expect("pretty json brokie, why are you brokie");
    println!("{jsonp}");
    write_to_file("./export/tripupdate.json", &json);
    write_to_file("./export/tripupdate.p.json", &jsonp);
    return Ok(());
}

pub async fn alerts() -> Result<(), Box<dyn std::error::Error>> {
    let wbuf = reqwest::get("https://api.bart.gov/gtfsrt/alerts.aspx").await?.bytes().await?;
    let buf = transit_realtime::FeedMessage::decode(wbuf).expect("Why are you brokie, protobuf brokie");
    let json = serde_json::to_string(&buf).expect("why are you brokie, serialization brokie");
    let jsonp = serde_json::to_string_pretty(&buf).expect("pretty json brokie, why are you brokie");
    // println!("{json}");
    write_to_file("./stuff/alerts.json", &json);
    write_to_file("./stuff/alerts.p.json", &jsonp);
    return Ok(());
}