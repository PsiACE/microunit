mod node;
mod unit;

pub use node::Command as NodeCommand;
pub use unit::Command as UnitCommand;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) const DEFAULT_URL: &'static str = "http://127.0.0.1:21812";
