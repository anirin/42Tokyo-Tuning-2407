use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::models::graph::Graph;
use crate::domains::map_service::MapRepository;


#[derive(Clone)]
pub struct GraphCache {
    store: Arc<Mutex<[Graph; 7]>>,
}

impl GraphCache {
    fn new() -> Self {
        GraphCache {
			store: Arc::new(Mutex::new([
				Graph::new(),
				Graph::new(),
				Graph::new(),
				Graph::new(),
				Graph::new(),
				Graph::new(),
				Graph::new(),
			])),
        }
    }

	pub fn update_edge_weight(&self, area_id: usize, node_a_id: i32, node_b_id: i32, weight: i32) {
		let mut store = self.store.lock().unwrap();
		store[area_id - 1].update_edge_weight(node_a_id, node_b_id, weight);
	}

	pub fn get_graph(&self, area_id: usize) -> Graph {
		let store = self.store.lock().unwrap();
		store[area_id - 1].clone()
	}
}

lazy_static! {
    pub static ref GRAPH_CACHE: GraphCache = {
        let cache = GraphCache::new();

        cache
    };
}

pub async fn cache_graph<T: MapRepository>(repository: T) {
	let mut graphs: [Graph; 7] = [
		Graph::new(),
		Graph::new(),
		Graph::new(),
		Graph::new(),
		Graph::new(),
		Graph::new(),
		Graph::new(),
	];
	for i in 1..7 {
		let i: usize = i;
		match repository.get_all_nodes(Some(i as i32)).await {
			Ok(nodes) => {
				for node in nodes {
					graphs[i-1].add_node(node);
				}
			},
			Err(e) => {
				println!("Error: {:?}", e);
			}
		}
		match repository.get_all_edges(Some(i as i32)).await {
			Ok(edges) => {
				for edge in edges {
					graphs[i-1].add_edge(edge);
				}
			},
			Err(e) => {
				println!("Error: {:?}", e);
			}
		}
	}

	let mut store = GRAPH_CACHE.store.lock().unwrap();
	*store = graphs;
}