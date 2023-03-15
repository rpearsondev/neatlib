#[macro_use]
extern crate lazy_static;

pub mod neat;
pub mod hyperneat;
pub mod phenome;
pub mod common;
pub mod renderer;
pub use common::node_kind;
pub use common::activation_functions;
pub mod distributed_compute;
