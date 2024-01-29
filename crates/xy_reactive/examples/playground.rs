use parking_lot::RwLock;
use std::mem;
use std::sync::Arc;
use xy_reactive::prelude::*;

pub async fn tick() {
    tokio::time::sleep(std::time::Duration::from_micros(1)).await;
}

#[tokio::main]
async fn main2() {
    let a = RwSignal::new(-1);

    println!("b");
    // simulate an arbitrary side effect
    let b = Arc::new(RwLock::new(String::new()));

    // we forget it so it continues running
    // if it's dropped, it will stop listening

    println!("effect");
    mem::forget(Effect::new_sync({
        let b = b.clone();
        move |_| {
            println!("new_sync");
            let formatted = format!("Value is {}", a.get());
            *b.write() = formatted;
            println!("new_sync ee");
        }
    }));

    println!("tick");
    tick().await;
    println!("b.read()");
    assert_eq!(b.read().as_str(), "Value is -1");

    println!("setting to 1");
    a.set(1);

    tick().await;
    assert_eq!(b.read().as_str(), "Value is 1");
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
