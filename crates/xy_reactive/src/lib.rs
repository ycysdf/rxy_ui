mod arena;
// pub mod async_signal;
// pub mod context;
pub mod effect;
pub mod memo;
mod notify;
pub mod render_effect;
pub mod selector;
#[cfg(feature = "serde")]
mod serde;
pub mod serialization;
// pub mod shared_context;
pub mod signal;
pub mod signal_traits;
mod source;
pub mod spawn;
pub mod store;
use crate::source::AnySubscriber;
pub use arena::{Owner, Root};
use futures::{Future, Stream};
use std::{cell::RefCell, pin::Pin};

pub mod prelude {
    pub use crate::{
        // async_signal::{AsyncDerived, Resource},
        // context::{provide_context, use_context},
        effect::{create_effect, Effect},
        memo::{use_memo, ArcMemo, Memo},
        render_effect::create_render_effect,
        signal::{
            use_rw_signal, use_signal, ArcRwSignal, ArcWriteSignal, ReadSignal, RwSignal, WriteSignal,
        },
        signal_traits::*,
        store::{StoreField, StoreFieldIndex, StoreFieldIterator},
        Root,
    };
}

thread_local! {
    static OBSERVER: RefCell<Option<AnySubscriber>> = RefCell::new(None);
}

pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
pub type PinnedStream<T> = Pin<Box<dyn Stream<Item = T> + Send + Sync>>;

pub(crate) struct Observer {}

impl Observer {
    fn get() -> Option<AnySubscriber> {
        OBSERVER.with(|o| o.borrow().clone())
    }

    fn is(observer: &AnySubscriber) -> bool {
        OBSERVER.with(|o| o.borrow().as_ref() == Some(observer))
    }

    fn take() -> Option<AnySubscriber> {
        OBSERVER.with(|o| o.borrow_mut().take())
    }

    fn set(observer: Option<AnySubscriber>) {
        OBSERVER.with(|o| *o.borrow_mut() = observer);
    }
}

pub fn untrack<T>(fun: impl FnOnce() -> T) -> T {
    let prev = Observer::take();
    let value = fun();
    Observer::set(prev);
    value
}

#[cfg(feature = "web")]
pub fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s));
}

#[cfg(not(feature = "web"))]
pub fn log(s: &str) {
    println!("{s}");
}
