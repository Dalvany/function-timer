use std::time::Duration;

use function_timer::time;
use metrics::Label;
use metrics_util::debugging::DebugValue;
use metrics_util::MetricKind;

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
    let recorder = metrics_util::debugging::DebuggingRecorder::new();

    metrics::with_local_recorder(&recorder, || {
        let test = Test {};

        test.disable();
        test.test();
    });

    let metrics = recorder.snapshotter().snapshot().into_vec();

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
