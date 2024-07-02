use crate::common::KeyValue;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(PartialEq, Eq, Clone)]
pub struct MetricAttributes {
    pub attributes: Vec<KeyValue>,
    hash_value: u64,
}

impl MetricAttributes {
    pub fn new(attributes: &[KeyValue]) -> MetricAttributes {
        let attributes_vec = attributes.to_vec();
        let hash_value = calculate_hash(&attributes_vec);
        MetricAttributes {
            attributes: attributes_vec,
            hash_value: hash_value,
        }
    }

    pub fn new_from_vec(attributes: Vec<KeyValue>) -> MetricAttributes {
        let hash_value = calculate_hash(&attributes);
        MetricAttributes {
            attributes,
            hash_value: hash_value,
        }
    }
}

impl Hash for MetricAttributes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_value)
    }
}

fn calculate_hash(values: &[KeyValue]) -> u64 {
    let mut hasher = DefaultHasher::new();
    values.iter().fold(&mut hasher, |mut hasher, item| {
        item.hash(&mut hasher);
        hasher
    });
    hasher.finish()
}
