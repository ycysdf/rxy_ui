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

pub struct ArcTrigger {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    pub(crate) inner: Arc<RwLock<SubscriberSet>>,
}

impl ArcTrigger {
    pub(crate) fn downgrade(&self) -> WeakTrigger {
        WeakTrigger {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::downgrade(&self.inner),
        }
    }
}

impl Clone for ArcTrigger {
    #[track_caller]
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Debug for ArcTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trigger")
            .field("data", &self.inner.data_ptr())
            .finish()
    }
}

impl ArcTrigger {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            inner: Arc::new(RwLock::new(SubscriberSet::new())),
        }
    }

    #[inline(always)]
    pub fn try_notify(&self) -> Option<()> {
        self.try_set(())
    }

    #[inline(always)]
    pub fn notify(&self) {
        self.set(());
    }
}

impl ReactiveNode for ArcTrigger {
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

impl ToAnySource for ArcTrigger {
    fn to_any_source(&self) -> AnySource {
        AnySource(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Source + Send + Sync>,
        )
    }
}

impl Source for ArcTrigger {
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

impl DefinedAt for ArcTrigger {
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

impl SignalWithUntracked for ArcTrigger {
    type Value = ();

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    #[inline(always)]
    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&()))
    }
}

impl Trigger for ArcTrigger {
    fn trigger(&self) {
        self.notify();
    }
}

impl SignalUpdate for ArcTrigger {
    type Value = ();

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    #[inline(always)]
    fn update(&self, _fun: impl FnOnce(&mut Self::Value)) {
        self.mark_dirty();
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    #[inline(always)]
    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        self.mark_dirty();
        Some(fun(&mut ()))
    }
}

impl SignalIsDisposed for ArcTrigger {
    #[inline(always)]
    fn is_disposed(&self) -> bool {
        false
    }
}

pub(crate) struct WeakTrigger {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    inner: Weak<RwLock<SubscriberSet>>,
}

impl Clone for WeakTrigger {
    #[track_caller]
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner: Weak::clone(&self.inner),
        }
    }
}

impl Debug for WeakTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakTrigger").finish_non_exhaustive()
    }
}

impl WeakTrigger {
    pub fn upgrade(&self) -> Option<ArcTrigger> {
        self.inner.upgrade().map(|inner| ArcTrigger {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            inner,
        })
    }
}
