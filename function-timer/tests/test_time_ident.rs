use function_timer::time;
use metrics::Label;
use metrics_util::debugging::DebugValue;
use metrics_util::MetricKind;

static METRIC_NAME: &str = "my_metric";
const OTHER_METRIC_NAME: &str = "other_metric";

struct Test {}

impl Test {
    #[time(METRIC_NAME)]
    fn test1(&self) {
        println!("test");
    }

    #[time(OTHER_METRIC_NAME)]
    fn test2(&self) {
        println!("test2");
    }
}

#[test]
fn test_ident_static() {
    let recorder = metrics_util::debugging::DebuggingRecorder::new();

    metrics::with_local_recorder(&recorder, || {
        let t = Test {};
        t.test1();
    });

    let metrics = recorder.snapshotter().snapshot().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "my_metric");
        assert_eq!(labels, vec![Label::new("function", "test1")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}

#[test]
fn test_ident_const() {
    let recorder = metrics_util::debugging::DebuggingRecorder::new();

    metrics::with_local_recorder(&recorder, || {
        let t = Test {};
        t.test2();
    });

    let metrics = recorder.snapshotter().snapshot().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "other_metric");
        assert_eq!(labels, vec![Label::new("function", "test2")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}
