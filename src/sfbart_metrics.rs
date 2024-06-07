use std::str::FromStr;

use crate::proto::gtfsrt_raw::tripupdate;

static SFBARTD_LEGACY_APIETD: &str = "https://api.bart.gov/api/etd.aspx";
static SFBARTD_LEGACY_PUBKEY: &str = "MW9S-E7SL-26DU-VV8V";

/*

NOTES: 
- 2000ms b/w pings to RT endpoint
- Used system UNIX time, result from API is unreliable

- assume following metrics for example (generalize later)

- arr-20s, arrival announcement
    - Arriving at, <STOP NAME>
- 0s, boarding announcement
    - Now boarding at, <STOP NAME>
    - This is a <COLOR> line train to, <DEST>
- dept-5s, closing announcement
    - The doors are closing, please stand clear of the doors, <3 reps>
- station dropping off, next station announcement
    - The next station is, <NEXT STOP NAME>


STATE MACHINE
EOL - Dwelling at terminal station, TODO link trips
Moving - Between stations
Arriving - 20s to arrival at next station
Boarding - 0s to T-5
Leaving - T-5..., doors closing
Moving(transitional state) - Next station announcement

EDGE CASES
1. Some stations are so close together that they can disappear from the trip update early
    - RELY ON STATE MACHINE, some states are required some are optional, just keep going through the states until you're up to date
    - tbh the only one you strictly need is doors closing
2. Trips can be shown as having already arrived at a station but then get delayed and not be there yet anymore?
    - ie. train to SFIA, next stop is embarcadero
        - Arrival time is listed as 5 seconds ago
        - Next ping, it now says it will arrive in 15 seconds
    - STATE MACHINE WOO
    - If we already played the boarding announcement, but we're not actually there yet, just keep track in state machine
    - impl?
        - Keep track of previous announcement made (prev stn and curr state is all you need for that)


*/


fn wrap_sub(a: i64, b: u64) -> i64 {
    return a - b as i64;
}

#[derive(std::fmt::Debug)]
struct TripInfo{
    timestamp: u64,
    id: String,
    dest: String,
    next: String,
    nexttime: i64,
    following: String,
    ftime: i64
}


impl std::fmt::Display for TripInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ftstr = if self.ftime != 0 {format!("{}", self.ftime)} else {String::from_str("").unwrap()};
        let ftstr_diff = if self.ftime != 0 {format!("{}", wrap_sub(self.ftime, self.timestamp))} else {String::from_str("N/A").unwrap()};
        write!(f, "\x1b[33mid: {}\x1b[0m to\x1b[36m {}\n\x1b[32mNext Station: \x1b[0m{} in {} ({}) \n\x1b[34mFollowing Station: \x1b[0m{} in {} ({})\n",
            self.id, self.dest, self.next, wrap_sub(self.nexttime, self.timestamp), self.nexttime, self.following, ftstr_diff, ftstr)
    }
}

pub async fn list_curr_trips() -> Result<(), Box<dyn std::error::Error>> {
    println!();


    let buf = tripupdate().await?;

    let timestamp = buf.header.timestamp();

    let mut tis: Vec<TripInfo> = Vec::new();

    for trip in &buf.entity {
        let stops = &trip.trip_update.as_ref().unwrap().stop_time_update;
        let nti = TripInfo {
            timestamp: timestamp,
            id: trip.id.clone(),
            dest: String::from_str(stops[stops.len() - 1].stop_id()).unwrap(),
            next: String::from_str(stops[0].stop_id()).unwrap(),
            nexttime: (&stops[0]).arrival.as_ref().unwrap().time.unwrap(),
            following: if stops.len() > 1 {String::from_str(stops[1].stop_id()).unwrap()} else {String::from_str("N/A").unwrap()},
            ftime: if stops.len() > 1 {(&stops[1]).arrival.as_ref().unwrap().time.unwrap()} else {0}
        };
        tis.push(nti);
    }
    
    println!("\x1b[34mThe current time is:\x1b[0m {}\n", timestamp);

    for ti in tis {
        println!("{ti}");
    }


    Ok(())
}

pub async fn track_trip(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let buf = tripupdate().await?;
    // let timestamp = buf.entity;

    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    let mut found = false;
    for trip in &buf.entity {
        if &trip.id != id {continue;}

        let stops = &trip.trip_update.as_ref().unwrap().stop_time_update;

        println!("\x1b[32mTrip found!\x1b[34m Current Time: {}\x1b[0m\n", timestamp);
        println!("\x1b[33mTrip {} to \x1b[36m{}\x1b[0m", id, (&stops[stops.len() - 1]).stop_id.as_ref().unwrap());

        println!("\n\x1b[34mNext {} stops:\x1b[0m", std::cmp::min(5, stops.len()));
        
        let mut i = 0;
        for stop in stops {
            if i == 5 {break;}
            i += 1;
            
            
            println!("    \x1b[32m{} - \x1b[36m {} \x1b[90m| Dwell: {}, Delay: {}", i, stop.stop_id(), stop.departure.as_ref().unwrap().time() - stop.arrival.as_ref().unwrap().time(), stop.departure.as_ref().unwrap().delay());
            println!("        \x1b[34mArrive in: \x1b[0m{} ± {} \x1b[90m({})", wrap_sub(stop.arrival.as_ref().unwrap().time(), timestamp.clone()), stop.arrival.as_ref().unwrap().uncertainty(), stop.arrival.as_ref().unwrap().time());
            println!("        \x1b[34mDepart in: \x1b[0m{} ± {} \x1b[90m({})", wrap_sub(stop.departure.as_ref().unwrap().time(), timestamp.clone()), stop.departure.as_ref().unwrap().uncertainty(), stop.departure.as_ref().unwrap().time());
            println!();

        }
        
        found = true;
        break;
    }

    if !found {
        println!("\x1b[31mCRITICAL:\x1b[0m Your trip was not found.");
    }


    Ok(())
}

pub async fn legacy_get_arr(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let params = [("cmd","etd"), ("orig", id), ("key", self::SFBARTD_LEGACY_PUBKEY), ("json", "y")];

    let res = client.get(self::SFBARTD_LEGACY_APIETD).query(&params).send().await?.text().await?;

    println!("{}", res);

    return Ok(());
}