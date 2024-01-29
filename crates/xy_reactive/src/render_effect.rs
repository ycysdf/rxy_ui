use crate::{
    arena::Owner,
    effect::EffectInner,
    notify::channel,
    source::{AnySubscriber, SourceSet, Subscriber, ToAnySubscriber},
    spawn::spawn_local,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::{
    fmt::Debug,
    mem,
    sync::{Arc, Weak},
};
use crate::effect::ErasureEffect;

pub fn create_render_effect<T>(fun: impl FnMut(Option<T>) -> T + 'static) -> RenderEffect<T> {
    RenderEffect::new(fun)
}

pub struct RenderEffect<T>
where
    T: 'static,
{
    value: Arc<RwLock<Option<T>>>,
    inner: Arc<RwLock<EffectInner>>,
}

impl<T> Debug for RenderEffect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderEffect")
            .field("inner", &Arc::as_ptr(&self.inner))
            .finish()
    }
}

impl<T> RenderEffect<T>
where
    T: 'static,
{
    pub fn new(fun: impl FnMut(Option<T>) -> T + 'static) -> Self {
        Self::new_with_value(fun, None)
    }

    pub fn new_with_value(
        mut fun: impl FnMut(Option<T>) -> T + 'static,
        initial_value: Option<T>,
    ) -> Self {
        let (observer, mut rx) = channel();
        let value = Arc::new(RwLock::new(None));
        let owner = Owner::new();
        let inner = Arc::new(RwLock::new(EffectInner {
            owner: owner.clone(),
            observer,
            sources: SourceSet::new(),
        }));

        let initial_value = Some(owner.with(|| {
            inner
                .to_any_subscriber()
                .with_observer(|| fun(initial_value))
        }));
        *value.write() = initial_value;

        spawn_local({
            let value = Arc::clone(&value);
            let subscriber = inner.to_any_subscriber();

            async move {
                while rx.next().await.is_some() {
                    subscriber.clear_sources(&subscriber);

                    let old_value = mem::take(&mut *value.write());
                    let new_value =
                        owner.with_cleanup(|| subscriber.with_observer(|| fun(old_value)));
                    *value.write() = Some(new_value);
                }
            }
        });
        RenderEffect { value, inner }
    }

    pub fn with_value_mut<U>(&self, fun: impl FnOnce(&mut T) -> U) -> Option<U> {
        self.value.write().as_mut().map(fun)
    }

    pub fn take_value(&self) -> Option<T> {
        self.value.write().take()
    }

    pub fn erase(self) -> ErasureEffect {
        ErasureEffect {
            value: self.value.data_ptr() as usize,
            inner: self.inner,
        }
    }
}

impl<T> ToAnySubscriber for RenderEffect<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.inner.data_ptr() as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Subscriber + Send + Sync>,
        )
    }
}
