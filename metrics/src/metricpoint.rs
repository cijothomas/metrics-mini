use std::sync::{atomic::AtomicU64, Arc};

#[derive(Clone)]
pub struct MetricPoint {
    inner: Arc<MetricPointInner>,
}

impl MetricPoint {
    pub fn new() -> MetricPoint {
        MetricPoint {
            inner: Arc::new(MetricPointInner::new()),
        }
    }

    pub fn add(&self, value: u32) {
        self.inner
            .sum
            .fetch_add(value as u64, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_sum(&self) -> u32 {
        self.inner.sum.load(std::sync::atomic::Ordering::Relaxed) as u32
    }
}

pub struct MetricPointInner {
    sum: AtomicU64,
}

impl MetricPointInner {
    fn new() -> MetricPointInner {
        MetricPointInner {
            sum: AtomicU64::new(1),
        }
    }
}
