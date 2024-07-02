use std::sync::Arc;
use std::vec;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, RwLock},
};

use crate::attributes::MetricAttributes;
use crate::common::KeyValue;
use crate::metric::Metric;
use crate::metricpoint::MetricPoint;

#[derive(Clone)]
pub struct Counter {
    inner: Arc<CounterInner>,
}

impl Counter {
    pub fn new(name: String) -> Counter {
        Counter {
            inner: Arc::new(CounterInner::new(name)),
        }
    }

    pub fn add(&self, value: u32, attributes: &[KeyValue]) {
        self.inner.add(value, attributes);
    }

    pub fn display_metrics(&self) {
        self.inner.display_metrics();
    }

    pub fn collect(&self) -> Metric {
        self.inner.collect()
    }
}

pub struct CounterInner {
    metric_points_map: RwLock<HashMap<MetricAttributes, MetricPoint>>,
    zero_attribute_point: AtomicUsize,
    name: String,
}

impl CounterInner {
    pub fn new(name: String) -> CounterInner {
        let counter = CounterInner {
            metric_points_map: RwLock::new(HashMap::new()),
            zero_attribute_point: AtomicUsize::new(0),
            name: name,
        };
        counter
    }

    pub fn collect(&self) -> Metric {
        let mut metric_points: Vec<(Vec<KeyValue>, u32)> = Vec::new();

        for kv in self.metric_points_map.write().unwrap().drain() {
            metric_points.push((kv.0.attributes.clone(), kv.1.get_sum()));
        }

        metric_points.push((
            vec![],
            self.zero_attribute_point
                .load(std::sync::atomic::Ordering::Relaxed) as u32,
        ));

        let metric = Metric::new(self.name.clone(), metric_points);

        self.zero_attribute_point
            .store(0, std::sync::atomic::Ordering::Relaxed);

        metric
    }

    pub fn add(&self, value: u32, attributes: &[KeyValue]) {
        if attributes.is_empty() {
            self.zero_attribute_point
                .fetch_add(value as usize, std::sync::atomic::Ordering::Relaxed);
            return;
        }

        let metric_attributes = MetricAttributes::new(attributes);
        let metric_points_map = self.metric_points_map.read().unwrap();
        if let Some(metric_point) = metric_points_map.get(&metric_attributes) {
            metric_point.add(value);
        } else {
            drop(metric_points_map);
            // TODO: De-dup keys.
            let mut metric_points_map = self.metric_points_map.write().unwrap();
            // sort and try again
            let mut attributes_as_vec = attributes.to_vec();
            attributes_as_vec.sort_by(|a, b| a.key.cmp(&b.key));
            let metric_attributes_sorted = MetricAttributes::new_from_vec(attributes_as_vec);

            if let Some(metric_point) = metric_points_map.get(&metric_attributes_sorted) {
                metric_point.add(value);
            } else {
                // insert both incoming order and sorted order
                // insert in incoming order.
                let mp_new = MetricPoint::new();
                mp_new.add(value);
                metric_points_map.insert(metric_attributes, mp_new.clone());

                // insert in sorted order
                metric_points_map.insert(metric_attributes_sorted.clone(), mp_new);
            }
        }
    }

    pub fn display_metrics(&self) {
        println!("Metrics:");
        let metric_points_map = self.metric_points_map.read().unwrap();
        for metric_point in metric_points_map.iter() {
            println!(
                "Attributes: {:?} Sum: {}",
                metric_point.0.attributes,
                metric_point.1.get_sum(),
            );
        }

        println!(
            "Zero attribute point: {}",
            self.zero_attribute_point
                .load(std::sync::atomic::Ordering::Relaxed)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
