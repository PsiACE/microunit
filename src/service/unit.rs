use std::sync::Arc;

use tonic::{Code, Request, Response, Status};

use crate::node::Node;
use crate::proto::unit_server::{Unit as UnitService, UnitServer};
use crate::proto::{CreateUnitRequest, CreateUnitResponse, ListUnitsRequest, ListUnitsResponse};

pub struct Service {
    node: Arc<Node>,
}

impl Service {
    pub fn new(node: Arc<Node>) -> Service {
        Service { node }
    }

    pub fn into_server(self) -> UnitServer<Service> {
        UnitServer::new(self)
    }
}

#[tonic::async_trait]
impl UnitService for Service {
    async fn list_units(
        &self,
        _: Request<ListUnitsRequest>,
    ) -> Result<Response<ListUnitsResponse>, Status> {
        let res = ListUnitsResponse {
            descs: self.node.list_units(),
        };
        Ok(Response::new(res))
    }

    async fn create_unit(
        &self,
        request: Request<CreateUnitRequest>,
    ) -> Result<Response<CreateUnitResponse>, Status> {
        let req = request.into_inner();
        if let Some(spec) = req.spec {
            match self.node.create_unit(&spec) {
                Ok(desc) => {
                    let res = CreateUnitResponse { desc: Some(desc) };
                    Ok(Response::new(res))
                }
                Err(err) => Err(Status::new(Code::InvalidArgument, err.to_string())),
            }
        } else {
            Err(Status::new(
                Code::InvalidArgument,
                "missing unit spec".to_owned(),
            ))
        }
    }
}
