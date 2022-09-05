use tokio::time::Duration;
use tokio_js_set_interval::{set_interval, set_interval_async, set_timeout, set_timeout_async};

#[tokio::main]
async fn main() {
    // ######################################################################
    // set_timeout

    // macro takes expression
    set_timeout!(println!("set_timeout 1"), 4);
    // macro takes block
    set_timeout!({ println!("set_timeout 2") }, 3);
    // macro takes direct closure expressions
    set_timeout!(|| println!("set_timeout 3"), 2);
    // macro takes direct move closure expressions
    set_timeout!(move || println!("set_timeout 4"), 2);
    // macro takes identifiers (which must point to closures)
    let closure = || println!("set_timeout 5");
    set_timeout!(closure, 1);

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(100)).await;

    // ######################################################################
    // set_interval

    // macro takes expression
    set_interval!(println!("set_interval 1"), 42);
    // macro takes block
    set_interval!({ println!("set_interval 2") }, 42);
    // macro takes direct closure expressions
    set_interval!(|| println!("set_interval 3"), 42);
    // macro takes direct move closure expressions
    set_interval!(move || println!("set_interval 4"), 42);
    // macro takes identifiers (which must point to closures)
    let closure = || println!("set_interval 5");
    set_interval!(closure, 42);

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(100)).await;

    // ######################################################################
    // set_timeout_async

    // macro takes identifiers (which must point to closures)
    async fn async_foo() {
        println!("set_timeout_async 1");
    }
    let future = async_foo();
    // macro takes future by identifier
    set_timeout_async!(future, 1);
    // macro takes a expression that returns a future
    set_timeout_async!(async_foo(), 1);
    // macro takes block
    set_timeout_async!(async { println!("set_timeout_async 2") }, 1);

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(100)).await;

    // ######################################################################
    // set_interval_async

    // macro takes identifiers (which must point to a future-producing closure)
    async fn async_foo2() {
        println!("set_interval_async 1");
    }
    set_interval_async!(async_foo2, 42);
    // macro takes block
    set_interval_async!(
        {
            async {
                println!("set_interval_async 2");
            }
        },
        42
    );
    // macro takes a closure that produces a future
    set_interval_async!(
        || {
            async move {
                println!("set_interval_async 3");
            }
        },
        42
    );
    // macro takes a move closure that produces a future
    set_interval_async!(
        move || {
            async move {
                println!("set_interval_async 4");
            }
        },
        42
    );

    // give enough time before tokios runtime exits
    tokio::time::sleep(Duration::from_millis(100)).await;
}
