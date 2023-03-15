#[derive(Clone)]
pub struct DistributedConfig{
    pub host_thread_stack_size: usize,
    pub host_worker_threads: usize,
    pub host_max_blocking_threads: usize,
    pub host_buffer_unordered: usize,
    pub client_thread_stack_size: usize,
    pub client_worker_threads: usize,
    pub client_max_blocking_threads: usize,
    pub client_sleep_when_connection_dead_milliseconds: u64,
    pub client_sleep_when_get_work_failed_milliseconds: u64,
    pub client_sleep_when_no_config_milliseconds: u64,
    pub client_sleep_when_no_work_milliseconds_high: usize,
    pub client_sleep_when_no_work_milliseconds_low: usize,
    pub client_sleep_when_no_work_after_100_attempts_milliseconds: u64,
    pub client_work_batch_size: usize
}

impl DistributedConfig{
    pub fn new() -> Self{
        Self{
            host_thread_stack_size: 8 * 1024 * 1024,
            host_worker_threads: 30,
            host_max_blocking_threads: 15,
            host_buffer_unordered: 100,
            client_thread_stack_size: 8 * 1024 * 1024,
            client_worker_threads: 1,
            client_max_blocking_threads: 3,
            client_sleep_when_connection_dead_milliseconds: 3000,
            client_sleep_when_get_work_failed_milliseconds: 300,
            client_sleep_when_no_config_milliseconds: 3000,
            client_sleep_when_no_work_milliseconds_high: 100,
            client_sleep_when_no_work_milliseconds_low: 80,
            client_sleep_when_no_work_after_100_attempts_milliseconds: 3000,
            client_work_batch_size: 100
        }
    }
}