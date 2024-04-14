// cargo run --release --example tmlu2json
// cargo install --path . --example tmlu2json

use std::{env, io::BufReader};

use serde::Serialize;

type Id = i32;

#[derive(Serialize)]
struct Station {
    id: Id,
    length: f64,
    azimuth: f64,
    depth: f64,
    date: String,
    from_id: Id,
}

#[derive(Serialize)]
struct StartStation {
    id: Id,
    latitude: f64,
    longitude: f64,
    depth: f64,
}

#[derive(Serialize)]
struct Closure {
    id: Id,
    from_id: Id,
    to_id: Id,
    //from_ids : []
}

#[derive(Serialize)]
enum Survey {
    #[serde(rename = "start")]
    Start(StartStation),
    #[serde(rename = "relative")]
    Station(Station),
    #[serde(rename = "loop")]
    Closure(Closure),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: tmlu2json <tmlu-file>");
        std::process::exit(1);
    }
    let file = &args[1];
    let reader = BufReader::new(std::fs::File::open(file).unwrap());
    let cave = tmlu_rs::tmlu::read_cavefile(reader);
    let json: Vec<Survey> = cave
        .data
        .iter()
        .map(|data| match data.station_type.as_str() {
            "START" => Survey::Start(StartStation {
                id: data.id,
                latitude: data.latitude.parse::<f64>().unwrap(),
                longitude: data.longitude.parse::<f64>().unwrap(),
                depth: data.depth.parse::<f64>().unwrap(),
            }),
            "CLOSURE" => Survey::Closure(Closure {
                id: data.id,
                from_id: data.from_id,
                to_id: data.closure_to_id,
            }),
            "REAL" | "VIRTUAL" => Survey::Station(Station {
                id: data.id,
                azimuth: data.azimuth.parse::<f64>().unwrap(),
                length: data.length.parse::<f64>().unwrap(),
                depth: data.depth.parse::<f64>().unwrap(),
                from_id: data.from_id,
                date: data.date.clone(),
            }),
            _ => panic!("Unknown station type: {}", data.station_type),
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
