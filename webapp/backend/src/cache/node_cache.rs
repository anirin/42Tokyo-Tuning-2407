use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::models::graph::Node;
use std::collections::HashMap;
use crate::domains::map_service::MapRepository;

#[derive(Clone)]
pub struct NodeCache {
    pub store: [Arc<Mutex<HashMap<i32, Node>>>; 7],
}

impl NodeCache {
    fn new() -> Self {
        NodeCache {
			store: [
				Arc::new(Mutex::new(HashMap::new())),
				Arc::new(Mutex::new(HashMap::new())),
				Arc::new(Mutex::new(HashMap::new())),
				Arc::new(Mutex::new(HashMap::new())),
				Arc::new(Mutex::new(HashMap::new())),
				Arc::new(Mutex::new(HashMap::new())),
				Arc::new(Mutex::new(HashMap::new())),
			]
        }
    }

	pub fn get_nodes(&self, area_id: usize) -> HashMap<i32, Node> {
		let store = self.store[area_id - 1].lock().unwrap();
		store.clone()
	}
}

lazy_static! {
    pub static ref NODE_CACHE: NodeCache = {
        let cache = NodeCache::new();

        cache
    };
}

pub async fn cache_nodes<T: MapRepository>(repository: T) {
	let mut nodeses: [HashMap<i32, Node>; 7] = [
		HashMap::new(),
		HashMap::new(),
		HashMap::new(),
		HashMap::new(),
		HashMap::new(),
		HashMap::new(),
		HashMap::new(),
	];
	for i in 1..7 {
		let i: usize = i;
		match repository.get_all_nodes(Some(i as i32)).await {
			Ok(nodes) => {
				for node in nodes {
					nodeses[i - 1].insert(node.id, node);
				}
			},
			Err(e) => {
				println!("Error: {:?}", e);
			}
		}
		let mut store = NODE_CACHE.store[i - 1].lock().unwrap();
		*store = nodeses[i - 1].clone();
	}
}