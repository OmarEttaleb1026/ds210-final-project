
//! Main control module: Loads data, builds graph, computes centrality,
//! assigns census tracts, and prints final summaries to terminal.

use ds210_finalproj::parser::{load_gtfs_data, load_census_csv};
use ds210_finalproj::graph::TransitGraph;
use ds210_finalproj::analysis::{compute_centrality_to_csv, cluster_neighborhoods_to_csv};
use std::fs::{create_dir_all, File};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Loading datasets...");

    // Load GTFS transit data and census tract data from file system
    let gtfs = load_gtfs_data("data/gtfs").expect("Failed to load GTFS");
    let census = load_census_csv("data/Census_Tracts_2010.csv").expect("Failed to load census CSV");

    println!("Constructing transit graph...");
    // Build graph from GTFS stop connections
    let mut graph = TransitGraph::new();
    graph.build_from_gtfs(&gtfs);

    println!("Saving results to output/...");
    // Ensure output folder exists
    create_dir_all("output").expect("Failed to create output directory");

    // Compute and save centrality scores to CSV
    compute_centrality_to_csv(&graph, "output/centrality.csv").expect("Failed to write centrality.csv");

    // Cluster census tracts to nearest stop and write results
    cluster_neighborhoods_to_csv(&graph, &census, "output/tract_clusters.csv").expect("Failed to write tract_clusters.csv");

    println!("Done. Files written to output/ directory.");

    // Print result summaries
    print_top_5_central_stops_with_names(&graph, "output/centrality.csv");
    print_tract_counts("output/tract_clusters.csv");
    print_tract_opportunity_summary("output/tract_opportunity.csv");
}

/// Print top 5 stops with highest closeness centrality
/// Inputs: reference to graph and path to centrality CSV
/// Output: printed ranked stop info with name and score
fn print_top_5_central_stops_with_names(graph: &TransitGraph, path: &str) {
    println!("\nTop 5 Most Central Stops (with names):");

    // Load centrality CSV
    let data = std::fs::read_to_string(path).expect("Failed to read centrality.csv");

    // Parse each line into (stop_id, score)
    let mut rows: Vec<(String, f64)> = data
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                let stop_id = parts[0].to_string();
                let closeness = parts[1].parse::<f64>().ok()?;
                Some((stop_id, closeness))
            } else {
                None
            }
        })
        .collect();

    // Sort descending by closeness score
    rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Print top 5
    for (i, (stop_id, score)) in rows.iter().take(5).enumerate() {
        let name = graph.nodes.get(stop_id).map(|s| s.name.as_str()).unwrap_or("Unknown");
        println!("{}. {} (ID: {}) → Closeness: {:.4}", i + 1, name, stop_id, score);
    }
}

/// Count how many census tracts were assigned to each stop
/// Inputs: path to tract_clusters.csv
/// Output: prints top 5 stops by number of tracts
fn print_tract_counts(path: &str) {
    println!("\nNumber of Census Tracts Assigned to Each Stop:");

    // Read tract-cluster assignment file
    let data = std::fs::read_to_string(path).expect("Failed to read tract_clusters.csv");
    let mut counts: HashMap<String, usize> = HashMap::new();

    // For each tract, increment the count for the assigned stop
    for line in data.lines().skip(1) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 3 {
            let stop_id = parts[2].to_string();
            *counts.entry(stop_id).or_insert(0) += 1;
        }
    }

    // Sort by count, descending
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    // Show top 5
    for (stop_id, count) in sorted.iter().take(5) {
        println!("Stop ID: {} → {} census tracts", stop_id, count);
    }
}

/// Print summary of tract → stop → income merged dataset
/// Inputs: path to tract_opportunity.csv
/// Output: first 5 merged records showing tract and income
fn print_tract_opportunity_summary(path: &str) {
    println!("\n📊 Tract Opportunity Summary (first 5 rows):");

    let file = File::open(path).expect("Failed to open tract_opportunity.csv");
    let reader = BufReader::new(file);

    // Print first 5 tract records
    for (i, line) in reader.lines().enumerate().skip(1).take(5) {
        if let Ok(row) = line {
            let parts: Vec<&str> = row.split(',').collect();
            if parts.len() == 4 {
                let tract_id = parts[0];
                let tract_name = parts[1];
                let stop_id = parts[2];
                let income = parts[3];
                println!(
                    "Tract {} ({}) → Closest Stop: {} → Median Income: ${}",
                    tract_id, tract_name, stop_id, income
                );
            }
        }
    }
}
