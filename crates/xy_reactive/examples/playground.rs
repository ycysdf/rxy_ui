use parking_lot::RwLock;
use std::mem;
use std::sync::Arc;
use xy_reactive::prelude::*;

pub async fn tick() {
    tokio::time::sleep(std::time::Duration::from_micros(1)).await;
}

#[tokio::main]
async fn main() {
    let first = RwSignal::new("Greg");
    let last = RwSignal::new("Johnston");
    let use_last = RwSignal::new(true);

    let combined_count = Arc::new(RwLock::new(0));

    mem::forget(Effect::new_sync({
        let combined_count = Arc::clone(&combined_count);
        move |_| {
            *combined_count.write() += 1;
            if use_last.get() {
                println!("{} {}", first.get(), last.get());
            } else {
                println!("{}", first.get());
            }
        }
    }));

    tick().await;
    assert_eq!(*combined_count.read(), 1);

    println!("\nsetting `first` to Bob");
    first.set("Bob");
    tick().await;
    assert_eq!(*combined_count.read(), 2);

    println!("\nsetting `last` to Bob");
    last.set("Thompson");
    tick().await;
    assert_eq!(*combined_count.read(), 3);

    println!("\nsetting `use_last` to false");
    use_last.set(false);
    tick().await;
    assert_eq!(*combined_count.read(), 4);

    println!("\nsetting `last` to Jones");
    last.set("Jones");
    tick().await;
    assert_eq!(*combined_count.read(), 4);

    println!("\nsetting `last` to Jones");
    last.set("Smith");
    tick().await;
    assert_eq!(*combined_count.read(), 4);

    println!("\nsetting `last` to Stevens");
    last.set("Stevens");
    tick().await;
    assert_eq!(*combined_count.read(), 4);

    println!("\nsetting `use_last` to true");
    use_last.set(true);
    tick().await;
    assert_eq!(*combined_count.read(), 5);
}
