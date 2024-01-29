mod arc_signal;
mod read;
mod write;
use crate::{
    arena::{Stored, StoredData},
    signal_traits::*,
    unwrap_signal,
};
pub use arc_signal::ArcRwSignal;
pub use read::*;
pub mod trigger;
use std::{fmt::Debug, sync::Arc};
pub use write::*;

pub fn use_rw_signal<T>(initial_value: T) -> RwSignal<T>
where
    T: Send + Sync + 'static,
{
    RwSignal::new(initial_value)
}

pub fn use_signal<T>(initial_value: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Send + Sync + 'static,
{
    let signal = RwSignal::new(initial_value);
    signal.split()
}

pub fn use_arc_signal<T>(initial_value: T) -> (ArcReadSignal<T>, ArcWriteSignal<T>)
where
    T: Send + Sync + 'static,
{
    let signal = ArcRwSignal::new(initial_value);
    signal.split()
}

pub struct RwSignal<T: Send + Sync + 'static> {
    inner: Stored<ArcRwSignal<T>>,
}

impl<T: Send + Sync + 'static> RwSignal<T> {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", skip_all,))]
    pub fn new(value: T) -> Self {
        Self {
            inner: Stored::new(ArcRwSignal::new(value)),
        }
    }

    #[inline(always)]
    pub fn read_only(&self) -> ReadSignal<T> {
        ReadSignal {
            inner: Stored::new(
                self.get_value()
                    .map(|inner| inner.read_only())
                    .unwrap_or_else(unwrap_signal!(self)),
            ),
        }
    }

    #[inline(always)]
    pub fn write_only(&self) -> WriteSignal<T> {
        WriteSignal {
            inner: Stored::new(
                self.get_value()
                    .map(|inner| inner.write_only())
                    .unwrap_or_else(unwrap_signal!(self)),
            ),
        }
    }

    #[inline(always)]
    pub fn split(&self) -> (ReadSignal<T>, WriteSignal<T>) {
        (self.read_only(), self.write_only())
    }

    pub fn unite(read: ReadSignal<T>, write: WriteSignal<T>) -> Option<Self> {
        match (read.inner.get(), write.inner.get()) {
            (Some(read), Some(write)) => {
                if Arc::ptr_eq(&read.0.value, &write.0.value) {
                    Some(Self {
                        inner: Stored::new(read.0.clone()),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl<T: Default + Send + Sync + 'static> Default for RwSignal<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Send + Sync + 'static> Copy for RwSignal<T> {}

impl<T: Send + Sync + 'static> Clone for RwSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Debug for RwSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("type", &std::any::type_name::<T>())
            .field("store", &self.inner)
            .finish()
    }
}

impl<T: Send + Sync + 'static> SignalIsDisposed for RwSignal<T> {
    fn is_disposed(&self) -> bool {
        self.inner.exists()
    }
}

impl<T: Send + Sync + 'static> StoredData for RwSignal<T> {
    type Data = ArcRwSignal<T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}
