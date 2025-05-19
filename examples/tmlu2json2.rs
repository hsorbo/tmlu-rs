// cargo run --release --example tmlu2json
// cargo install --path . --example tmlu2json

use std::{collections::HashMap, env, io::BufReader};

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
    stations : Vec<Station>,
}

#[derive(Serialize)]
struct Closure {
    id: Id,
    from_id: Id,
    to_id: Id,
}

#[derive(Serialize)]
struct SurveyData {
    shots: Vec<StartStation>,
    closures: Vec<Closure>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: tmlu2json2 <tmlu-file>");
        std::process::exit(1);
    }
    let file = &args[1];
    let reader = BufReader::new(std::fs::File::open(file).unwrap());
    let cave = tmlu_rs::tmlu::read_cavefile(reader);

    let mut survey_data = SurveyData {
        shots: Vec::new(),
        closures: Vec::new()
    };

    let mut stations : HashMap<i32, Station> = HashMap::new();
    let mut child_parent : HashMap<i32, i32> = HashMap::new();

    for data in cave.data.iter() {
        if data.station_type.as_str() == "START" {
            survey_data.shots.push(StartStation {
                id: data.id,
                latitude: data.latitude.parse::<f64>().unwrap(),
                longitude: data.longitude.parse::<f64>().unwrap(),
                depth: data.depth.parse::<f64>().unwrap(),
                stations: Vec::new()
            });
        }
        if data.station_type.as_str() == "CLOSURE" {
            survey_data.closures.push(Closure {
                id: data.id,
                from_id: data.from_id,
                to_id: data.closure_to_id,
            });
        }
        if data.station_type.as_str() == "REAL" || data.station_type.as_str() == "VIRTUAL" {
            child_parent.insert(data.id, data.from_id);
            stations.insert(data.id, Station {
                id: data.id,
                azimuth: data.azimuth.parse::<f64>().unwrap(),
                length: data.length.parse::<f64>().unwrap(),
                depth: data.depth.parse::<f64>().unwrap(),
                from_id: data.from_id,
                date: data.date.clone(),
            });
           
        }
    }

    let mut parent_children : HashMap<i32, Vec<i32>> = HashMap::new();
    for (child, parent) in child_parent.iter() {
        parent_children.entry(*parent).or_default().push(*child);
        // if !parent_children.contains_key(child) {
        //     parent_children.insert(*child, Vec::new());
        // }
    }
    //dbg!(parent_children);

    for x in survey_data.shots.iter_mut() {
        let mut current_id = vec![x.id];
        while !current_id.is_empty() {
            let mut next_id = Vec::new();
            for id in current_id.iter() {
                if parent_children.contains_key(id) {
                    for child in parent_children.get(id).unwrap() {
                        next_id.push(*child);
                        x.stations.push(stations.remove(child).unwrap());
                    }
                }
            }
            current_id = next_id;
        }
    }

    println!("{}", serde_json::to_string_pretty(&survey_data).unwrap());
}
