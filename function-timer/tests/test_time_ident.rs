use function_timer::time;
use metrics::Label;
use metrics_util::debugging::{DebugValue, Snapshotter};
use metrics_util::MetricKind;
use std::error::Error;

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
fn test_ident_static() -> Result<(), Box<dyn Error>> {
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread().install();

    let t = Test {};
    t.test1();

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "my_metric");
        assert_eq!(labels, vec![Label::new("function", "test1")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }

    Ok(())
}

#[test]
fn test_ident_const() -> Result<(), Box<dyn Error>> {
    let _ = metrics_util::debugging::DebuggingRecorder::per_thread().install();

    let t = Test {};
    t.test2();

    let snapshot = Snapshotter::current_thread_snapshot();
    assert!(snapshot.is_some(), "No snapshot");
    let metrics = snapshot.unwrap().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "other_metric");
        assert_eq!(labels, vec![Label::new("function", "test2")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }

    Ok(())
}
