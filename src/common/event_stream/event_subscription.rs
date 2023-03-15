use serde::{Deserialize, Serialize};

use super::{listeners::listeners::Listeners, event::EventType};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct EventSubscription{
    pub event_type: EventType,
    pub listeners: Listeners
}