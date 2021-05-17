use std::time::Duration;
use tokio_js_set_interval::{set_interval, set_timeout};

#[tokio::main]
async fn main() {
    set_timeout!(println!("hello from timeout"), 25);
    set_interval!(println!("hello from interval"), 10);

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(40)).await;
}
