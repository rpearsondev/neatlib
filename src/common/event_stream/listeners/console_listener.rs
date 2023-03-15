use crate::common::event_stream::event::Event;

pub struct ConsoleEventListener;

impl ConsoleEventListener{
    pub fn process(event: &Event){
        println!("{:?}", event)
    }
}