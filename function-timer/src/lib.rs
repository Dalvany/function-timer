pub use function_timer_macro::time;
use metrics::histogram;
use std::time::Instant;

pub struct FunctionTimer {
    metric_name: String,
    function: String,
    chrono: Instant,
}

impl FunctionTimer {
    pub fn new(metric_name: String, function: String) -> Self {
        Self {
            metric_name,
            function,
            chrono: Instant::now(),
        }
    }
}

impl Drop for FunctionTimer {
    fn drop(&mut self) {
        let d = self.chrono.elapsed();
        histogram!(self.metric_name.clone(), d, "function" => self.function.clone());
    }
}
