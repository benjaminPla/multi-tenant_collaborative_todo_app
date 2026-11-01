pub mod authz;
pub mod boards;
pub mod hash;
pub mod sessions;
pub mod teams;
pub mod todos;
pub mod users;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
