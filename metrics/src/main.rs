use metrics::{common::KeyValue, meter_provider::MeterProvider};
use std::thread;

fn main() {
    let meter_provider = MeterProvider::new_with_periodic_flush();
    let meter = meter_provider.get_meter("meter");
    let counter = meter.create_counter("counter-name");
    let attributes = [
        KeyValue::new("key2", "value2"),
        KeyValue::new("key1", "value1"),
        KeyValue::new("key3", "value3"),
    ];
    let attributes_in_diff_order = [
        KeyValue::new("key1", "value1"),
        KeyValue::new("key2", "value2"),
        KeyValue::new("key3", "value3"),
    ];
    let attributes_in_diff_order2 = [
        KeyValue::new("key1", "value1"),
        KeyValue::new("key3", "value3"),
        KeyValue::new("key2", "value2"),
    ];

    println!("Metrics displayed every 10 secs. Press Ctrl+C to stop.");
    loop {
        counter.add(10, &attributes);
        counter.add(10, &attributes_in_diff_order);
        counter.add(10, &attributes_in_diff_order2);
        counter.add(10, &attributes);
        counter.add(10, &attributes_in_diff_order);

        counter.add(10, &[]);
        counter.add(10, &[]);
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
