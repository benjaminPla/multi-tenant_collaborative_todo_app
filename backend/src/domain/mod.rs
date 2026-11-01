pub mod hash;
pub mod users;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
