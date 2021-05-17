use tokio::time::Duration;
use tokio_js_set_interval::{set_interval, set_timeout};

#[tokio::main]
async fn main() {
    println!("hello1");
    set_timeout!(println!("hello2"), 1900);
    println!("hello3");
    set_timeout!(println!("hello4"), 0);
    println!("hello5");
    set_interval!(println!("hello but from interval"), 333);
    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(2000)).await;
}
