//! This crate allow to put a `time` attribut macro on any function
//! or `impl` block.
//! It will time the execution of functions and emit a histogram
//! metric using [metrics](https://crates.io/crates/metrics) crate.
//!
//! In case the annotation is on an `impl` block :
//! * all method will be timed
//! * there will be a tag `struct` with the struct name.
//! * all `time` annotations on any method will override the one on `impl` block.
//! * it's possible to disable specific methods using `#[time(disable)]`.
//!
//! Note that `#[time(disable)]` can't be on an `impl` block.
//!
//! # Example
//!
//! * On functions and methods :
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
//! * on `impl` block :
//!
//! ```rust
//! use std::error::Error;
//! use metrics_exporter_prometheus::PrometheusBuilder;
//! use function_timer::time;
//!
//! struct Test {}
//!
//! #[time("my_metric")]
//! impl Test {
//!     #[time("override_my_metric")]
//!     pub fn impl_function(&self) {
//!         println!("This another test");
//!     }
//!
//!     pub fn impl_fail_function(&self, text:&str) -> Result<(), Box<dyn Error>>{
//!         let number:usize = text.parse()?;
//!         println!("{number}");
//!
//!         Ok(())
//!     }
//!
//!     pub fn static_function() {
//!         println!("This another test");
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let builder = PrometheusBuilder::new();
//!     let handle = builder.install_recorder()?;
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
pub use function_timer_macro::time;
use metrics::histogram;
use std::time::Instant;

/// Timer.
pub struct FunctionTimer {
    metric_name: &'static str,
    struct_name: Option<&'static str>,
    function: &'static str,
    chrono: Instant,
}

impl FunctionTimer {
    /// Create a new [FunctionTimer].
    ///
    /// # Parameters
    ///
    /// * `metric_name` : name of the metric.
    /// * `struct_name` : name of the struct.
    /// * `function` : name of the function that have the annotation. It is used to generate
    /// the tag `function`.
    pub fn new(
        metric_name: &'static str,
        struct_name: Option<&'static str>,
        function: &'static str,
    ) -> Self {
        Self {
            metric_name,
            struct_name,
            function,
            chrono: Instant::now(),
        }
    }
}

impl Drop for FunctionTimer {
    /// Get execution time and call [`histogram!`](histogram).
    fn drop(&mut self) {
        let d = self.chrono.elapsed();
        if let Some(struct_name) = self.struct_name {
            histogram!(self.metric_name, d, "struct" => struct_name, "function" => self.function);
        } else {
            histogram!(self.metric_name, d, "function" => self.function);
        }
    }
}
