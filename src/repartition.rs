use multimap::MultiMap;
use std::collections::{HashMap, HashSet};
use crate::Instance;


pub struct RepartitionClusters<'r> {
    clusters: MultiMap<&'r Instance, &'r Instance>,
    node_to_cluster: HashMap<&'r Instance, &'r Instance>,
    pub bruit: HashSet<&'r Instance>,
}

impl<'r> RepartitionClusters<'r> {
    pub fn new() -> RepartitionClusters<'r> {
        RepartitionClusters {
            clusters: MultiMap::new(),
            node_to_cluster: HashMap::new(),
            bruit: HashSet::new(),
        }
    }

    pub fn clusters(&self) -> &MultiMap<&'r Instance, &'r Instance> { &self.clusters }

    pub fn deja_visite(&self, node: &'r Instance) -> bool {
        self.node_to_cluster.contains_key(node) || self.bruit.contains(node)
    }

    pub fn get_cluster_vec(&self, node: &'r Instance) -> Option<&Vec<&'r Instance>> {
        self.node_to_cluster.get(node)
            .and_then(|n| self.clusters.get_vec(n))
    }

    pub fn create_cluster(&mut self, node: &'r Instance) {
        self.clusters.insert(node, node);
        self.node_to_cluster.insert(node, node);
    }

    pub fn add_to_cluster(&mut self, key: &'r Instance, value: &'r Instance) {
        self.clusters.insert(key, value);
        self.node_to_cluster.insert(value, key);
    }
    
    pub fn get_cluster_by_id(&self,id: usize) -> Option<&'r Instance>{
        self.clusters.keys().skip(id).next().map(|inst|*inst)
    }
}

