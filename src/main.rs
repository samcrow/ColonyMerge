
extern crate rustc_serialize;
extern crate csv;
extern crate clap;
use clap::App;
use clap::Arg;
use csv::Writer;
use rustc_serialize::json::Json;
use std::fs::File;
use std::cmp::Ordering::*;
mod colony;
use colony::Colony;

type ColonySet = std::collections::HashMap<u32, Colony>;

fn main() {
    let matches = App::new("colony_merge")
        .author("Sam Crow")
        .version("0.1.0")
        .about("Converts colony data from JSON into CSV")
        .arg(Arg::with_name("input").help("An input JSON file to read").short("i").long("input").takes_value(true).required(true).min_values(1))
        .arg(Arg::with_name("output").short("o").long("output").takes_value(true).help("An output CSV file to write").required(true))
        .get_matches();

    let json_paths = matches.values_of("input").unwrap();
    let csv_path = matches.value_of("output").unwrap();

    // Read and merge
    let mut colonies = ColonySet::new();
    for path in json_paths {
        match read_from_json(path) {
            Ok(json_colonies) => {
                colonies = merge_sets(colonies, json_colonies);
            },
            Err(message) => println!("Could not read colonies from {}: {}", path, message),
        }
    }

    match write_to_csv(csv_path, colonies) {
        Ok(()) => {},
        Err(message) => println!("Could not write colonies: {}", message),
    }

}

fn read_from_json<P>(path: P) -> Result<ColonySet, String>  where P : AsRef<std::path::Path> {
    match File::open(path) {
        Ok(mut file) => {
            match Json::from_reader(&mut file) {
                Ok(json) => {
                    match json {
                        Json::Object(json_obj) => {
                            match json_obj.get("colonies") {
                                Some(colonies) => {
                                    match *colonies {
                                        Json::Array(ref colonies_array) => {
                                            let mut map = ColonySet::new();
                                            for ref colony_item in colonies_array {
                                                let colony_result = Colony::from_json((*colony_item).clone());
                                                match colony_result {
                                                    Ok(colony) => {
                                                        map.insert(colony.id, colony);
                                                    },
                                                    Err(message) => println!("Failed to read colony: {}", message),
                                                }
                                            }
                                            Ok(map)
                                        },
                                        _ => Err("Colonies item not an array".to_string()),
                                    }
                                },
                                _ => Err("No colonies item".to_string()),
                            }
                        },
                        _ => Err("JSON root is not an object".to_string()),
                    }
                }
                Err(_) => Err("Could not parse JSON".to_string()),
            }
        },
        Err(_) => Err("Could not open file".to_string()),
    }
}

fn write_to_csv<P>(path: P, colonies: ColonySet) -> Result<(), String> where P : AsRef<std::path::Path> {
    match File::create(path) {
        Ok(file) => {
            let mut writer = Writer::from_writer(file);
            for (_, colony) in colonies {
                let active_str = match colony.visited {
                    false => "",
                    true => match colony.active {
                        true => "A",
                        false => "NA",
                    }
                };
                let _ = writer.encode((colony.id, colony.x, colony.y, active_str));
            }
            Ok(())
        },
        Err(_) => Err("Could not open file".to_string()),
    }
}

fn merge_sets(set1: ColonySet, set2: ColonySet) -> ColonySet {
    // Procedure:
    // Copy all colonies from set1 to out
    // For each colony in set2, add to out
    // If a colony also exists in out, choose one and replace it

    let mut out = set1.clone();
    for (_, colony2) in set2 {
        if out.contains_key(&colony2.id) {
            let colony1 = *out.get(&colony2.id).unwrap();
            let chosen = choose_colony(colony1, colony2);
            out.insert(chosen.id, chosen);
        }
        else {
            out.insert(colony2.id, colony2);
        }
    }

    out
}

fn choose_colony(c1: Colony, c2: Colony) -> Colony {
    assert!(c1.id == c2.id);
    // Choose the more recently updated colony
    match c1.updated.cmp(&c2.updated) {
        Less => c2,
        Greater => c1,
        Equal => {
            // If one is active, choose it
            match (c1.active, c2.active) {
                (true, false) => c1,
                (false, true) => c2,
                _ => {
                    // Choose one arbitrarily
                    c1
                }
            }
        }
    }
}
