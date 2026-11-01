pub mod hash;
pub mod sessions;
pub mod users;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
