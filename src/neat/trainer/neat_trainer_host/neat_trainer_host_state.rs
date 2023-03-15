#[derive(Debug)]
pub struct NeatTrainerHostState{
    pub is_sleeping: bool,
    pub is_stopping: bool
}

impl NeatTrainerHostState {
    pub fn new() -> Self{
        Self {
            is_sleeping: true,
            is_stopping: false
        }
    }
}