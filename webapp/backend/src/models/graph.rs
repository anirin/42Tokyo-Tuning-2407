use sqlx::FromRow;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use crate::models::tow_truck::TowTruck;
use dashmap::DashMap;

#[derive(FromRow, Clone, Debug, Default)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug, Default)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: DashMap<i32, Vec<Edge>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: DashMap::new(),
        }
    }

    pub fn update_edge_weight(&mut self, node_a_id: i32, node_b_id: i32, weight: i32) {
        {
            if let Some(mut edges) = self.edges.get_mut(&node_a_id) {
                for edge in edges.iter_mut() {
                    if edge.node_b_id == node_b_id {
                        edge.weight = weight;
                    }
                }
            }
        }
        if let Some(mut edges) = self.edges.get_mut(&node_b_id) {
            for edge in edges.iter_mut() {
                if edge.node_b_id == node_a_id {
                    edge.weight = weight;
                }
            }
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn find_nearest_tow_truck(&self, tow_trucks: Vec<TowTruck>, from_node_id: i32) -> Option<TowTruck> {
        let mut distances = HashMap::new();
        let mut heap = BinaryHeap::new();

        // Initialize distances
        for node_id in self.nodes.keys() {
            distances.insert(*node_id, i32::MAX);
        }
        distances.insert(from_node_id, 0);

        // Push the start node into the heap
        heap.push(State { cost: 0, position: from_node_id });

        while let Some(State { cost, position }) = heap.pop() {
            // if a track is found, return it
            if let Some(tow_truck) = tow_trucks.iter().find(|t| t.node_id == position) {
                return Some(tow_truck.clone());
            }

            // If the cost is greater than the recorded distance, skip it
            if cost > *distances.get(&position).unwrap_or(&i32::MAX) {
                continue;
            }

            // Update distances to neighboring nodes
            if let Some(edges_ref) = self.edges.get(&position) {
                let edges = edges_ref.value();
                for edge in edges {
                    let next = State { cost: cost + edge.weight, position: edge.node_b_id };

                    if next.cost < *distances.get(&next.position).unwrap_or(&i32::MAX) {
                        distances.insert(next.position, next.cost);
                        heap.push(next);
                    }
                }
            }
        }
        return None;
    }
}