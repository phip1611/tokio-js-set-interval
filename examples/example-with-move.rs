use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::time::Duration;
use tokio_js_set_interval::set_interval;

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicU64::new(0));

    let counter_t = counter.clone();
    set_interval!(
        move || {
            counter_t.fetch_add(1, Ordering::SeqCst);
        },
        10
    );

    let counter_t = counter.clone();
    set_interval!(
        move || {
            counter_t.fetch_add(3, Ordering::SeqCst);
            println!("val={}", counter_t.load(Ordering::SeqCst));
        },
        5
    );

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(25)).await;
}
