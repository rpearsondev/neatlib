
use serde::{Deserialize, Serialize};
use bitflags::bitflags;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Listeners: u32{
        const CONSOLE = 1;
        const SQL = 2;
    }
}