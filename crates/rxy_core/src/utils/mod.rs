use core::hash::BuildHasherDefault;
use core::any::TypeId;

#[cfg(feature = "bevy")]
pub use bevy_utils::all_tuples;
#[cfg(feature = "bevy")]
pub use bevy_utils::futures::now_or_never;
#[cfg(feature = "bevy")]
pub use bevy_utils::synccell::SyncCell;
#[cfg(feature = "bevy")]
pub use bevy_utils::OnDrop;

#[cfg(not(feature = "bevy"))]
pub use apis::now_or_never;
#[cfg(not(feature = "bevy"))]
pub use on_drop::OnDrop;
#[cfg(not(feature = "bevy"))]
pub use rxy_macro::{all_tuples, all_tuples_with_size};
#[cfg(not(feature = "bevy"))]
pub use synccell::SyncCell;

pub type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<AHasher>>;
pub type AHasher = ahash::AHasher;

#[cfg(not(feature = "bevy"))]
mod synccell;
mod on_drop;

#[cfg(not(feature = "bevy"))]
mod apis {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    pub fn now_or_never<F: Future>(mut future: F) -> Option<F::Output> {
        let noop_waker = noop_waker();
        let mut cx = Context::from_waker(&noop_waker);

        // SAFETY: `future` is not moved and the original value is shadowed
        let future = unsafe { Pin::new_unchecked(&mut future) };

        match future.poll(&mut cx) {
            Poll::Ready(x) => Some(x),
            _ => None,
        }
    }

    unsafe fn noop_clone(_data: *const ()) -> RawWaker {
        noop_raw_waker()
    }
    unsafe fn noop(_data: *const ()) {}

    const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

    fn noop_raw_waker() -> RawWaker {
        RawWaker::new(core::ptr::null(), &NOOP_WAKER_VTABLE)
    }

    fn noop_waker() -> Waker {
        // SAFETY: the `RawWakerVTable` is just a big noop and doesn't violate any of the rules in `RawWakerVTable`s documentation
        // (which talks about retaining and releasing any "resources", of which there are none in this case)
        unsafe { Waker::from_raw(noop_raw_waker()) }
    }
}

pub type TypeIdMap<V> = hashbrown::HashMap<TypeId, V, BuildHasherDefault<NoOpTypeIdHasher>>;

#[doc(hidden)]
#[derive(Default)]
pub struct NoOpTypeIdHasher(u64);

// TypeId already contains a high-quality hash, so skip re-hashing that hash.
impl core::hash::Hasher for NoOpTypeIdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        // This will never be called: TypeId always just calls write_u64 once!
        // This is a known trick and unlikely to change, but isn't officially guaranteed.
        // Don't break applications (slower fallback, just check in test):
        self.0 = bytes.iter().fold(self.0, |hash, b| {
            hash.rotate_left(8).wrapping_add(*b as u64)
        });
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}
