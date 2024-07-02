use criterion::{criterion_group, criterion_main, Criterion};
use metrics::{common::KeyValue, meter_provider::MeterProvider};

// cargo bench --bench counter
pub fn counter_benchmark(c: &mut Criterion) {
    let meter_provider = MeterProvider::new_with_periodic_flush();
    let meter = meter_provider.get_meter("meter");
    let counter = meter.create_counter("counter-name");

    let attributes1 = [KeyValue::new("key1", "value1")];

    let attributes3 = [
        KeyValue::new("key1", "value1"),
        KeyValue::new("key2", "value2"),
        KeyValue::new("key3", "value3"),
    ];

    let attributes5 = [
        KeyValue::new("key1", "value1"),
        KeyValue::new("key2", "value2"),
        KeyValue::new("key3", "value3"),
        KeyValue::new("key4", "value4"),
        KeyValue::new("key5", "value5"),
    ];

    c.bench_function("counter_0", |b| {
        b.iter(|| {
            counter.add(10, &[]);
        });
    });

    c.bench_function("counter_1", |b| {
        b.iter(|| {
            counter.add(10, &attributes1);
        });
    });

    c.bench_function("counter_3", |b| {
        b.iter(|| {
            counter.add(10, &attributes3);
        });
    });

    c.bench_function("counter_5", |b| {
        b.iter(|| {
            counter.add(10, &attributes5);
        });
    });
}

criterion_group!(benches, counter_benchmark);
criterion_main!(benches);
