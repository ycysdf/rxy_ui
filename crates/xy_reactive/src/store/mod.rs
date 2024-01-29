use crate::{
    arena::{Stored, StoredData},
    prelude::{SignalUpdateUntracked, SignalWithUntracked, Trigger},
    signal::trigger::ArcTrigger,
    signal_traits::{DefinedAt, SignalIsDisposed},
    source::Track,
};
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock};
use rustc_hash::FxHashMap;
use std::{fmt::Debug, panic::Location, sync::Arc};
mod indexed;
pub use indexed::*;
mod keyed;
pub use keyed::*;
mod path;
pub use path::*;
mod stored;
pub use stored::*;

pub struct Store<T: Send + Sync + 'static> {
    inner: Stored<ArcStore<T>>,
}

impl<T: Send + Sync + 'static> Store<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "debug", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            inner: Stored::new(ArcStore::new(value)),
        }
    }
}

impl<T: Send + Sync + 'static> Copy for Store<T> {}

impl<T: Send + Sync + 'static> Clone for Store<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Debug + Send + Sync + 'static> Debug for Store<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            .field("type", &std::any::type_name::<T>())
            .field("store", &self.inner)
            .finish()
    }
}

impl<T: Send + Sync + 'static> SignalIsDisposed for Store<T> {
    fn is_disposed(&self) -> bool {
        self.inner.exists()
    }
}

impl<T: Send + Sync + 'static> StoredData for Store<T> {
    type Data = ArcStore<T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

pub struct ArcStore<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    pub(crate) value: Arc<RwLock<T>>,
    signals: Arc<RwLock<TriggerMap>>,
}

#[derive(Debug, Default)]
struct TriggerMap(FxHashMap<StorePath, ArcTrigger>);

impl TriggerMap {
    fn get_or_insert(&mut self, key: StorePath) -> ArcTrigger {
        if let Some(trigger) = self.0.get(&key) {
            trigger.clone()
        } else {
            let new = ArcTrigger::new();
            self.0.insert(key, new.clone());
            new
        }
    }

    fn remove(&mut self, key: &StorePath) -> Option<ArcTrigger> {
        self.0.remove(key)
    }
}

impl<T> ArcStore<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            value: Arc::new(RwLock::new(value)),
            signals: Default::default(),
            /* inner: Arc::new(RwLock::new(SubscriberSet::new())), */
        }
    }
}

impl<T: Debug> Debug for ArcStore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("ArcStore");
        #[cfg(debug_assertions)]
        let f = f.field("defined_at", &self.defined_at);
        f.field("value", &self.value)
            .field("signals", &self.signals)
            .finish()
    }
}

impl<T> Clone for ArcStore<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            value: Arc::clone(&self.value),
            signals: Arc::clone(&self.signals),
        }
    }
}

impl<T> SignalWithUntracked for ArcStore<T> {
    type Value = T;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&self.value.read()))
    }
}

impl<T> Trigger for ArcStore<T> {
    fn trigger(&self) {
        self.get_trigger(self.path().collect()).notify();
    }
}

impl<T> SignalUpdateUntracked for ArcStore<T> {
    type Value = T;

    fn try_update_untracked<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&mut self.value.write()))
    }
}

impl<T> SignalIsDisposed for ArcStore<T> {
    fn is_disposed(&self) -> bool {
        false
    }
}

pub struct ArcRwStoreField<Orig, T>
where
    T: 'static,
{
    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,
    data: Arc<RwLock<Orig>>,
    trigger: ArcTrigger,
    read: Arc<
        dyn for<'a> Fn(&'a RwLock<Orig>) -> MappedRwLockReadGuard<'a, T>
            + Send
            + Sync,
    >,
    write: Arc<
        dyn for<'a> Fn(&'a RwLock<Orig>) -> MappedRwLockWriteGuard<'a, T>
            + Send
            + Sync,
    >,
}

impl<Orig, T> ArcRwStoreField<Orig, T>
where
    T: 'static,
{
    #[track_caller]
    pub fn read_only(&self) -> ArcReadStoreField<Orig, T> {
        ArcReadStoreField {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            data: Arc::clone(&self.data),
            trigger: self.trigger.clone(),
            read: Arc::clone(&self.read),
        }
    }

    #[track_caller]
    pub fn write_only(&self) -> ArcWriteStoreField<Orig, T> {
        ArcWriteStoreField {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            data: Arc::clone(&self.data),
            trigger: self.trigger.clone(),
            write: Arc::clone(&self.write),
        }
    }

    #[inline(always)]
    pub fn split(
        &self,
    ) -> (ArcReadStoreField<Orig, T>, ArcWriteStoreField<Orig, T>) {
        (self.read_only(), self.write_only())
    }

    pub fn unite(
        read: ArcReadStoreField<Orig, T>,
        write: ArcWriteStoreField<Orig, T>,
    ) -> Option<Self> {
        if Arc::ptr_eq(&read.trigger.inner, &write.trigger.inner) {
            let ArcReadStoreField {
                #[cfg(debug_assertions)]
                defined_at,
                data,
                trigger,
                read,
            } = read;
            let ArcWriteStoreField { write, .. } = write;
            Some(Self {
                #[cfg(debug_assertions)]
                defined_at,
                data,
                trigger,
                read,
                write,
            })
        } else {
            None
        }
    }
}

impl<Orig, T> Clone for ArcRwStoreField<Orig, T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            data: Arc::clone(&self.data),
            trigger: self.trigger.clone(),
            read: Arc::clone(&self.read),
            write: Arc::clone(&self.write),
        }
    }
}

impl<Orig, T> DefinedAt for ArcRwStoreField<Orig, T> {
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

impl<Orig, T> Track for ArcRwStoreField<Orig, T> {
    fn track(&self) {
        self.trigger.track();
    }
}

impl<Orig, T> SignalWithUntracked for ArcRwStoreField<Orig, T> {
    type Value = T;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&*(self.read)(&self.data)))
    }
}

impl<Orig, T> Trigger for ArcRwStoreField<Orig, T> {
    fn trigger(&self) {
        self.trigger.notify();
    }
}

impl<Orig, T> SignalUpdateUntracked for ArcRwStoreField<Orig, T> {
    type Value = T;

    fn try_update_untracked<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&mut *(self.write)(&self.data)))
    }
}

impl<Orig, T> SignalIsDisposed for ArcRwStoreField<Orig, T> {
    fn is_disposed(&self) -> bool {
        false
    }
}

pub struct ArcReadStoreField<Orig, T>
where
    T: 'static,
{
    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,
    data: Arc<RwLock<Orig>>,
    trigger: ArcTrigger,
    read: Arc<
        dyn for<'a> Fn(&'a RwLock<Orig>) -> MappedRwLockReadGuard<'a, T>
            + Send
            + Sync,
    >,
}

impl<Orig, T> Clone for ArcReadStoreField<Orig, T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            data: Arc::clone(&self.data),
            trigger: self.trigger.clone(),
            read: Arc::clone(&self.read),
        }
    }
}

impl<Orig, T> DefinedAt for ArcReadStoreField<Orig, T> {
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

impl<Orig, T> Track for ArcReadStoreField<Orig, T> {
    fn track(&self) {
        self.trigger.track();
    }
}

impl<Orig, T> SignalWithUntracked for ArcReadStoreField<Orig, T> {
    type Value = T;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&*(self.read)(&self.data)))
    }
}

pub struct ArcWriteStoreField<Orig, T>
where
    T: 'static,
{
    #[cfg(debug_assertions)]
    defined_at: &'static std::panic::Location<'static>,
    data: Arc<RwLock<Orig>>,
    trigger: ArcTrigger,
    write: Arc<
        dyn for<'a> Fn(&'a RwLock<Orig>) -> MappedRwLockWriteGuard<'a, T>
            + Send
            + Sync,
    >,
}

impl<Orig, T> Clone for ArcWriteStoreField<Orig, T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            data: Arc::clone(&self.data),
            trigger: self.trigger.clone(),
            write: Arc::clone(&self.write),
        }
    }
}

impl<Orig, T> DefinedAt for ArcWriteStoreField<Orig, T> {
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

impl<Orig, T> Trigger for ArcWriteStoreField<Orig, T> {
    fn trigger(&self) {
        self.trigger.notify();
    }
}

impl<Orig, T> SignalUpdateUntracked for ArcWriteStoreField<Orig, T> {
    type Value = T;

    fn try_update_untracked<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        Some(fun(&mut *(self.write)(&self.data)))
    }
}

impl<Orig, T> SignalIsDisposed for ArcWriteStoreField<Orig, T> {
    fn is_disposed(&self) -> bool {
        false
    }
}

/* #[cfg(test)]
mod tests {
    use super::{ArcReadStoreField, ArcStore, ArcWriteStoreField};
    use crate::{
        effect::Effect,
        prelude::{SignalSet, SignalUpdate, SignalWith},
    };
    use parking_lot::RwLock;
    use std::{mem, sync::Arc};

    pub async fn tick() {
        tokio::time::sleep(std::time::Duration::from_micros(1)).await;
    }

    #[derive(Debug)]
    struct Todos {
        user: String,
        todos: Vec<Todo>,
    }

    // macro expansion 2
    impl<Orig> ReadStoreField<Orig, Todos>
    where
        Orig: 'static,
    {
        pub fn user(self) -> ReadStoreField<Orig, String> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::user as usize,
                |prev| &prev.user,
            )
        }

        pub fn todos(self) -> ReadStoreField<Orig, Vec<Todo>> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::todos as usize,
                |prev| &prev.todos,
            )
        }
    }

    impl<Orig> WriteStoreField<Orig, Todos>
    where
        Orig: 'static,
    {
        pub fn user(self) -> WriteStoreField<Orig, String> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::user as usize,
                |prev| &mut prev.user,
            )
        }

        pub fn todos(self) -> WriteStoreField<Orig, Vec<Todo>> {
            self.subfield(
                ReadStoreField::<Orig, Todos>::todos as usize,
                |prev| &mut prev.todos,
            )
        }
    }
    // end macro expansion 2

    #[derive(Debug)]
    struct Todo {
        label: String,
        completed: bool,
    }

    fn data() -> Todos {
        Todos {
            user: "Bob".to_string(),
            todos: vec![
                Todo {
                    label: "Create reactive store".to_string(),
                    completed: true,
                },
                Todo {
                    label: "???".to_string(),
                    completed: false,
                },
                Todo {
                    label: "Profit".to_string(),
                    completed: false,
                },
            ],
        }
    }

    #[tokio::test]
    async fn mutating_field_triggers_effect() {
        let combined_count = Arc::new(RwLock::new(0));

        let store = ArcStore::new(data());
        mem::forget(Effect::new_sync({
            let store = store.clone();
            let combined_count = Arc::clone(&combined_count);
            move |prev| {
                if prev.is_none() {
                    println!("first run");
                } else {
                    println!("next run");
                }
                store.at().user().with(|user| println!("{user:?}"));
                *combined_count.write() += 1;
            }
        }));
        tick().await;
        store.at_mut().user().set("Greg");
        tick().await;
        store.at_mut().user().set("Carol");
        tick().await;
        store.at_mut().user().update(|name| name.push_str("!!!"));
        tick().await;
        assert_eq!(*combined_count.read(), 4);
    }
}
 */
