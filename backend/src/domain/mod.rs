pub mod hash;
pub mod sessions;
pub mod teams;
pub mod users;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
