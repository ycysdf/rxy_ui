use xy_reactive::prelude::SignalWith;
use crate::{rx, Reactive};

pub trait SignalExt<T> {
    fn map_to_rx<U>(
        self,
        f: impl Fn(&T) -> U + Clone + Send + 'static,
    ) -> Reactive<impl Fn() -> U + Send + 'static, U>;
}

impl<T, S> SignalExt<T> for S
where
    S: SignalWith<Value = T> + Send + Clone + 'static,
    T: Send + 'static,
{
    fn map_to_rx<U>(
        self,
        f: impl Fn(&T) -> U + Clone + Send + 'static,
    ) -> Reactive<impl Fn() -> U + Send + 'static, U> {
        let signal = self;
        rx(move || signal.clone().with(f.clone()))
    }
}
