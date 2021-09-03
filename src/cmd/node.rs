use std::sync::Arc;

use clap::Clap;
use tonic::transport::Server;

use super::{Result, DEFAULT_URL};
use crate::node::Node;
use crate::proto::{ListNodesRequest, NodeClient};
use crate::service::{NodeService, UnitService};

const DEFAULT_ADDR: &str = "127.0.0.1:21812";

#[derive(Clap)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Command {
    #[tokio::main(flavor = "current_thread")]
    pub async fn run(&self, node: Node) {
        let result = match &self.subcmd {
            SubCommand::Init(cmd) => cmd.run(node).await,
            SubCommand::List(cmd) => cmd.run().await,
        };
        result.unwrap();
    }
}

#[derive(Clap)]
enum SubCommand {
    Init(InitCommand),
    List(ListCommand),
}

#[derive(Clap)]
struct InitCommand {}

impl InitCommand {
    async fn run(&self, node: Node) -> Result<()> {
        let node = Arc::new(node);
        let addr = DEFAULT_ADDR.parse()?;
        println!("node {} listen on {}", node.uuid(), addr);
        let node_service = NodeService::new(node.clone());
        let unit_service = UnitService::new(node.clone());
        Server::builder()
            .add_service(node_service.into_server())
            .add_service(unit_service.into_server())
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[derive(Clap)]
struct ListCommand {
    #[clap(default_value = DEFAULT_URL)]
    url: String,
}

impl ListCommand {
    async fn run(&self) -> Result<()> {
        let mut cli = NodeClient::connect(self.url.clone()).await?;
        let req = ListNodesRequest {};
        let res = cli.list_nodes(req).await?;
        println!("{:#?}", res.into_inner().descs);
        Ok(())
    }
}
