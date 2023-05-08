#[macro_use]
extern crate lazy_static;

pub mod neat;
pub mod cpu_phenome;
pub mod phenome;
pub mod common;
pub mod renderer;
pub use common::node_kind;
pub use common::activation_functions;
pub mod distributed_compute;
