use super::{AsyncState, ScopedFuture};
use crate::{
    arena::{Owner, Stored, StoredData},
    notify::{channel, Sender},
    prelude::{DefinedAt, SignalWithUntracked},
    source::{
        AnySource, AnySubscriber, ReactiveNode, Source, SourceSet, Subscriber,
        SubscriberSet, ToAnySource, ToAnySubscriber, Track,
    },
    spawn::{spawn, spawn_local},
    unwrap_signal,
};
use futures::{FutureExt, StreamExt};
use parking_lot::RwLock;
use std::{
    fmt::Debug,
    future::{Future, IntoFuture},
    mem,
    panic::Location,
    pin::Pin,
    sync::{Arc, Weak},
    task::{Context, Poll, Waker},
};
pub struct ArcAsyncDerived<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    // the current state of this signal
    value: Arc<RwLock<AsyncState<T>>>,
    // holds wakers generated when you .await this
    wakers: Arc<RwLock<Vec<Waker>>>,
    inner: Arc<RwLock<ArcAsyncDerivedInner>>,
}

impl<T> Clone for ArcAsyncDerived<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            value: Arc::clone(&self.value),
            wakers: Arc::clone(&self.wakers),
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Debug for ArcAsyncDerived<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("ArcAsyncDerived");
        #[cfg(debug_assertions)]
        f.field("defined_at", &self.defined_at);
        f.finish_non_exhaustive()
    }
}

struct ArcAsyncDerivedInner {
    owner: Owner,
    // holds subscribers so the dependency can be cleared when this needs to rerun
    sources: SourceSet,
    // tracks reactive subscribers so they can be notified
    // when the new async value is ready
    subscribers: SubscriberSet,
    // when a source changes, notifying this will cause the async work to rerun
    notifier: Sender,
}

// This implemented creating a derived async signal.
// It needs to be implemented as a macro because it needs to be flexible over
// whether `fun` returns a `Future` that is `Send + Sync`. Doing it as a function would,
// as far as I can tell, require repeating most of the function body.
macro_rules! spawn_derived {
    ($spawner:ident, $initial:ident, $fun:ident) => {{
        let (mut notifier, mut rx) = channel();

        // begin loading eagerly but asynchronously, if not already loaded
        if matches!($initial, AsyncState::Loading) {
            notifier.notify();
        }
        let is_ready = matches!($initial, AsyncState::Complete(_));

        let inner = Arc::new(RwLock::new(ArcAsyncDerivedInner {
            owner: Owner::new(),
            notifier,
            sources: SourceSet::new(),
            subscribers: SubscriberSet::new(),
        }));
        let value = Arc::new(RwLock::new($initial));
        let wakers = Arc::new(RwLock::new(Vec::new()));

        let this = ArcAsyncDerived {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            value,
            wakers,
            inner: Arc::clone(&inner),
        };
        let any_subscriber = this.to_any_subscriber();

        // if it's immediately available, poll once
        // this means either
        // a) it's synchronous, or
        // b) it was hydrated, and we want to access any reactivity
        if is_ready {
            let owner = this.inner.read().owner.clone();
            let fut = owner.with_cleanup(|| {
                any_subscriber
                    .with_observer(|| ScopedFuture::new($fun()))
            });
            _ = fut.now_or_never();
        }

        $spawner({
            let value = Arc::downgrade(&this.value);
            let inner = Arc::downgrade(&this.inner);
            let wakers = Arc::downgrade(&this.wakers);
            async move {
                while rx.next().await.is_some() {
                    match (value.upgrade(), inner.upgrade(), wakers.upgrade()) {
                        (Some(value), Some(inner), Some(wakers)) => {
                            // generate new Future
                            let owner = inner.read().owner.clone();
                            let fut = owner.with_cleanup(|| {
                                any_subscriber
                                    .with_observer(|| ScopedFuture::new($fun()))
                            });

                            // update state from Complete to Reloading
                            {
                                let mut value = value.write();
                                // if it's initial Loading, it will just reset to Loading
                                if let AsyncState::Complete(old) =
                                    mem::take(&mut *value)
                                {
                                    *value = AsyncState::Reloading(old);
                                }
                            }

                            // notify reactive subscribers that we're now loading
                            for sub in (&inner.read().subscribers).into_iter() {
                                sub.mark_check();
                            }

                            // generate and assign new value
                            let new_value = fut.await;
                            *value.write() = AsyncState::Complete(new_value);

                            // notify reactive subscribers that we're not loading any more
                            for sub in (&inner.read().subscribers).into_iter() {
                                sub.mark_check();
                            }

                            // notify async .awaiters
                            for waker in mem::take(&mut *wakers.write()) {
                                waker.wake();
                            }
                        }
                        _ => break,
                    }
                }
            }
        });

        this
    }};
}

impl<T> DefinedAt for ArcAsyncDerived<T> {
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

impl<T: 'static> ArcAsyncDerived<T> {
    #[track_caller]
    pub fn new<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
    where
        T: Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        Self::new_with_initial(AsyncState::Loading, fun)
    }

    #[track_caller]
    pub fn new_with_initial<Fut>(
        initial_value: AsyncState<T>,
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> Self
    where
        T: Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        spawn_derived!(spawn, initial_value, fun)
    }

    #[track_caller]
    pub fn new_unsync<Fut>(fun: impl Fn() -> Fut + 'static) -> Self
    where
        T: 'static,
        Fut: Future<Output = T> + 'static,
    {
        Self::new_unsync_with_initial(AsyncState::Loading, fun)
    }

    #[track_caller]
    pub fn new_unsync_with_initial<Fut>(
        initial_value: AsyncState<T>,
        fun: impl Fn() -> Fut + 'static,
    ) -> Self
    where
        T: 'static,
        Fut: Future<Output = T> + 'static,
    {
        spawn_derived!(spawn_local, initial_value, fun)
    }

    pub fn ready(&self) -> AsyncDerivedReadyFuture<T> {
        AsyncDerivedReadyFuture {
            source: self.to_any_source(),
            value: Arc::clone(&self.value),
            wakers: Arc::clone(&self.wakers),
        }
    }
}

impl<T> SignalWithUntracked for ArcAsyncDerived<T> {
    type Value = AsyncState<T>;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&self.value.read()))
    }
}

impl<T: 'static> ToAnySource for ArcAsyncDerived<T> {
    fn to_any_source(&self) -> AnySource {
        AnySource(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Source + Send + Sync>,
        )
    }
}

impl<T: 'static> ToAnySubscriber for ArcAsyncDerived<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Subscriber + Send + Sync>,
        )
    }
}

impl ReactiveNode for RwLock<ArcAsyncDerivedInner> {
    fn mark_dirty(&self) {
        self.write().notifier.notify();
    }

    fn mark_check(&self) {
        self.write().notifier.notify();
    }

    fn mark_subscribers_check(&self) {
        let lock = self.read();
        for sub in (&lock.subscribers).into_iter() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        // always return false, because the async work will not be ready yet
        // we'll mark subscribers dirty again when it resolves
        false
    }
}

impl<T> Source for ArcAsyncDerived<T> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.inner.add_subscriber(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.inner.remove_subscriber(subscriber);
    }

    fn clear_subscribers(&self) {
        self.inner.clear_subscribers();
    }
}

impl<T> ReactiveNode for ArcAsyncDerived<T> {
    fn mark_dirty(&self) {
        self.inner.mark_dirty();
    }

    fn mark_check(&self) {
        self.inner.mark_check();
    }

    fn mark_subscribers_check(&self) {
        self.inner.mark_subscribers_check();
    }

    fn update_if_necessary(&self) -> bool {
        self.inner.update_if_necessary()
    }
}

impl<T> Subscriber for ArcAsyncDerived<T> {
    fn add_source(&self, source: AnySource) {
        self.inner.add_source(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.inner.clear_sources(subscriber);
    }
}

impl Source for RwLock<ArcAsyncDerivedInner> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.write().subscribers.subscribe(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.write().subscribers.unsubscribe(subscriber);
    }

    fn clear_subscribers(&self) {
        self.write().subscribers.take();
    }
}

impl Subscriber for RwLock<ArcAsyncDerivedInner> {
    fn add_source(&self, source: AnySource) {
        self.write().sources.insert(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.write().sources.clear_sources(subscriber);
    }
}

/// A [`Future`] that is ready when an [`ArcAsyncDerived`] is finished loading or reloading,
/// but does not contain its value.
pub struct AsyncDerivedReadyFuture<T> {
    source: AnySource,
    value: Arc<RwLock<AsyncState<T>>>,
    wakers: Arc<RwLock<Vec<Waker>>>,
}

impl<T: 'static> Future for AsyncDerivedReadyFuture<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        self.source.track();
        match &*self.value.read() {
            AsyncState::Loading | AsyncState::Reloading(_) => {
                self.wakers.write().push(waker.clone());
                Poll::Pending
            }
            AsyncState::Complete(_) => Poll::Ready(()),
        }
    }
}

/// A [`Future`] that is ready when an [`ArcAsyncDerived`] is finished loading or reloading,
/// and contains its value.
pub struct AsyncDerivedFuture<T> {
    source: AnySource,
    value: Arc<RwLock<AsyncState<T>>>,
    wakers: Arc<RwLock<Vec<Waker>>>,
}

impl<T: Clone + 'static> IntoFuture for ArcAsyncDerived<T> {
    type Output = T;
    type IntoFuture = AsyncDerivedFuture<T>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncDerivedFuture {
            source: self.to_any_source(),
            value: Arc::clone(&self.value),
            wakers: Arc::clone(&self.wakers),
        }
    }
}

impl<T: Clone + 'static> Future for AsyncDerivedFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        self.source.track();
        match &*self.value.read() {
            AsyncState::Loading | AsyncState::Reloading(_) => {
                self.wakers.write().push(waker.clone());
                Poll::Pending
            }
            AsyncState::Complete(value) => Poll::Ready(value.clone()),
        }
    }
}

pub struct AsyncDerived<T: Send + Sync + 'static> {
    inner: Stored<ArcAsyncDerived<T>>,
}

impl<T: Send + Sync + 'static> StoredData for AsyncDerived<T> {
    type Data = ArcAsyncDerived<T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

impl<T: Send + Sync + 'static> AsyncDerived<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
    where
        T: Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        Self {
            inner: Stored::new(ArcAsyncDerived::new(fun)),
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new_with_initial<Fut>(
        initial_value: AsyncState<T>,
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> Self
    where
        T: Send + Sync + 'static,
        Fut: Future<Output = T> + Send + Sync + 'static,
    {
        Self {
            inner: Stored::new(ArcAsyncDerived::new_with_initial(
                initial_value,
                fun,
            )),
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new_unsync<Fut>(fun: impl Fn() -> Fut + 'static) -> Self
    where
        T: 'static,
        Fut: Future<Output = T> + 'static,
    {
        Self {
            inner: Stored::new(ArcAsyncDerived::new_unsync(fun)),
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new_unsync_with_initial<Fut>(
        initial_value: AsyncState<T>,
        fun: impl Fn() -> Fut + 'static,
    ) -> Self
    where
        T: 'static,
        Fut: Future<Output = T> + 'static,
    {
        Self {
            inner: Stored::new(ArcAsyncDerived::new_unsync_with_initial(
                initial_value,
                fun,
            )),
        }
    }

    #[track_caller]
    pub fn ready(&self) -> AsyncDerivedReadyFuture<T> {
        let this = self.inner.get().unwrap_or_else(unwrap_signal!(self));
        this.ready()
    }
}

impl<T: Send + Sync + 'static> Copy for AsyncDerived<T> {}

impl<T: Send + Sync + 'static> Clone for AsyncDerived<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Send + Sync + 'static> Debug for AsyncDerived<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncDerived")
            .field("type", &std::any::type_name::<T>())
            .field("store", &self.inner)
            .finish()
    }
}

impl<T: Send + Sync + Clone + 'static> IntoFuture for AsyncDerived<T> {
    type Output = T;
    type IntoFuture = AsyncDerivedFuture<T>;

    #[track_caller]
    fn into_future(self) -> Self::IntoFuture {
        let this = self.inner.get().unwrap_or_else(unwrap_signal!(self));
        this.into_future()
    }
}
/*
#[cfg(test)]
mod tests {
    use crate::{
        async_signal::{AsyncDerived, AsyncState},
        prelude::{RwSignal, SignalGet, SignalGetUntracked, SignalSet},
    };
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn tracks_in_fn_and_async_block() {
        let a = RwSignal::new(1);
        let b = RwSignal::new(2);

        let c = AsyncDerived::new(move || {
            let a = a.get();
            async move {
                sleep(Duration::from_millis(50)).await;
                b.get() + a
            }
        });

        assert_eq!(a.get(), 1);
        assert_eq!(b.get(), 2);

        // state is initially Loading
        assert_eq!(c.get_untracked(), AsyncState::Loading);

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.get_untracked(), AsyncState::Complete(3));

        a.set(2);

        // state is asynchronously set to Reloading and holds old value
        sleep(Duration::from_millis(5)).await;
        assert_eq!(c.get_untracked(), AsyncState::Reloading(3));

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.get_untracked(), AsyncState::Complete(4));

        b.set(3);

        // state is asynchronously set to Reloading and holds old value
        sleep(Duration::from_millis(5)).await;
        assert_eq!(c.get_untracked(), AsyncState::Reloading(4));
        sleep(Duration::from_millis(150)).await;
        assert_eq!(c.get_untracked(), AsyncState::Complete(5));
    }

    #[tokio::test]
    async fn awaiting_directly_works() {
        let a = RwSignal::new(1);
        let b = RwSignal::new(2);

        let c = AsyncDerived::new(move || {
            let a = a.get();
            async move {
                sleep(Duration::from_millis(50)).await;
                b.get() + a
            }
        });

        assert_eq!(a.get(), 1);
        assert_eq!(b.get(), 2);

        // after it's done loading, state is Complete
        assert_eq!(c.await, 3);

        a.set(2);

        // after it's done loading, state is Complete
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.await, 4);

        b.set(3);
        sleep(Duration::from_millis(75)).await;
        assert_eq!(c.await, 5);
    }
}
 */
