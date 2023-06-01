use function_timer::time;
use metrics::Label;
use metrics_util::debugging::{DebugValue, Snapshotter};
use metrics_util::MetricKind;
use std::error::Error;
use std::time::Duration;

struct Test {}

#[time("my_metric")]
impl Test {
    pub async fn impl_function(&self) {
        std::thread::sleep(Duration::from_secs(2));
    }

    #[time("other_metric")]
    pub async fn static_function() {
        std::thread::sleep(Duration::from_secs(2));
    }

    pub async fn impl_fail_function(&self, text: &str) -> Result<(), Box<dyn Error>> {
        std::thread::sleep(Duration::from_secs(2));
        let number: usize = text.parse()?;
        println!("{number}");

        Ok(())
    }
}

#[futures_test::test]
async fn test_time_static_function() {
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread().install();

    Test::static_function().await;

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "other_metric");
        assert_eq!(labels, vec![Label::new("function", "static_function")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}

#[futures_test::test]
async fn test_time_impl_function() {
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread().install();

    let t = Test {};
    t.impl_function().await;

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "my_metric");
        assert_eq!(
            labels,
            vec![
                Label::new("struct", "Test"),
                Label::new("function", "impl_function")
            ]
        );
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}

#[futures_test::test]
async fn test_time_impl_fail_function() {
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread().install();

    let t = Test {};
    let _ = t.impl_fail_function("azerty").await;

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "my_metric");
        assert_eq!(
            labels,
            vec![
                Label::new("struct", "Test"),
                Label::new("function", "impl_fail_function")
            ]
        );
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}
