use std::sync::Arc;
use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use crate::sensor::Sensor;

/// Trait for the declaration of an underlying structure. The struct which implements the trait
/// has to have an field, which is of type [`petgraph::Graph`](petgraph::Graph), that has nodes of type [`Nodes`] and edges of type [`Edge`].
pub trait UnderlyingStructure {
    /// Gets the necessary graph structure.
    fn get_graph_structure(&self) -> &Graph<Node<Arc<dyn NodeData>>, Edge<Arc<dyn EdgeData>>, Undirected>;
    /*
    /// Sets the scheme for a start node id.
    fn set_start_nodes(&mut self, id_scheme: String);
    */
    /// Gets the start nodes. Nodes in which movable objects should be seen first.
    fn get_start_nodes(&self) -> Vec<NodeIndex>;
    /*
    /// Sets
    fn set_end_nodes(&mut self);
    */
    /// Gets the end nodes. Nodes in which movable objects should be seen last.
    fn get_end_nodes(&self) -> Vec<NodeIndex>;

    /// Get the nodes todo!()
    fn get_nodes_to_move_to(&self) -> Vec<NodeIndex>;
}

/// Trait for a vertex to specify different sections of a underlying structure.
/// <br/>For example the representation of a building etc.
pub trait NodeData {
    /// Function to get Node id.
    fn get_id(&self) -> String;
}

/// Trait for a edge to specify different connections of a underlying structure.
/// <br/>For example the representation of a connection in a building.
pub trait EdgeData {
    /// Function to get Edge id.
    fn get_id(&self) -> String;
}

pub struct Edge<T>{
    data: T,
    sensors: Vec<Sensor>
}

impl Edge<Arc<dyn EdgeData>> {
    /// Creates NewEdge.
    pub fn new(data: Arc<dyn EdgeData>) -> Edge<Arc<dyn EdgeData>> {
        return Edge {
      //      weight: 1,
            data,
            sensors: Vec::<Sensor>::new()
        }
    }

    /// Gets data.
    pub fn get_data(&self) -> &Arc<dyn EdgeData> {
        &self.data
    }

    /// Adds sensor.
    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    /// Gets copy of sensors.
    pub fn get_sensors(&self) -> Vec<Sensor> {
        self.sensors.to_vec()
    }

}

pub struct Node<T> {
    data: T,
    sensors: Vec<Sensor>
}

impl Node<Arc<dyn NodeData>> {
    /// Creates NewNode
    pub fn new(data: Arc<dyn NodeData>) -> Node<Arc<dyn NodeData>> {
        return Node {
            data,
            sensors: Vec::<Sensor>::new()
        }
    }

    /// Gets data.
    pub fn get_data(&self) -> &Arc<dyn NodeData> {
        &self.data
    }

    /// Adds sensor.
    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    /// Adds a vector of sensors.
    pub fn add_sensors(&mut self, sensors: Vec<Sensor>) {
        for sensor in sensors {
            self.add_sensor(sensor);
        }
    }

    /// Gets sensors.
    pub fn get_sensors(&self) -> Vec<Sensor> {
        self.sensors.to_vec()
    }

}
