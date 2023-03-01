use std::error::Error;
use std::time::Duration;
use metrics::Label;
use function_timer::time;
use metrics_util::debugging::{DebugValue, Snapshotter};
use metrics_util::MetricKind;

struct Test {}

impl Test {
    #[time("my_metric")]
    pub fn impl_function(&self) {
        std::thread::sleep(Duration::from_secs(2));
    }

    #[time("my_metric")]
    pub fn static_function() {
        std::thread::sleep(Duration::from_secs(2));
    }

    #[time("another_metric")]
    pub fn impl_fail_function(&self, text:&str) -> Result<(), Box<dyn Error>>{
        std::thread::sleep(Duration::from_secs(2));
        let number:usize = text.parse()?;
        println!("{number}");

        Ok(())
    }
}

#[time("my_metric")]
pub fn free_function() {
    std::thread::sleep(Duration::from_secs(2));
}

#[test]
fn test_time_free_function() -> Result<(), Box<dyn Error>>{
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread()
        .install();

    free_function();

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "my_metric");
        assert_eq!(labels, vec![Label::new("function", "free_function")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }

    Ok(())
}

#[test]
fn test_time_static_function() -> Result<(), Box<dyn Error>>{
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread()
        .install();

    Test::static_function();

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "my_metric");
        assert_eq!(labels, vec![Label::new("function", "static_function")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }

    Ok(())
}

#[test]
fn test_time_impl_function() -> Result<(), Box<dyn Error>>{
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread()
        .install();

    let t = Test{};
    t.impl_function();

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "my_metric");
        assert_eq!(labels, vec![Label::new("function", "impl_function")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));

    }

    Ok(())
}

#[test]
fn test_time_impl_fail_function() -> Result<(), Box<dyn Error>>{
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread()
        .install();

    let t = Test{};
    let _ = t.impl_fail_function("azerty");

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "another_metric");
        assert_eq!(labels, vec![Label::new("function", "impl_fail_function")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));

    }

    Ok(())
}
