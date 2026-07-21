use std::collections::{HashMap, HashSet};

use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Undirected;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Typed dispatch basis for a segment (REQ-007): is the corridor served on
/// dedicated track or shared with a host (freight) railroad?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Dispatch {
    Dedicated,
    SharedHost,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub state: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,
    pub miles: f64,
    pub max_mph: f64,
    pub dispatch: Dispatch,
}

#[derive(Debug, Error, PartialEq)]
pub enum NetworkError {
    #[error("station id already exists: {0}")]
    DuplicateStation(String),
    #[error("segment id already exists: {0}")]
    DuplicateSegment(String),
    #[error("unknown station id: {0}")]
    UnknownStation(String),
    #[error("segment miles must be positive for {segment_id}: {miles}")]
    NonPositiveMiles { segment_id: String, miles: f64 },
    #[error("segment max_mph must be positive for {segment_id}: {max_mph}")]
    NonPositiveSpeed { segment_id: String, max_mph: f64 },
    #[error("no segment connects {from} to {to}")]
    NoSegment { from: String, to: String },
}

#[derive(Debug, Default)]
pub struct Network {
    graph: Graph<Station, Segment, Undirected>,
    stations_by_id: HashMap<String, NodeIndex>,
    segment_ids: HashSet<String>,
}

impl Network {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_station(&mut self, station: Station) -> Result<(), NetworkError> {
        if self.stations_by_id.contains_key(&station.id) {
            return Err(NetworkError::DuplicateStation(station.id));
        }
        let id = station.id.clone();
        let index = self.graph.add_node(station);
        self.stations_by_id.insert(id, index);
        Ok(())
    }

    pub fn add_segment(
        &mut self,
        from_station: &str,
        to_station: &str,
        segment: Segment,
    ) -> Result<(), NetworkError> {
        if segment.miles <= 0.0 {
            return Err(NetworkError::NonPositiveMiles {
                segment_id: segment.id,
                miles: segment.miles,
            });
        }
        if segment.max_mph <= 0.0 {
            return Err(NetworkError::NonPositiveSpeed {
                segment_id: segment.id,
                max_mph: segment.max_mph,
            });
        }
        if self.segment_ids.contains(&segment.id) {
            return Err(NetworkError::DuplicateSegment(segment.id));
        }
        let from = self.node_index(from_station)?;
        let to = self.node_index(to_station)?;
        let segment_id = segment.id.clone();
        self.graph.add_edge(from, to, segment);
        self.segment_ids.insert(segment_id);
        Ok(())
    }

    pub fn station_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn segment_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Degree of a station — the connectivity/centrality proxy for DIM-04.
    pub fn degree(&self, station_id: &str) -> Result<usize, NetworkError> {
        let index = self.node_index(station_id)?;
        Ok(self.graph.edges(index).count())
    }

    pub fn is_connected(&self, a: &str, b: &str) -> Result<bool, NetworkError> {
        let start = self.node_index(a)?;
        let goal = self.node_index(b)?;
        Ok(self.reachable(start, goal))
    }

    /// Total route miles along a corridor given as a station-id path.
    pub fn corridor_miles(&self, path: &[&str]) -> Result<f64, NetworkError> {
        let mut total = 0.0_f64;
        for window in path.windows(2) {
            total += self.segment_between(window[0], window[1])?.miles;
        }
        Ok(total)
    }

    /// Free-flow trip time (minutes) along a corridor: sum of segment
    /// `miles / max_mph * 60`.
    pub fn trip_time_minutes(&self, path: &[&str]) -> Result<f64, NetworkError> {
        let mut total = 0.0_f64;
        for window in path.windows(2) {
            let segment = self.segment_between(window[0], window[1])?;
            total += segment.miles / segment.max_mph * 60.0;
        }
        Ok(total)
    }

    fn segment_between(&self, from: &str, to: &str) -> Result<&Segment, NetworkError> {
        let from_index = self.node_index(from)?;
        let to_index = self.node_index(to)?;
        self.graph
            .edges(from_index)
            .find(|edge| edge.target() == to_index || edge.source() == to_index)
            .map(|edge| edge.weight())
            .ok_or_else(|| NetworkError::NoSegment {
                from: from.to_string(),
                to: to.to_string(),
            })
    }

    fn node_index(&self, station_id: &str) -> Result<NodeIndex, NetworkError> {
        self.stations_by_id
            .get(station_id)
            .copied()
            .ok_or_else(|| NetworkError::UnknownStation(station_id.to_string()))
    }

    fn reachable(&self, start: NodeIndex, goal: NodeIndex) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            if node == goal {
                return true;
            }
            if !visited.insert(node) {
                continue;
            }
            for edge in self.graph.edges(node) {
                let neighbor = if edge.target() == node {
                    edge.source()
                } else {
                    edge.target()
                };
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn station(id: &str) -> Station {
        Station {
            id: id.to_string(),
            name: format!("{id} station"),
            state: "CA".to_string(),
        }
    }

    fn segment(id: &str, miles: f64, max_mph: f64, dispatch: Dispatch) -> Segment {
        Segment {
            id: id.to_string(),
            miles,
            max_mph,
            dispatch,
        }
    }

    fn three_station_corridor() -> Network {
        let mut network = Network::new();
        network
            .add_station(station("a"))
            .expect("station a accepted");
        network
            .add_station(station("b"))
            .expect("station b accepted");
        network
            .add_station(station("c"))
            .expect("station c accepted");
        network
            .add_segment("a", "b", segment("ab", 60.0, 60.0, Dispatch::Dedicated))
            .expect("segment ab accepted");
        network
            .add_segment("b", "c", segment("bc", 90.0, 90.0, Dispatch::SharedHost))
            .expect("segment bc accepted");
        network
    }

    #[test]
    fn builds_graph_and_counts_stations_and_segments() {
        let network = three_station_corridor();
        assert_eq!(network.station_count(), 3);
        assert_eq!(network.segment_count(), 2);
    }

    #[test]
    fn degree_counts_incident_segments() {
        let network = three_station_corridor();
        assert_eq!(network.degree("b"), Ok(2));
        assert_eq!(network.degree("a"), Ok(1));
    }

    #[test]
    fn connectivity_distinguishes_reachable_and_gap() {
        let mut network = three_station_corridor();
        network
            .add_station(station("z"))
            .expect("station z accepted");
        assert_eq!(network.is_connected("a", "c"), Ok(true));
        assert_eq!(network.is_connected("a", "z"), Ok(false));
    }

    #[test]
    fn corridor_miles_sums_segment_lengths() {
        let network = three_station_corridor();
        assert_eq!(network.corridor_miles(&["a", "b", "c"]), Ok(150.0));
    }

    #[test]
    fn trip_time_minutes_sums_free_flow_segment_times() {
        let network = three_station_corridor();
        // 60 mi @ 60 mph = 60 min; 90 mi @ 90 mph = 60 min.
        assert_eq!(network.trip_time_minutes(&["a", "b", "c"]), Ok(120.0));
    }

    #[test]
    fn segment_dispatch_basis_is_preserved() {
        let network = three_station_corridor();
        let bases = network
            .graph
            .edge_weights()
            .map(|segment| segment.dispatch)
            .collect::<HashSet<_>>();
        assert!(bases.contains(&Dispatch::Dedicated));
        assert!(bases.contains(&Dispatch::SharedHost));
    }

    #[test]
    fn duplicate_station_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network
            .add_station(station("a"))
            .expect("first station accepted");
        assert_eq!(
            network.add_station(station("a")),
            Err(NetworkError::DuplicateStation("a".to_string()))
        );
    }

    #[test]
    fn duplicate_segment_is_rejected_with_typed_error() {
        let mut network = three_station_corridor();
        assert_eq!(
            network.add_segment("a", "c", segment("ab", 10.0, 50.0, Dispatch::Dedicated)),
            Err(NetworkError::DuplicateSegment("ab".to_string()))
        );
    }

    #[test]
    fn non_positive_miles_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network
            .add_station(station("a"))
            .expect("station a accepted");
        network
            .add_station(station("b"))
            .expect("station b accepted");
        assert_eq!(
            network.add_segment("a", "b", segment("ab", 0.0, 60.0, Dispatch::Dedicated)),
            Err(NetworkError::NonPositiveMiles {
                segment_id: "ab".to_string(),
                miles: 0.0
            })
        );
    }

    #[test]
    fn non_positive_speed_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network
            .add_station(station("a"))
            .expect("station a accepted");
        network
            .add_station(station("b"))
            .expect("station b accepted");
        assert_eq!(
            network.add_segment("a", "b", segment("ab", 60.0, 0.0, Dispatch::Dedicated)),
            Err(NetworkError::NonPositiveSpeed {
                segment_id: "ab".to_string(),
                max_mph: 0.0
            })
        );
    }

    #[test]
    fn unknown_station_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network
            .add_station(station("a"))
            .expect("station a accepted");
        assert_eq!(
            network.add_segment(
                "a",
                "missing",
                segment("am", 10.0, 40.0, Dispatch::Dedicated)
            ),
            Err(NetworkError::UnknownStation("missing".to_string()))
        );
        assert_eq!(
            network.degree("missing"),
            Err(NetworkError::UnknownStation("missing".to_string()))
        );
    }

    #[test]
    fn trip_time_errors_when_no_segment_connects_path() {
        let mut network = three_station_corridor();
        network
            .add_station(station("d"))
            .expect("station d accepted");
        assert_eq!(
            network.trip_time_minutes(&["a", "d"]),
            Err(NetworkError::NoSegment {
                from: "a".to_string(),
                to: "d".to_string()
            })
        );
    }
}
