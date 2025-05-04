
//! Contains analysis logic: centrality computation and assigning census tracts to the closest stops,
//! then writing results to CSV for external use or terminal summary.

use crate::graph::TransitGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Compute closeness centrality for all stops in the graph
/// and write results to a CSV file.
/// Inputs:
/// - graph: reference to TransitGraph
/// - output_path: path to output CSV
/// Output: Result<(), std::io::Error>
pub fn compute_centrality_to_csv(graph: &TransitGraph, output_path: &str) -> Result<(), std::io::Error> {
    let closeness = graph.compute_closeness_centrality();
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "stop_id,closeness")?;
    // Write each stop_id and its centrality score to file
    for (stop_id, score) in closeness {
        writeln!(writer, "{},{}", stop_id, score)?;
    }

    Ok(())
}

/// Assign each census tract to the closest transit stop,
/// and write the assignments to a CSV file.
/// Inputs:
/// - graph: reference to TransitGraph
/// - census: vector of (tract_id, tract_name, lat, lon)
/// - output_path: path to write results
/// Output: Result<(), std::io::Error>
pub fn cluster_neighborhoods_to_csv(
    graph: &TransitGraph,
    census: &Vec<(String, String, f64, f64)>,
    output_path: &str,
) -> Result<(), std::io::Error> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "tract_id\ttract_name\tclosest_stop_id")?;

    for (i, (tract_id, tract_name, lat, lon)) in census.iter().enumerate() {
        if i % 100 == 0 {
            println!("Processing tract {} of {}", i + 1, census.len());
        }

        // Find the stop closest to this tract's lat/lon
        if let Some((closest_stop, _)) = graph.find_closest_stop(*lat, *lon) {
            writeln!(writer, "{}\t{}\t{}", tract_id, tract_name, closest_stop)?;
        }
    }

    Ok(())
}