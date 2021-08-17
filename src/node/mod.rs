mod error;
mod node;
mod node_builder;
mod unit;

pub use error::NodeError;
pub use node::Node;
pub use node_builder::NodeBuilder;
pub use unit::{Unit, UnitHandle};

pub(crate) type Result<T> = std::result::Result<T, NodeError>;
