use super::dto::tow_truck::TowTruckDto;
use super::map_service::MapRepository;
use super::order_service::OrderRepository;
use crate::errors::AppError;
use crate::models::graph::Graph;
use crate::models::tow_truck::TowTruck;
use crate::cache::node_cache::NODE_CACHE;
use crate::cache::edge_cache::EDGE_CACHE;

pub trait TowTruckRepository {
    async fn get_paginated_tow_trucks(
        &self,
        page: i32,
        page_size: i32,
        status: Option<String>,
        area_id: Option<i32>,
    ) -> Result<Vec<TowTruck>, AppError>;
    async fn update_location(&self, truck_id: i32, node_id: i32) -> Result<(), AppError>;
    async fn update_status(&self, truck_id: i32, status: &str) -> Result<(), AppError>;
    async fn find_tow_truck_by_id(&self, id: i32) -> Result<Option<TowTruck>, AppError>;
}

#[derive(Debug)]
pub struct TowTruckService<
    T: TowTruckRepository + std::fmt::Debug,
    U: OrderRepository + std::fmt::Debug,
    V: MapRepository + std::fmt::Debug,
> {
    tow_truck_repository: T,
    order_repository: U,
    map_repository: V,
}

impl<
        T: TowTruckRepository + std::fmt::Debug,
        U: OrderRepository + std::fmt::Debug,
        V: MapRepository + std::fmt::Debug,
    > TowTruckService<T, U, V>
{
    pub fn new(tow_truck_repository: T, order_repository: U, map_repository: V) -> Self {
        TowTruckService {
            tow_truck_repository,
            order_repository,
            map_repository,
        }
    }

    pub async fn get_tow_truck_by_id(&self, id: i32) -> Result<Option<TowTruckDto>, AppError> {
        let tow_truck = self.tow_truck_repository.find_tow_truck_by_id(id).await?;
        Ok(tow_truck.map(TowTruckDto::from_entity))
    }

    pub async fn get_all_tow_trucks(
        &self,
        page: i32,
        page_size: i32,
        status: Option<String>,
        area: Option<i32>,
    ) -> Result<Vec<TowTruckDto>, AppError> {
        let tow_trucks = self
            .tow_truck_repository
            .get_paginated_tow_trucks(page, page_size, status, area)
            .await?;
        let tow_truck_dtos = tow_trucks
            .into_iter()
            .map(TowTruckDto::from_entity)
            .collect();

        Ok(tow_truck_dtos)
    }

    pub async fn update_location(&self, truck_id: i32, node_id: i32) -> Result<(), AppError> {
        self.tow_truck_repository
            .update_location(truck_id, node_id)
            .await?;

        Ok(())
    }

    pub async fn get_nearest_available_tow_trucks(
        &self,
        order_id: i32,
    ) -> Result<Option<TowTruckDto>, AppError> {
        let order = self.order_repository.find_order_by_id(order_id).await?;
        let area_id = self
            .map_repository
            .get_area_id_by_node_id(order.node_id)
            .await?;
        let tow_trucks = self
            .tow_truck_repository
            .get_paginated_tow_trucks(0, -1, Some("available".to_string()), Some(area_id))
            .await?;

		let mut graph = Graph::new();
		graph.nodes = NODE_CACHE.get_nodes(area_id as usize);
        if area_id <= 3 {
            graph.edges = EDGE_CACHE.get_edges(area_id as usize);
        }
        else {
		let nodes = self.map_repository.get_all_edges(Some(area_id)).await.unwrap();
		for node in nodes {
			graph.add_edge(node);
		}}

		let truck = graph.find_nearest_tow_truck(tow_trucks, order.node_id);
		let result: Option<TowTruckDto> = match truck {
			Some(truck) => Some(TowTruckDto::from_entity(truck)),
			None => None,
		};

		Ok(result)
    }
}
