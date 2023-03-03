use function_timer::time;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::error::Error;

struct Test {}

#[time("my_metric")]
impl Test {
    pub fn impl_function(&self) {
        println!("This another test");
    }

    pub fn impl_fail_function(&self, text: &str) -> Result<(), Box<dyn Error>> {
        let number: usize = text.parse()?;
        println!("{number}");

        Ok(())
    }

    pub fn static_function() {
        println!("This another test");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder()?;

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
