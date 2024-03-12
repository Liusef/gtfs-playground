use super::super::utils;


fn write_out(basedir: &str, pdir: &str, data: &impl serde::Serialize, name: &str) {
    utils::write_to_file(&format!("{basedir}/{name}.json"), &serde_json::to_string(data).unwrap());
    utils::write_to_file(&format!("{pdir}/{name}.p.json"), &serde_json::to_string_pretty(data).unwrap());
}

pub async fn ingest(uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ingested = gtfs_structures::Gtfs::from_url_async(uri).await.unwrap();

    let basedir = "./export/gtfs_static";
    let pdir = "./export/gtfs_static/pretty"; // TODO See if there's a way to do compile time formating

    utils::make_dir(basedir);
    utils::make_dir(pdir);

    write_out(&basedir, &pdir, &ingested.read_duration, "read_duration");
    write_out(&basedir, &pdir, &ingested.calendar, "calendar");
    write_out(&basedir, &pdir, &ingested.calendar_dates, "calendar_dates");
    write_out(&basedir, &pdir, &ingested.stops, "stops");
    write_out(&basedir, &pdir, &ingested.routes, "routes");
    write_out(&basedir, &pdir, &ingested.trips, "trips");
    write_out(&basedir, &pdir, &ingested.agencies, "agencies");
    write_out(&basedir, &pdir, &ingested.shapes, "shapes");
    write_out(&basedir, &pdir, &ingested.fare_attributes, "fare_attributes");
    write_out(&basedir, &pdir, &ingested.fare_rules, "fare_rules");
    write_out(&basedir, &pdir, &ingested.feed_info, "feed_info");
    
    return Ok(());
}