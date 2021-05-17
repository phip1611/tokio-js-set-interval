use tokio::time::Duration;
use tokio_js_set_interval::set_timeout;

#[tokio::main]
async fn main() {
    // macro takes expression
    set_timeout!(println!("hello1"), 4);
    // macro takes block
    set_timeout!({ println!("hello2") }, 3);
    // macro takes direct closure expressions
    set_timeout!(|| println!("hello3"), 2);
    // macro takes direct move closure expressions
    set_timeout!(move || println!("hello4"), 2);
    // macro takes identifiers (which must point to closures)
    let closure = || println!("hello5");
    set_timeout!(closure, 1);

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(100)).await;
}
