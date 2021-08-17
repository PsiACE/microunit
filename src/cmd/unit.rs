use clap::Clap;

use super::{Result, DEFAULT_URL};
use crate::proto::{CreateUnitRequest, ListUnitsRequest, UnitClient, UnitSpec};

#[derive(Clap)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Command {
    #[tokio::main(flavor = "current_thread")]
    pub async fn run(&self) {
        let result = match &self.subcmd {
            SubCommand::List(cmd) => cmd.run().await,
            SubCommand::Create(cmd) => cmd.run().await,
        };
        result.unwrap();
    }
}

#[derive(Clap)]
enum SubCommand {
    List(ListCommand),
    Create(CreateCommand),
}

#[derive(Clap)]
struct ListCommand {
    #[clap(default_value = DEFAULT_URL)]
    url: String,
}

impl ListCommand {
    async fn run(&self) -> Result<()> {
        let mut cli = UnitClient::connect(self.url.clone()).await?;
        let req = ListUnitsRequest {};
        let res = cli.list_units(req).await?;
        println!("{:#?}", res.into_inner().descs);
        Ok(())
    }
}

#[derive(Clap)]
struct CreateCommand {
    #[clap(long, default_value = DEFAULT_URL)]
    url: String,
    #[clap(long)]
    kind: String,
}

impl CreateCommand {
    async fn run(&self) -> Result<()> {
        let mut cli = UnitClient::connect(self.url.clone()).await?;
        let spec = UnitSpec {
            kind: self.kind.clone(),
        };
        let req = CreateUnitRequest { spec: Some(spec) };
        let res = cli.create_unit(req).await?;
        let desc = res.into_inner().desc;
        println!("created unit {:#?}", desc);
        Ok(())
    }
}
