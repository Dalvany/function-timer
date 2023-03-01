use std::error::Error;
use metrics_exporter_prometheus::PrometheusBuilder;
use function_timer::time;

struct Test {}

impl Test {
    #[time("my_metric")]
    pub fn impl_function(&self) {
        println!("This another test");
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
    free_function();
    free_function();

    Test::static_function();

    let t = Test {};
    t.impl_function();
    t.impl_function();

    println!("{}", handle.render());

    Ok(())
}
