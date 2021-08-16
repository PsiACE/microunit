use clap::Clap;
use tonic::transport::Server;

use super::Result;
use crate::node::Node;
use crate::proto::universe_client::UniverseClient;
use crate::proto::ListNodesRequest;
use crate::universe::UniverseServer;

const DEFAULT_ADDR: &'static str = "127.0.0.1:21812";
const DEFAULT_URL: &'static str = "http://127.0.0.1:21812";

#[derive(Clap)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Command {
    #[tokio::main(flavor = "current_thread")]
    pub async fn run(&self) {
        let result = match &self.subcmd {
            SubCommand::Init(cmd) => cmd.run().await,
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
    async fn run(&self) -> Result<()> {
        let addr = DEFAULT_ADDR.parse()?;
        let node = Node::new();
        println!("node {} listen on {}", node.get_uuid(), addr);
        let server = UniverseServer::new(node);
        Server::builder()
            .add_service(server.into_service())
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
        let mut cli = UniverseClient::connect(self.url.clone()).await?;
        let req = ListNodesRequest {};
        let res = cli.list_nodes(req).await?;
        println!("{:#?}", res.into_inner().descs);
        Ok(())
    }
}
