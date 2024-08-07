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
        self.inner.add(value);
    }

    pub fn get_sum(&self) -> u32 {
        self.inner.get_sum()
    }

    pub fn reset(&self) {
        self.inner.reset();
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

    fn get_sum(&self) -> u32 {
        self.sum.load(std::sync::atomic::Ordering::Relaxed) as u32
    }

    fn add(&self, value: u32) {
        self.sum
            .fetch_add(value as u64, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.sum.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}
