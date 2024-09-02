use std::collections::HashMap;
use std::clone::Clone;
use std::hash::Hash;
use std::cmp::Eq;

pub struct Graph<K: Clone + Eq + Hash, N, E: Eq + Hash> {
    nodes: HashMap<K, N>,
    edges: HashMap<K, HashMap<E, K>>,
}

impl<K: Clone + Eq + Hash, N, E: Eq + Hash> Graph<K, N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, key: K, value: N) {
        self.nodes.insert(key.clone(), value);
        self.edges.insert(key.clone(), HashMap::new());
    }

    pub fn get_node(&self, key: K) -> Option<&N> {
        self.nodes.get(&key)
    }

    pub fn add_edge(&mut self, from: K, to: K, edge: E) {
        self.edges.get_mut(&from).unwrap().insert(edge, to);
    }

    pub fn get_edge(&self, from: K, edge: E) -> Option<&K> {
        self.edges.get(&from).unwrap().get(&edge)
    }
}