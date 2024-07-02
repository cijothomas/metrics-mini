use crossterm::event::{self, Event, KeyCode};
use metrics::{common::KeyValue, counter::Counter};
use std::thread;

fn main() {
    let counter = Counter::new_with_periodic_flush();
    let attributes = [KeyValue::new("key2", "value2"), KeyValue::new("key1", "value1"), KeyValue::new("key3", "value3")];
    let attributes_in_diff_order = [KeyValue::new("key1", "value1"), KeyValue::new("key2", "value2"), KeyValue::new("key3", "value3")];
    let attributes_in_diff_order2 = [KeyValue::new("key1", "value1"), KeyValue::new("key3", "value3"), KeyValue::new("key2", "value2")];

    println!("Press 'Enter' to display metrics, 'Esc'/Ctrl+C to quit");
    loop {
        counter.add("counter", &attributes);
        counter.add("counter", &attributes_in_diff_order);
        counter.add("counter", &attributes_in_diff_order2);
        counter.add("counter", &attributes);
        counter.add("counter", &attributes_in_diff_order);

        counter.add("counter2", &[]);
        counter.add("counter2", &[]);
        thread::sleep(std::time::Duration::from_secs(1));

        if event::poll(std::time::Duration::from_secs(0)).unwrap() {
            match event::read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => break,                       // Quit the loop
                    KeyCode::Enter => counter.display_metrics(), // Display metrics
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
