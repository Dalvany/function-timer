use std::error::Error;

use function_timer::time;
use metrics_exporter_prometheus::PrometheusBuilder;

struct Test {}

impl Test {
    #[time("my_metric")]
    pub fn impl_function(&self) {
        println!("This another test");
    }

    #[time("another_metric")]
    pub fn impl_fail_function(&self, text: &str) -> Result<(), Box<dyn Error>> {
        let number: usize = text.parse()?;
        println!("{number}");

        Ok(())
    }

    #[time("my_metric")]
    pub fn static_function() {
        println!("This another test");
    }
}

#[time("my_metric")]
pub fn free_function() {
    println!("This a test");
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder()?;

    free_function();

    Test::static_function();

    let t = Test {};
    t.impl_function();

    let result = t.impl_fail_function("azerty");
    assert!(result.is_err());
    let result = t.impl_fail_function("1");
    assert!(result.is_ok());

    println!("{}", handle.render());

    Ok(())
}
