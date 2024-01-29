/* use leptos_reactive::{
    batch, create_isomorphic_effect, create_memo, create_runtime,
    create_rw_signal, create_signal, untrack, SignalGet, SignalSet,
}; */
use parking_lot::RwLock;
use std::{mem, sync::Arc};
use tachy_reaccy::prelude::*;

pub async fn tick() {
    tokio::time::sleep(std::time::Duration::from_micros(1)).await;
}

#[tokio::test]
async fn effect_runs() {
    let a = RwSignal::new(-1);

    // simulate an arbitrary side effect
    let b = Arc::new(RwLock::new(String::new()));

    // we forget it so it continues running
    // if it's dropped, it will stop listening
    mem::forget(Effect::new_sync({
        let b = b.clone();
        move |_| {
            let formatted = format!("Value is {}", a.get());
            *b.write() = formatted;
        }
    }));

    tick().await;
    assert_eq!(b.read().as_str(), "Value is -1");

    println!("setting to 1");
    a.set(1);

    tick().await;
    assert_eq!(b.read().as_str(), "Value is 1");
}

#[tokio::test]
async fn dynamic_dependencies() {
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

/*
#[test]
fn effect_tracks_memo() {
    use std::{cell::RefCell, rc::Rc};

    let runtime = create_runtime();
    let (a, set_a) = create_signal(-1);
    let b = create_memo(move |_| format!("Value is {}", a.get()));

    // simulate an arbitrary side effect
    let c = Arc::new(RwLock::new(String::new()));

    create_isomorphic_effect({
        let c = c.clone();
        move |_| {
            *c.write() = b.get();
        }
    });

    assert_eq!(b.get().as_str(), "Value is -1");
    assert_eq!(c.read().as_str(), "Value is -1");

    set_a.set(1);

    assert_eq!(b.get().as_str(), "Value is 1");
    assert_eq!(c.read().as_str(), "Value is 1");

    runtime.dispose();
}

#[test]
fn untrack_mutes_effect() {
    use std::{cell::RefCell, rc::Rc};

    let runtime = create_runtime();

    let (a, set_a) = create_signal(-1);

    // simulate an arbitrary side effect
    let b = Arc::new(RwLock::new(String::new()));

    create_isomorphic_effect({
        let b = b.clone();
        move |_| {
            let formatted = format!("Value is {}", untrack(move || a.get()));
            *b.write() = formatted;
        }
    });

    assert_eq!(a.get(), -1);
    assert_eq!(b.read().as_str(), "Value is -1");

    set_a.set(1);

    assert_eq!(a.get(), 1);
    assert_eq!(b.read().as_str(), "Value is -1");

    runtime.dispose();
}

#[test]
fn batching_actually_batches() {
    use std::{cell::Cell, rc::Rc};

    let runtime = create_runtime();

    let first_name = create_rw_signal("Greg".to_string());
    let last_name = create_rw_signal("Johnston".to_string());

    // simulate an arbitrary side effect
    let count = Arc::new(Cell::new(0));

    create_isomorphic_effect({
        let count = count.clone();
        move |_| {
            _ = first_name.get();
            _ = last_name.get();

            count.set(count.get() + 1);
        }
    });

    // runs once initially
    assert_eq!(count.get(), 1);

    // individual updates run effect once each
    first_name.set("Alice".to_string());
    assert_eq!(count.get(), 2);

    last_name.set("Smith".to_string());
    assert_eq!(count.get(), 3);

    // batched effect only runs twice
    batch(move || {
        first_name.set("Bob".to_string());
        last_name.set("Williams".to_string());
    });
    assert_eq!(count.get(), 4);

    runtime.dispose();
}
 */
