use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{counter::Counter, metric::Metric};

#[derive(Clone)]
pub struct Meter {
    inner: Arc<MeterInner>,
}

impl Meter {
    pub fn new(name: &str) -> Meter {
        Meter {
            inner: Arc::new(MeterInner {
                name: name.to_string(),
                counters: Mutex::new(HashMap::new()),
            }),
        }
    }

    pub fn create_counter(&self, name: &str) -> Counter {
        self.inner.create_counter(name)
    }

    pub fn collect(&self) -> Vec<Metric> {
        let mut metrics = vec![];
        let counters = self.inner.counters.lock().unwrap();
        for counter in counters.values() {
            metrics.push(counter.collect());
        }

        metrics
    }
}

pub struct MeterInner {
    name: String,
    counters: Mutex<HashMap<String, Counter>>,
}

impl MeterInner {
    pub fn create_counter(&self, name: &str) -> Counter {
        let mut counters = self.counters.lock().unwrap();
        if let Some(counter) = counters.get(name) {
            counter.clone()
        } else {
            let counter = Counter::new(name.to_string());
            counters.insert(name.to_string(), counter.clone());
            counter
        }
    }
}
