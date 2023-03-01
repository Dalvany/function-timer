//! This crate allow to put a `time` attribut macro on any function.
//! It will time the execution of the function and emit a histogram
//! metric using [metrics](https://crates.io/crates/metrics) crate.
//!
//! # Example
//!
//! ```rust
//! use std::error::Error;
//! use metrics_exporter_prometheus::PrometheusBuilder;
//! use function_timer::time;
//!
//! struct Test {}
//!
//! impl Test {
//!     #[time("my_metric")]
//!     pub fn impl_function(&self) {
//!         println!("This another test");
//!     }
//!
//!     #[time("another_metric")]
//!     pub fn impl_fail_function(&self, text:&str) -> Result<(), Box<dyn Error>>{
//!         let number:usize = text.parse()?;
//!         println!("{number}");
//!
//!         Ok(())
//!     }
//!
//!     #[time("my_metric")]
//!     pub fn static_function() {
//!         println!("This another test");
//!     }
//! }
//!
//! #[time("my_metric")]
//! pub fn free_function() {
//!     println!("This a test");
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let builder = PrometheusBuilder::new();
//!     let handle = builder.install_recorder()?;
//!
//!     free_function();
//!
//!     Test::static_function();
//!
//!     let t = Test {};
//!     t.impl_function();
//!
//!     let result = t.impl_fail_function("azerty");
//!     assert!(result.is_err());
//!     let result = t.impl_fail_function("1");
//!     assert!(result.is_ok());
//!
//!
//!     println!("{}", handle.render());
//!
//!     Ok(())
//! }
//! ```
//!
//! # Note
//!
//! If time is put on functions that have the same names, the only way to distinguish between them
//! is to not use the same metric name. Plan is to allow custom tag on annotation and/or allow to
//! put `time` on an `impl` block.
pub use function_timer_macro::time;
use metrics::histogram;
use std::time::Instant;

/// Timer.
pub struct FunctionTimer {
    metric_name: String,
    function: String,
    chrono: Instant,
}

impl FunctionTimer {
    /// Create a new [FunctionTimer].
    ///
    /// # Parameters
    ///
    /// * `metric_name` : name of the metric.
    /// * `function` : name of the function that have the annotation. It is used to generate
    /// the tag `function`.
    pub fn new(metric_name: String, function: String) -> Self {
        Self {
            metric_name,
            function,
            chrono: Instant::now(),
        }
    }
}

impl Drop for FunctionTimer {
    /// Get execution time and call [`histogram!`](histogram).
    fn drop(&mut self) {
        let d = self.chrono.elapsed();
        histogram!(self.metric_name.clone(), d, "function" => self.function.clone());
    }
}
