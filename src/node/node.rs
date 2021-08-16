use uuid::Uuid;

use crate::proto::NodeDesc;

pub struct Node {
    desc: NodeDesc,
}

impl Node {
    pub fn new() -> Node {
        let desc = NodeDesc {
            uuid: Uuid::new_v4().to_string(),
        };
        Node { desc }
    }

    pub fn get_uuid(&self) -> String {
        self.desc.uuid.clone()
    }

    pub fn get_desc(&self) -> NodeDesc {
        self.desc.clone()
    }
}
