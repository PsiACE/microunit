mod error;
mod node_builder;
mod unit;

pub use error::NodeError;
pub use node_builder::NodeBuilder;
pub use unit::{Unit, UnitHandle};

pub(crate) type Result<T> = std::result::Result<T, NodeError>;

use std::collections::HashMap;
use std::sync::Mutex;

use uuid::Uuid;

use crate::proto::{NodeDesc, UnitDesc, UnitSpec};

pub struct Node {
    desc: NodeDesc,
    units: Mutex<HashMap<String, Box<dyn Unit>>>,
    unit_handles: Mutex<HashMap<String, Box<dyn UnitHandle>>>,
}

impl Node {
    pub fn new(units: HashMap<String, Box<dyn Unit>>) -> Node {
        let desc = NodeDesc {
            uuid: Uuid::new_v4().to_string(),
        };
        Node {
            desc,
            units: Mutex::new(units),
            unit_handles: Mutex::new(HashMap::new()),
        }
    }

    pub fn uuid(&self) -> String {
        self.desc.uuid.clone()
    }

    pub fn list_nodes(&self) -> Vec<NodeDesc> {
        vec![self.desc.clone()]
    }

    pub fn list_units(&self) -> Vec<UnitDesc> {
        let handles = self.unit_handles.lock().unwrap();
        handles.values().map(|x| x.desc()).collect()
    }

    pub fn create_unit(&self, spec: &UnitSpec) -> Result<UnitDesc> {
        let units = self.units.lock().unwrap();
        if let Some(unit) = units.get(&spec.kind) {
            let uuid = Uuid::new_v4().to_string();
            let handle = unit.spawn(uuid);
            let unit_desc = handle.desc();
            self.insert_unit_handle(handle);
            Ok(unit_desc)
        } else {
            let reason = format!("unknown unit kind {}", spec.kind);
            Err(NodeError::InvalidArgument(reason))
        }
    }

    fn insert_unit_handle(&self, handle: Box<dyn UnitHandle>) {
        let uuid = handle.desc().uuid;
        let mut handles = self.unit_handles.lock().unwrap();
        assert!(
            !handles.insert(uuid.clone(), handle).is_some(),
            "duplicated unit {}",
            uuid
        );
    }
}

unsafe impl Sync for Node {}
