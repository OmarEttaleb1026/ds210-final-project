// project210/tests/tests.rs

use ds210_finalproj::parser::{load_gtfs_data, load_census_csv};
use ds210_finalproj::graph::TransitGraph;
use ds210_finalproj::analysis::compute_centrality;

#[test]
fn test_load_gtfs_data() {
    let result = load_gtfs_data("data/gtfs");
    assert!(result.is_ok(), "Failed to load GTFS data");

    let gtfs = result.unwrap();
    assert!(!gtfs.stops.is_empty(), "No stops loaded");
    assert!(!gtfs.routes.is_empty(), "No routes loaded");
}

#[test]
fn test_build_graph_and_neighbors() {
    let gtfs = load_gtfs_data("data/gtfs").unwrap();
    let mut graph = TransitGraph::new();
    graph.build_from_gtfs(&gtfs);

    assert!(!graph.nodes.is_empty(), "Graph nodes are empty");
    assert!(!graph.edges.is_empty(), "Graph edges are empty");

    let sample_id = graph.nodes.keys().next().unwrap();
    let neighbors = graph.neighbors(sample_id);
    assert!(neighbors.len() > 0, "No neighbors found for sample stop");
}

#[test]
fn test_load_census_csv() {
    let result = load_census_csv("data/Census_Tracts_2010.csv");
    assert!(result.is_ok(), "Failed to load census CSV");

    let tracts = result.unwrap();
    assert!(!tracts.is_empty(), "No tracts loaded");
}

#[test]
fn test_centrality_runs() {
    let gtfs = load_gtfs_data("data/gtfs").unwrap();
    let mut graph = TransitGraph::new();
    graph.build_from_gtfs(&gtfs);

    // Ensure it doesn't panic
    compute_centrality(&graph);
}
