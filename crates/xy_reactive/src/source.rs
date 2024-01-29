use crate::{Observer, OBSERVER};
use smallvec::{IntoIter, SmallVec};
use std::{fmt::Debug, hash::Hash, mem, slice, sync::Weak};

pub trait ReactiveNode {
    /// Notifies the source's dependencies that it has changed.
    fn mark_dirty(&self);

    /// Notifies the source's dependencies that it may have changed.
    fn mark_check(&self);

    /// Marks that all subscribers need to be checked.
    fn mark_subscribers_check(&self);

    /// Regenerates the value for this node, if needed, and returns whether
    /// it has actually changed or not.
    fn update_if_necessary(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReactiveNodeState {
    Clean,
    Check,
    Dirty,
}

pub trait ToAnySource {
    /// Converts this type to its type-erased equivalent.
    fn to_any_source(&self) -> AnySource;
}

/// Describes the behavior of any source of reactivity (like a signal, trigger, or memo.)
pub trait Source: ReactiveNode {
    /// Adds a subscriber to this source's list of dependencies.
    fn add_subscriber(&self, subscriber: AnySubscriber);

    /// Removes a subscriber from this source's list of dependencies.
    fn remove_subscriber(&self, subscriber: &AnySubscriber);

    /// Remove all subscribers from this source's list of dependencies.
    fn clear_subscribers(&self);
}

pub trait Track {
    fn track(&self);
}

impl<T: Source + ToAnySource> Track for T {
    #[track_caller]
    fn track(&self) {
        if let Some(subscriber) = Observer::get() {
            subscriber.add_source(self.to_any_source());
            self.add_subscriber(subscriber);
        }
    }
}

#[derive(Clone)]
pub struct AnySource(pub usize, pub Weak<dyn Source + Send + Sync>);

impl Debug for AnySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnySource").field(&self.0).finish()
    }
}

impl Hash for AnySource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for AnySource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AnySource {}

impl ToAnySource for AnySource {
    fn to_any_source(&self) -> AnySource {
        self.clone()
    }
}

impl Source for AnySource {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        if let Some(inner) = self.1.upgrade() {
            inner.add_subscriber(subscriber)
        }
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        if let Some(inner) = self.1.upgrade() {
            inner.remove_subscriber(subscriber)
        }
    }

    fn clear_subscribers(&self) {
        if let Some(inner) = self.1.upgrade() {
            inner.clear_subscribers();
        }
    }
}

impl ReactiveNode for AnySource {
    fn mark_dirty(&self) {
        if let Some(inner) = self.1.upgrade() {
            inner.mark_dirty()
        }
    }

    fn mark_subscribers_check(&self) {
        if let Some(inner) = self.1.upgrade() {
            inner.mark_subscribers_check()
        }
    }

    fn update_if_necessary(&self) -> bool {
        if let Some(inner) = self.1.upgrade() {
            inner.update_if_necessary()
        } else {
            false
        }
    }

    fn mark_check(&self) {
        if let Some(inner) = self.1.upgrade() {
            inner.mark_check()
        }
    }
}

/// Converts a [`Subscriber`] to a type-erased [`AnySubscriber`].
pub trait ToAnySubscriber {
    /// Converts this type to its type-erased equivalent.
    fn to_any_subscriber(&self) -> AnySubscriber;
}

/// Any type that can track reactive values (like an effect or a memo).
pub trait Subscriber: ReactiveNode {
    /// Adds a subscriber to this subscriber's list of dependencies.
    fn add_source(&self, source: AnySource);

    // Clears the set of sources for this subscriber.
    fn clear_sources(&self, subscriber: &AnySubscriber);
}

/// A type-erased subscriber.
#[derive(Clone)]
pub struct AnySubscriber(pub usize, pub Weak<dyn Subscriber + Send + Sync>);

impl ToAnySubscriber for AnySubscriber {
    fn to_any_subscriber(&self) -> AnySubscriber {
        self.clone()
    }
}

impl Subscriber for AnySubscriber {
    fn add_source(&self, source: AnySource) {
        if let Some(inner) = self.1.upgrade() {
            inner.add_source(source);
        }
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        if let Some(inner) = self.1.upgrade() {
            inner.clear_sources(subscriber);
        }
    }
}

impl ReactiveNode for AnySubscriber {
    fn mark_dirty(&self) {
        if let Some(inner) = self.1.upgrade() {
            inner.mark_dirty()
        }
    }

    fn mark_subscribers_check(&self) {
        if let Some(inner) = self.1.upgrade() {
            inner.mark_subscribers_check()
        }
    }

    fn update_if_necessary(&self) -> bool {
        if let Some(inner) = self.1.upgrade() {
            inner.update_if_necessary()
        } else {
            false
        }
    }

    fn mark_check(&self) {
        if let Some(inner) = self.1.upgrade() {
            inner.mark_check()
        }
    }
}

impl AnySubscriber {
    pub fn with_observer<T>(&self, fun: impl FnOnce() -> T) -> T {
        let prev = {
            OBSERVER.with(|o| {
                mem::replace(&mut *o.borrow_mut(), Some(self.clone()))
            })
        };
        let val = fun();
        OBSERVER.with(|o| {
            *o.borrow_mut() = prev;
        });
        val
    }
}

impl Debug for AnySubscriber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnySubscriber").field(&self.0).finish()
    }
}

impl Hash for AnySubscriber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for AnySubscriber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AnySubscriber {}

#[derive(Default, Clone)]
pub struct SourceSet(SmallVec<[AnySource; 4]>);

impl SourceSet {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn insert(&mut self, source: AnySource) {
        self.0.push(source);
    }

    pub fn remove(&mut self, source: &AnySource) {
        if let Some(pos) = self.0.iter().position(|s| s == source) {
            self.0.remove(pos);
        }
    }

    pub fn take(&mut self) -> SmallVec<[AnySource; 4]> {
        mem::take(&mut self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear_sources(&mut self, subscriber: &AnySubscriber) {
        for source in self.take() {
            source.remove_subscriber(subscriber);
        }
    }
}

impl IntoIterator for SourceSet {
    type Item = AnySource;
    type IntoIter = IntoIter<[AnySource; 4]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a SourceSet {
    type Item = &'a AnySource;
    type IntoIter = slice::Iter<'a, AnySource>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Default, Clone)]
pub struct SubscriberSet(SmallVec<[AnySubscriber; 4]>);

impl SubscriberSet {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn subscribe(&mut self, subscriber: AnySubscriber) {
        if !self.0.contains(&subscriber) {
            self.0.push(subscriber);
        }
    }

    pub fn unsubscribe(&mut self, subscriber: &AnySubscriber) {
        if let Some(pos) = self.0.iter().position(|s| s == subscriber) {
            self.0.remove(pos);
        }
    }

    pub fn take(&mut self) -> SmallVec<[AnySubscriber; 4]> {
        mem::take(&mut self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for SubscriberSet {
    type Item = AnySubscriber;
    type IntoIter = IntoIter<[AnySubscriber; 4]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a SubscriberSet {
    type Item = &'a AnySubscriber;
    type IntoIter = slice::Iter<'a, AnySubscriber>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
