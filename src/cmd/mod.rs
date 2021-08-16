mod node;

pub use node::Command as NodeCommand;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
