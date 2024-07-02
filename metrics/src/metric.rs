use crate::common::KeyValue;

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub metric_points: Vec<(Vec<KeyValue>, u32)>,
}
impl Metric {
    pub(crate) fn new(name: String, points: Vec<(Vec<KeyValue>, u32)>) -> Self {
        Self {
            name: name,
            metric_points: points,
        }
    }
}
