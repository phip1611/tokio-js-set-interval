use std::time::Duration;
use tokio_js_set_interval::{clear_interval, set_interval, set_timeout};

#[tokio::main]
async fn main() {
    set_timeout!(println!("hello from timeout"), 25);
    set_interval!(println!("hello from interval"), 10);
    let id = set_interval!(
        println!("hello from interval that will be cleared shortly"),
        5
    );

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(20)).await;
    clear_interval(id);
    tokio::time::sleep(Duration::from_millis(40)).await;
}
