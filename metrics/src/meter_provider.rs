use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::meter::Meter;

pub struct MeterProvider {
    inner: Arc<MeterProviderInner>,
}

impl MeterProvider {
    pub fn new() -> MeterProvider {
        MeterProvider {
            inner: Arc::new(MeterProviderInner::new()),
        }
    }

    pub fn get_meter(&self, name: &str) -> Meter {
        self.inner.get_meter(name)
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
}
