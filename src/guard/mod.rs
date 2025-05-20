mod error;
mod role;

pub use error::GuardError;
pub use role::{has_role, set_roles};

pub trait RouteGuard {
    fn check(&self) -> bool;
}