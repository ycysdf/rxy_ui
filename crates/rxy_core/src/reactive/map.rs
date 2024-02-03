use xy_reactive::prelude::SignalWith;
use crate::{rx, Reactive, MaybeSend};

pub trait SignalExt<T> {
    fn map_to_rx<U>(
        self,
        f: impl Fn(&T) -> U + Clone + MaybeSend + 'static,
    ) -> Reactive<impl Fn() -> U + MaybeSend + 'static, U>;
}

impl<T, S> SignalExt<T> for S
where
    S: SignalWith<Value = T> + MaybeSend + Clone + 'static,
    T: MaybeSend + 'static,
{
    fn map_to_rx<U>(
        self,
        f: impl Fn(&T) -> U + Clone + MaybeSend + 'static,
    ) -> Reactive<impl Fn() -> U + MaybeSend + 'static, U> {
        let signal = self;
        rx(move || signal.clone().with(f.clone()))
    }
}
