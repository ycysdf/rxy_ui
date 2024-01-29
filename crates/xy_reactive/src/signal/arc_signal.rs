use super::{ArcReadSignal, ArcWriteSignal};
use crate::{
    signal_traits::*,
    source::{
        AnySource, AnySubscriber, ReactiveNode, Source, SubscriberSet,
        ToAnySource,
    },
};
use parking_lot::RwLock;
use std::{
    fmt::Debug,
    panic::Location,
    sync::{Arc, Weak},
};

pub struct ArcRwSignal<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    pub(crate) value: Arc<RwLock<T>>,
    inner: Arc<RwLock<SubscriberSet>>,
}

impl<T> Clone for ArcRwSignal<T> {
    #[track_caller]
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            value: Arc::clone(&self.value),
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Debug for ArcRwSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcSignal")
            .field("type", &std::any::type_name::<T>())
            .field("data", &self.inner.data_ptr())
            .finish()
    }
}

impl<T> ArcRwSignal<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            value: Arc::new(RwLock::new(value)),
            inner: Arc::new(RwLock::new(SubscriberSet::new())),
        }
    }

    #[inline(always)]
    pub fn read_only(&self) -> ArcReadSignal<T> {
        ArcReadSignal(self.clone())
    }

    #[inline(always)]
    pub fn write_only(&self) -> ArcWriteSignal<T> {
        ArcWriteSignal(self.clone())
    }

    #[inline(always)]
    pub fn split(&self) -> (ArcReadSignal<T>, ArcWriteSignal<T>) {
        (self.read_only(), self.write_only())
    }

    pub fn unite(
        read: ArcReadSignal<T>,
        write: ArcWriteSignal<T>,
    ) -> Option<Self> {
        if Arc::ptr_eq(&read.0.inner, &write.0.inner) {
            Some(read.0.clone())
        } else {
            None
        }
    }
}

impl ReactiveNode for RwLock<SubscriberSet> {
    fn mark_dirty(&self) {
        self.mark_subscribers_check();
    }

    fn mark_check(&self) {}

    fn mark_subscribers_check(&self) {
        for sub in self.write().take() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        // if they're being checked, signals always count as "dirty"
        true
    }
}

impl Source for RwLock<SubscriberSet> {
    fn clear_subscribers(&self) {
        self.write().take();
    }

    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.write().subscribe(subscriber)
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.write().unsubscribe(subscriber)
    }
}

impl<T> ReactiveNode for ArcRwSignal<T> {
    fn mark_dirty(&self) {
        self.mark_subscribers_check();
    }

    fn mark_check(&self) {}

    fn mark_subscribers_check(&self) {
        self.inner.mark_subscribers_check();
    }

    fn update_if_necessary(&self) -> bool {
        // if they're being checked, signals always count as "dirty"
        true
    }
}

impl<T> ToAnySource for ArcRwSignal<T> {
    fn to_any_source(&self) -> AnySource {
        AnySource(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Source + Send + Sync>,
        )
    }
}

impl<T> Source for ArcRwSignal<T> {
    fn clear_subscribers(&self) {
        self.inner.clear_subscribers();
    }

    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.inner.add_subscriber(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.inner.remove_subscriber(subscriber);
    }
}

impl<T> DefinedAt for ArcRwSignal<T> {
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        #[cfg(debug_assertions)]
        {
            Some(self.defined_at)
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

impl<T> SignalWithUntracked for ArcRwSignal<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&self.value.read()))
    }
}

impl<T> Trigger for ArcRwSignal<T> {
    fn trigger(&self) {
        self.mark_dirty();
    }
}

impl<T> SignalUpdateUntracked for ArcRwSignal<T> {
    type Value = T;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    fn try_update_untracked<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        let value = {
            let mut value = self.value.write();
            fun(&mut value)
        };
        Some(value)
    }
}

impl<T> SignalIsDisposed for ArcRwSignal<T> {
    #[inline(always)]
    fn is_disposed(&self) -> bool {
        false
    }
}
