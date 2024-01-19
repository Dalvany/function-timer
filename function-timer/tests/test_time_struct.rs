use function_timer::time;
use metrics::Label;
use metrics_util::debugging::DebugValue;
use metrics_util::MetricKind;
use std::error::Error;
use std::time::Duration;

struct Test {}

#[time("my_metric")]
impl Test {
    pub fn impl_function(&self) {
        std::thread::sleep(Duration::from_secs(2));
    }

    #[time("other_metric")]
    pub fn static_function() {
        std::thread::sleep(Duration::from_secs(2));
    }

    pub fn impl_fail_function(&self, text: &str) -> Result<(), Box<dyn Error>> {
        std::thread::sleep(Duration::from_secs(2));
        let number: usize = text.parse()?;
        println!("{number}");

        Ok(())
    }
}

trait MyTrait {
    type Output;

    fn trait_function(&self) -> Self::Output;
}

#[time("trait_metric")]
impl MyTrait for Test {
    type Output = &'static str;

    fn trait_function(&self) -> Self::Output {
        "test"
    }
}

#[test]
fn test_time_static_function() {
    let recorder = metrics_util::debugging::DebuggingRecorder::new();

    metrics::with_local_recorder(&recorder, || {
        Test::static_function();
    });

    let metrics = recorder.snapshotter().snapshot().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "other_metric");
        assert_eq!(labels, vec![Label::new("function", "static_function")]);
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}

#[test]
fn test_time_impl_function() {
    let recorder = metrics_util::debugging::DebuggingRecorder::new();

    metrics::with_local_recorder(&recorder, || {
        let t = Test {};
        t.impl_function();
    });

    let metrics = recorder.snapshotter().snapshot().into_vec();

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

#[test]
fn test_time_impl_fail_function() {
    let recorder = metrics_util::debugging::DebuggingRecorder::new();

    metrics::with_local_recorder(&recorder, || {
        let t = Test {};
        let _ = t.impl_fail_function("azerty");
    });

    let metrics = recorder.snapshotter().snapshot().into_vec();

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

#[test]
fn test_time_impl_trait() {
    let recorder = metrics_util::debugging::DebuggingRecorder::new();

    metrics::with_local_recorder(&recorder, || {
        let t = Test {};
        let _ = t.trait_function();
    });

    let metrics = recorder.snapshotter().snapshot().into_vec();

    for (key, _, _, debug_value) in metrics {
        let (kind, key) = key.into_parts();
        let (name, labels) = key.into_parts();
        assert_eq!(kind, MetricKind::Histogram);
        assert_eq!(name.as_str(), "trait_metric");
        assert_eq!(
            labels,
            vec![
                Label::new("struct", "Test"),
                Label::new("function", "trait_function")
            ]
        );
        assert!(matches!(debug_value, DebugValue::Histogram(_)));
    }
}
