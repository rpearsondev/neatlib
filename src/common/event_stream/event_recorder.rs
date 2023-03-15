use crate::neat::trainer::configuration::Configuration;
use super::{event::{Event, EventType}, listeners::{listeners::Listeners, console_listener::ConsoleEventListener, sql_listener::SqlRepositoryEventListener}};

pub struct EventRecorder;

impl EventRecorder{
    pub fn record_event(config: &Configuration, event: &Event){
        for subscription in &config.event_subscriptions{
            if subscription.event_type & event.event_type == event.event_type{
                if subscription.listeners & Listeners::CONSOLE == Listeners::CONSOLE{
                    ConsoleEventListener::process(event);
                }
                if subscription.listeners & Listeners::SQL == Listeners::SQL{
                    SqlRepositoryEventListener::process(event);
                }
            }
        }
    }
    pub fn has_subscription(config: &Configuration, event_type: EventType) -> bool{
        for subscription in &config.event_subscriptions{
            if subscription.event_type & event_type == event_type{
                return true;
            }
        }
        false
    }
}