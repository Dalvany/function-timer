use function_timer::time;
use metrics::Label;
use metrics_util::debugging::{DebugValue, Snapshotter};
use metrics_util::MetricKind;
use std::time::Duration;

struct Test {}

#[time("my_metric")]
impl Test {
    pub fn test(&self) {
        std::thread::sleep(Duration::from_secs(2));
    }

    #[time(disable)]
    pub fn disable(&self) {
        std::thread::sleep(Duration::from_secs(2));
    }
}

#[test]
fn test_time_disable() {
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread().install();

    let test = Test {};

    test.disable();
    test.test();

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
            vec![Label::new("struct", "Test"), Label::new("function", "test")]
        );
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}
