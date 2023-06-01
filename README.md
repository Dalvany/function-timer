[![Crate](https://img.shields.io/crates/v/function-timer.svg)](https://crates.io/crates/function-timer)
[![Build Status](https://github.com/Dalvany/function-timer/actions/workflows/rust.yml/badge.svg)](https://github.com/Dalvany/function-timer/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Dalvany/function-timer/branch/main/graph/badge.svg)](https://codecov.io/gh/Dalvany/function-timer)
[![dependency status](https://deps.rs/repo/github/Dalvany/function-timer/status.svg)](https://deps.rs/repo/github/Dalvany/function-timer)
[![Documentation](https://docs.rs/function-timer/badge.svg)](https://docs.rs/function-timer/)
[![Crate](https://img.shields.io/crates/d/function-timer.svg)](https://crates.io/crates/function-timer)
[![Crate](https://img.shields.io/crates/l/function-timer.svg)](https://crates.io/crates/function-timer)

# Function timer

Macro that allows to time a function and emit a histogram metric
using [metrics](https://crates.io/crates/metrics) crate.

Note: with the use of another attribut macro, declaration order might matter.
Especially using [async-trait](https://crates.io/crates/async-trait), depending on which one is first, you
time the actual execution of the function if time macro is declared before, or the creation of the future if
it's declared after.

## Example

```rust
use std::error::Error;
use metrics_exporter_prometheus::PrometheusBuilder;
use function_timer::time;

struct Test {}

impl Test {
    #[time("my_metric")]
    pub fn impl_function(&self) {
        println!("This another test");
    }

    #[time("another_metric")]
    pub fn impl_fail_function(&self, text:&str) -> Result<(), Box<dyn Error>>{
        let number:usize = text.parse()?;
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
```

Output :
```
This a test
This another test
This another test
1
# TYPE another_metric summary
another_metric{function="impl_fail_function",quantile="0"} 0.000000677
another_metric{function="impl_fail_function",quantile="0.5"} 0.0000006770639874327633
another_metric{function="impl_fail_function",quantile="0.9"} 0.0000006770639874327633
another_metric{function="impl_fail_function",quantile="0.95"} 0.0000006770639874327633
another_metric{function="impl_fail_function",quantile="0.99"} 0.0000006770639874327633
another_metric{function="impl_fail_function",quantile="0.999"} 0.0000006770639874327633
another_metric{function="impl_fail_function",quantile="1"} 0.000012062
another_metric_sum{function="impl_fail_function"} 0.000012739000000000001
another_metric_count{function="impl_fail_function"} 2

# TYPE my_metric summary
my_metric{function="free_function",quantile="0"} 0.000005702
my_metric{function="free_function",quantile="0.5"} 0.000005701963063845405
my_metric{function="free_function",quantile="0.9"} 0.000005701963063845405
my_metric{function="free_function",quantile="0.95"} 0.000005701963063845405
my_metric{function="free_function",quantile="0.99"} 0.000005701963063845405
my_metric{function="free_function",quantile="0.999"} 0.000005701963063845405
my_metric{function="free_function",quantile="1"} 0.000005702
my_metric_sum{function="free_function"} 0.000005702
my_metric_count{function="free_function"} 1
my_metric{function="impl_function",quantile="0"} 0.000002602
my_metric{function="impl_function",quantile="0.5"} 0.0000026018182046361393
my_metric{function="impl_function",quantile="0.9"} 0.0000026018182046361393
my_metric{function="impl_function",quantile="0.95"} 0.0000026018182046361393
my_metric{function="impl_function",quantile="0.99"} 0.0000026018182046361393
my_metric{function="impl_function",quantile="0.999"} 0.0000026018182046361393
my_metric{function="impl_function",quantile="1"} 0.000002602
my_metric_sum{function="impl_function"} 0.000002602
my_metric_count{function="impl_function"} 1
my_metric{function="static_function",quantile="0"} 0.000002894
my_metric{function="static_function",quantile="0.5"} 0.0000028939157344447597
my_metric{function="static_function",quantile="0.9"} 0.0000028939157344447597
my_metric{function="static_function",quantile="0.95"} 0.0000028939157344447597
my_metric{function="static_function",quantile="0.99"} 0.0000028939157344447597
my_metric{function="static_function",quantile="0.999"} 0.0000028939157344447597
my_metric{function="static_function",quantile="1"} 0.000002894
my_metric_sum{function="static_function"} 0.000002894
my_metric_count{function="static_function"} 1
```

It can also be put on an `impl` block :

```rust
use std::error::Error;
use metrics_exporter_prometheus::PrometheusBuilder;
use function_timer::time;

struct Test {}

#[time("my_metric")]
impl Test {
    pub fn impl_function(&self) {
        println!("This another test");
    }

    pub fn impl_fail_function(&self, text:&str) -> Result<(), Box<dyn Error>>{
        let number:usize = text.parse()?;
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
```

It will output :

```
This another test
This another test
1
# TYPE my_metric summary
my_metric{struct="Test",function="static_function",quantile="0"} 0.000005976
my_metric{struct="Test",function="static_function",quantile="0.5"} 0.000005976352983111928
my_metric{struct="Test",function="static_function",quantile="0.9"} 0.000005976352983111928
my_metric{struct="Test",function="static_function",quantile="0.95"} 0.000005976352983111928
my_metric{struct="Test",function="static_function",quantile="0.99"} 0.000005976352983111928
my_metric{struct="Test",function="static_function",quantile="0.999"} 0.000005976352983111928
my_metric{struct="Test",function="static_function",quantile="1"} 0.000005976
my_metric_sum{struct="Test",function="static_function"} 0.000005976
my_metric_count{struct="Test",function="static_function"} 1
my_metric{struct="Test",function="impl_fail_function",quantile="0"} 0.000000771
my_metric{struct="Test",function="impl_fail_function",quantile="0.5"} 0.0000007710596865495025
my_metric{struct="Test",function="impl_fail_function",quantile="0.9"} 0.0000007710596865495025
my_metric{struct="Test",function="impl_fail_function",quantile="0.95"} 0.0000007710596865495025
my_metric{struct="Test",function="impl_fail_function",quantile="0.99"} 0.0000007710596865495025
my_metric{struct="Test",function="impl_fail_function",quantile="0.999"} 0.0000007710596865495025
my_metric{struct="Test",function="impl_fail_function",quantile="1"} 0.00000257
my_metric_sum{struct="Test",function="impl_fail_function"} 0.000003341
my_metric_count{struct="Test",function="impl_fail_function"} 2
my_metric{struct="Test",function="impl_function",quantile="0"} 0.000003853
my_metric{struct="Test",function="impl_function",quantile="0.5"} 0.000003852839894857494
my_metric{struct="Test",function="impl_function",quantile="0.9"} 0.000003852839894857494
my_metric{struct="Test",function="impl_function",quantile="0.95"} 0.000003852839894857494
my_metric{struct="Test",function="impl_function",quantile="0.99"} 0.000003852839894857494
my_metric{struct="Test",function="impl_function",quantile="0.999"} 0.000003852839894857494
my_metric{struct="Test",function="impl_function",quantile="1"} 0.000003853
my_metric_sum{struct="Test",function="impl_function"} 0.000003853
my_metric_count{struct="Test",function="impl_function"} 1
```