
//! Responsible for loading and parsing GTFS transit data and census tract geometry from CSV files.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Represents a transit stop, including its ID, name, and coordinates.
/// Used as a node in the TransitGraph.
#[derive(Clone, Debug)]
pub struct Stop {
    pub stop_id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

/// Represents a transit route, currently only tracking route ID.
/// Included for extensibility.
#[derive(Debug)]
pub struct Route {
    pub route_id: String,
}

/// Represents a connection between two stops (an edge in the transit graph).
#[derive(Debug)]
pub struct Connection {
    pub from_stop_id: String,
    pub to_stop_id: String,
}

/// Represents a GTFS dataset: stops and connections between them.
#[derive(Debug)]
pub struct GTFSData {
    pub stops: HashMap<String, Stop>,
    pub connections: Vec<Connection>,
}

/// Load GTFS transit data from a directory containing GTFS CSV files.
/// Inputs: path to directory (string)
/// Outputs: GTFSData struct (stops + connections)
pub fn load_gtfs_data(dir: &str) -> Result<GTFSData, std::io::Error> {
    let stops_path = format!("{}/stops.txt", dir);
    let stop_times_path = format!("{}/stop_times.txt", dir);
    let trips_path = format!("{}/trips.txt", dir);

    let stops_file = File::open(stops_path)?;
    let stop_times_file = File::open(stop_times_path)?;
    let trips_file = File::open(trips_path)?;

    let stops_reader = BufReader::new(stops_file);
    let stop_times_reader = BufReader::new(stop_times_file);
    let trips_reader = BufReader::new(trips_file);

    let mut stops = HashMap::new();
    let mut stop_sequence_map: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    // Parse stops.txt → Build map of stop_id to Stop struct
    for (i, line) in stops_reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 6 {
            continue;
        }
        let stop = Stop {
            stop_id: parts[0].to_string(),
            name: parts[2].to_string(),
            lat: parts[4].parse().unwrap_or(0.0),
            lon: parts[5].parse().unwrap_or(0.0),
        };
        stops.insert(stop.stop_id.clone(), stop);
    }

    // Parse stop_times.txt → Map trip_id to list of stop sequences
    for (i, line) in stop_times_reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 4 {
            continue;
        }
        let trip_id = parts[0];
        let stop_id = parts[3];
        let stop_sequence = parts[4].parse::<u32>().unwrap_or(0);
        stop_sequence_map
            .entry(trip_id.to_string())
            .or_insert_with(Vec::new)
            .push((stop_sequence, stop_id.to_string()));
    }

    let mut connections = Vec::new();

    // Build connections by ordering stops in each trip
    for (_trip_id, mut stops_seq) in stop_sequence_map {
        stops_seq.sort_by_key(|(seq, _)| *seq);
        for i in 0..stops_seq.len().saturating_sub(1) {
            let from = &stops_seq[i].1;
            let to = &stops_seq[i + 1].1;
            connections.push(Connection {
                from_stop_id: from.clone(),
                to_stop_id: to.clone(),
            });
        }
    }

    Ok(GTFSData { stops, connections })
}

/// Load census tract data from CSV file with tract ID, name, and lat/lon.
/// Inputs: path to CSV file
/// Outputs: Vector of (tract_id, name, lat, lon) tuples
pub fn load_census_csv(path: &str) -> Result<Vec<(String, String, f64, f64)>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();

    // Parse each row into tract data tuple
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 4 {
            continue;
        }
        let tract_id = parts[0].to_string();
        let tract_name = parts[1].to_string();
        let lat = parts[2].parse::<f64>().unwrap_or(0.0);
        let lon = parts[3].parse::<f64>().unwrap_or(0.0);
        results.push((tract_id, tract_name, lat, lon));
    }

    Ok(results)
}