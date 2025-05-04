
//! Builds and manages the transit graph using stops as nodes and trips as edges.
//! Also provides utilities for calculating distances and closest nodes.

use std::collections::{HashMap, HashSet, VecDeque};
use crate::parser::{Stop, GTFSData};

/// Represents a graph of transit stops and their connections.
/// Used for centrality analysis and tract clustering.
pub struct TransitGraph {
    pub nodes: HashMap<String, Stop>,       // stop_id → Stop
    pub edges: HashMap<String, Vec<String>>, // stop_id → list of connected stop_ids
}

impl TransitGraph {
    /// Create a new, empty transit graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Build graph structure from GTFS stops and connections
    /// Inputs: GTFSData with stops and directional connections
    /// Populates nodes and edges fields
    pub fn build_from_gtfs(&mut self, data: &GTFSData) {
        self.nodes = data.stops.clone();

        for conn in &data.connections {
            self.edges
                .entry(conn.from_stop_id.clone())
                .or_insert_with(Vec::new)
                .push(conn.to_stop_id.clone());
        }
    }

    /// Compute closeness centrality for each node using BFS
    /// Returns: HashMap of stop_id to centrality score
    pub fn compute_closeness_centrality(&self) -> HashMap<String, f64> {
        let mut centrality = HashMap::new();

        for node in self.nodes.keys() {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            let mut distance_sum = 0.0;

            visited.insert(node.clone());
            queue.push_back((node.clone(), 0));

            // Breadth-first search to accumulate distances
            while let Some((current, dist)) = queue.pop_front() {
                distance_sum += dist as f64;

                if let Some(neighbors) = self.edges.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back((neighbor.clone(), dist + 1));
                        }
                    }
                }
            }

            // Avoid divide-by-zero if disconnected
            if distance_sum > 0.0 {
                let score = (visited.len() as f64 - 1.0) / distance_sum;
                centrality.insert(node.clone(), score);
            }
        }

        centrality
    }

    /// Find the stop closest to a given latitude/longitude using Euclidean distance
    /// Inputs: lat, lon
    /// Output: Option<(stop_id, distance)>
    pub fn find_closest_stop(&self, lat: f64, lon: f64) -> Option<(String, f64)> {
        let mut closest: Option<(String, f64)> = None;

        for (id, stop) in &self.nodes {
            let dist = Self::euclidean_distance(lat, lon, stop.lat, stop.lon);
            match &closest {
                Some((_, best_dist)) if dist < *best_dist => {
                    closest = Some((id.clone(), dist));
                }
                None => {
                    closest = Some((id.clone(), dist));
                }
                _ => {}
            }
        }

        closest
    }

    /// Compute straight-line (Euclidean) distance between two points
    fn euclidean_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let dlat = lat1 - lat2;
        let dlon = lon1 - lon2;
        (dlat.powi(2) + dlon.powi(2)).sqrt()
    }
}