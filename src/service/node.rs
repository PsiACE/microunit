use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::node::Node;
use crate::proto::node_server::{Node as NodeService, NodeServer};
use crate::proto::{ListNodesRequest, ListNodesResponse};

pub struct Service {
    node: Arc<Node>,
}

impl Service {
    pub fn new(node: Arc<Node>) -> Service {
        Service { node }
    }

    pub fn into_server(self) -> NodeServer<Service> {
        NodeServer::new(self)
    }
}

#[tonic::async_trait]
impl NodeService for Service {
    async fn list_nodes(
        &self,
        _: Request<ListNodesRequest>,
    ) -> Result<Response<ListNodesResponse>, Status> {
        let res = ListNodesResponse {
            descs: self.node.list_nodes(),
        };
        Ok(Response::new(res))
    }
}
