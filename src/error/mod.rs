pub mod enums;
pub use enums::*;

pub mod messages;
pub use messages::*;

pub mod default_messages;
pub use default_messages::*;

pub type Result<T>    = std::result::Result<T, MglError>;
pub type TopResult<T> = std::result::Result<T, Vec<MglError>>;

