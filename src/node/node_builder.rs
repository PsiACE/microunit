use std::collections::HashMap;

use super::unit::Unit;
use crate::node::Node;

pub struct NodeBuilder {
    units: HashMap<String, Box<dyn Unit>>,
}

impl NodeBuilder {
    pub fn new() -> NodeBuilder {
        NodeBuilder {
            units: HashMap::new(),
        }
    }

    pub fn add_unit<U>(mut self, unit: U) -> Self
    where
        U: Unit + 'static,
    {
        let kind = unit.kind();
        assert!(
            !self.units.insert(kind.clone(), Box::new(unit)).is_some(),
            "duplicated unit kind {}",
            kind
        );
        self
    }

    pub fn build(self) -> Node {
        Node::new(self.units)
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
