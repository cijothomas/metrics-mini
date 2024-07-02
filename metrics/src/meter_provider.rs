use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    vec,
};

use crate::{meter::Meter, metric::Metric};

#[derive(Clone)]
pub struct MeterProvider {
    inner: Arc<MeterProviderInner>,
}

impl MeterProvider {
    pub fn new() -> MeterProvider {
        MeterProvider {
            inner: Arc::new(MeterProviderInner::new()),
        }
    }

    pub fn new_with_periodic_flush() -> MeterProvider {
        let mp = MeterProvider {
            inner: Arc::new(MeterProviderInner::new()),
        };

        let mp_clone = mp.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
            let metrics = mp_clone.collect();
            for metric in metrics {
                println!("{:?}", metric);
            }
        });

        mp
    }

    pub fn get_meter(&self, name: &str) -> Meter {
        self.inner.get_meter(name)
    }

    pub fn collect(&self) -> Vec<Metric> {
        self.inner.collect()
    }
}

struct MeterProviderInner {
    meters: Mutex<HashMap<String, Meter>>,
}

impl MeterProviderInner {
    fn new() -> MeterProviderInner {
        MeterProviderInner {
            meters: Mutex::new(HashMap::new()),
        }
    }

    fn get_meter(&self, name: &str) -> Meter {
        let mut meters = self.meters.lock().unwrap();
        if let Some(meter) = meters.get(name) {
            meter.clone()
        } else {
            let meter = Meter::new(name);
            meters.insert(name.to_string(), meter.clone());
            meter
        }
    }

    pub fn collect(&self) -> Vec<Metric> {
        let mut metrics: Vec<Metric> = vec![];
        let meters = self.meters.lock().unwrap();
        for meter in meters.values() {
            metrics.append(meter.collect().as_mut());
        }

        metrics
    }
}
