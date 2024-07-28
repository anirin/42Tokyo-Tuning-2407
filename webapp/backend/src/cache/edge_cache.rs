use std::sync::{Arc};
use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::models::graph::Edge;
use crate::domains::map_service::MapRepository;

#[derive(Clone)]
pub struct EdgeCache {
    pub store: [Arc<DashMap<i32, Vec<Edge>>>; 7],
}

impl EdgeCache {
    fn new() -> Self {
        EdgeCache {
            store: [
                Arc::new(DashMap::new()),
                Arc::new(DashMap::new()),
                Arc::new(DashMap::new()),
                Arc::new(DashMap::new()),
                Arc::new(DashMap::new()),
                Arc::new(DashMap::new()),
                Arc::new(DashMap::new()),
            ]
        }
    }

    pub fn get_edges(&self, area_id: usize) -> DashMap<i32, Vec<Edge>> {
        let store = self.store[area_id - 1].clone();
        store.iter().map(|entry| (*entry.key(), entry.value().clone())).collect()
    }

    pub fn update_edge_weight(&self, area_id: usize, node_a_id: i32, node_b_id: i32, weight: i32) {
        let store = self.store[area_id - 1].clone();
		if let Some(mut edges) = store.get_mut(&node_a_id) {
			for edge in edges.iter_mut() {
				if edge.node_b_id == node_b_id {
					edge.weight = weight;
				}
			}
		}
		if let Some(mut edges) = store.get_mut(&node_b_id) {
			for edge in edges.iter_mut() {
				if edge.node_b_id == node_a_id {
					edge.weight = weight;
				}
			}
		};
    }
}

lazy_static! {
    pub static ref EDGE_CACHE: EdgeCache = {
        let cache = EdgeCache::new();
        cache
    };
}

pub async fn cache_edges<T: MapRepository>(repository: T) {
    let mut edgeses: [DashMap<i32, Vec<Edge>>; 7] = [
        DashMap::new(),
        DashMap::new(),
        DashMap::new(),
        DashMap::new(),
        DashMap::new(),
        DashMap::new(),
        DashMap::new(),
    ];
    for i in 1..7 {
        let i: usize = i;
        match repository.get_all_edges(Some(i as i32)).await {
            Ok(edges) => {
                for edge in edges {
                    edgeses[i - 1]
                        .entry(edge.node_a_id)
                        .or_default()
                        .push(edge.clone());
                    let reverse_edge = Edge {
                        node_a_id: edge.node_b_id,
                        node_b_id: edge.node_a_id,
                        weight: edge.weight,
                    };
                    edgeses[i - 1]
                        .entry(reverse_edge.node_a_id)
                        .or_default()
                        .push(reverse_edge);
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        let store = EDGE_CACHE.store[i - 1].clone();
        for entry in edgeses[i - 1].iter() {
            store.insert(*entry.key(), entry.value().clone());
        }
    }
}