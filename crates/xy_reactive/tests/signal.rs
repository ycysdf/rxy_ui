use tachy_reaccy::prelude::*;

#[test]
fn create_signal() {
    let a = RwSignal::new(0);
    assert_eq!(a.get(), 0);
}

#[test]
fn update_signal() {
    let a = RwSignal::new(0);
    a.update(|n| *n += 1);
    assert_eq!(a.get(), 1);
}
