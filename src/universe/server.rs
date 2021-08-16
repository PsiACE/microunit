use tonic::{Request, Response, Status};

use crate::node::Node;
use crate::proto::universe_server::{Universe, UniverseServer};
use crate::proto::{ListNodesRequest, ListNodesResponse};

pub struct Server {
    node: Node,
}

impl Server {
    pub fn new(node: Node) -> Server {
        Server { node }
    }

    pub fn into_service(self) -> UniverseServer<Server> {
        UniverseServer::new(self)
    }
}

#[tonic::async_trait]
impl Universe for Server {
    async fn list_nodes(
        &self,
        _: Request<ListNodesRequest>,
    ) -> Result<Response<ListNodesResponse>, Status> {
        let res = ListNodesResponse {
            descs: vec![self.node.get_desc()],
        };
        Ok(Response::new(res))
    }
}
