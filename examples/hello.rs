use clap::Clap;

use microunit::cmd::{NodeCommand, UnitCommand};
use microunit::proto::UnitDesc;
use microunit::{NodeBuilder, Unit, UnitHandle};

pub struct HelloUnit {}

impl HelloUnit {
    pub fn new() -> HelloUnit {
        HelloUnit {}
    }
}

impl Unit for HelloUnit {
    fn kind(&self) -> String {
        "hello".to_owned()
    }

    fn spawn(&self, uuid: String) -> Box<dyn UnitHandle> {
        Box::new(HelloHandle::new(uuid))
    }
}

struct HelloHandle {
    unit: UnitDesc,
}

impl HelloHandle {
    fn new(uuid: String) -> HelloHandle {
        let desc = UnitDesc {
            uuid,
            kind: "hello".to_owned(),
        };
        HelloHandle { unit: desc }
    }
}

impl UnitHandle for HelloHandle {
    fn desc(&self) -> UnitDesc {
        self.unit.clone()
    }
}

#[derive(Clap)]
struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Command {
    pub fn run(&self) {
        let unit = HelloUnit::new();
        let node = NodeBuilder::new().add_unit(unit).build();
        match &self.subcmd {
            SubCommand::Node(cmd) => cmd.run(node),
            SubCommand::Unit(cmd) => cmd.run(),
        }
    }
}

#[derive(Clap)]
enum SubCommand {
    Node(NodeCommand),
    Unit(UnitCommand),
}

fn main() {
    let cmd: Command = Command::parse();
    cmd.run();
}
