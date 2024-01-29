pub use crate::source::Track;
use std::panic::Location;

#[macro_export]
macro_rules! unwrap_signal {
    ($signal:ident) => {{
        #[cfg(debug_assertions)]
        let location = std::panic::Location::caller();
        || {
            #[cfg(debug_assertions)]
            {
                panic!(
                    "{}",
                    $crate::signal_traits::panic_getting_disposed_signal(
                        $signal.defined_at(),
                        location
                    )
                );
            }
            #[cfg(not(debug_assertions))]
            {
                panic!(
                    "Tried to access a reactive value that has already been \
                     disposed."
                );
            }
        }
    }};
}

pub trait SignalWithUntracked: DefinedAt {
    type Value;

    #[track_caller]
    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U>;

    #[track_caller]
    fn with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> U {
        self.try_with_untracked(fun)
            .unwrap_or_else(unwrap_signal!(self))
    }
}

pub trait SignalWith: SignalWithUntracked + Track {
    #[track_caller]
    fn try_with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        self.track();
        self.try_with_untracked(fun)
    }

    #[track_caller]
    fn with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> U {
        self.try_with(fun).unwrap_or_else(unwrap_signal!(self))
    }
}

impl<T> SignalWith for T where T: SignalWithUntracked + Track {}

pub trait SignalGetUntracked: SignalWithUntracked
where
    Self::Value: Clone,
{
    fn try_get_untracked(&self) -> Option<Self::Value> {
        self.try_with_untracked(Self::Value::clone)
    }

    #[track_caller]
    fn get_untracked(&self) -> Self::Value {
        self.try_get_untracked()
            .unwrap_or_else(unwrap_signal!(self))
    }
}

impl<T> SignalGetUntracked for T
where
    T: SignalWithUntracked,
    T::Value: Clone,
{
}

pub trait SignalGet: SignalWith
where
    Self::Value: Clone,
{
    fn try_get(&self) -> Option<Self::Value> {
        self.try_with(Self::Value::clone)
    }

    #[track_caller]
    fn get(&self) -> Self::Value {
        self.try_with(Self::Value::clone)
            .unwrap_or_else(unwrap_signal!(self))
    }
}

impl<T> SignalGet for T
where
    T: SignalWith,
    T::Value: Clone,
{
}

pub trait Trigger {
    fn trigger(&self);
}

pub trait SignalUpdateUntracked {
    type Value;

    fn try_update_untracked<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U>;
}

pub trait SignalUpdate {
    type Value;

    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        self.try_update(fun);
    }

    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U>;
}

impl<T> SignalUpdate for T
where
    T: Trigger + SignalUpdateUntracked,
{
    type Value = T::Value;

    fn try_update<U>(
        &self,
        fun: impl FnOnce(&mut Self::Value) -> U,
    ) -> Option<U> {
        let value = self.try_update_untracked(fun);
        self.trigger();
        value
    }
}

pub trait SignalSet: SignalUpdate + SignalIsDisposed {
    fn set(&self, value: impl Into<Self::Value>) {
        self.update(|n| *n = value.into());
    }

    fn try_set(&self, value: impl Into<Self::Value>) -> Option<Self::Value> {
        if self.is_disposed() {
            Some(value.into())
        } else {
            self.set(value);
            None
        }
    }
}

impl<T> SignalSet for T where T: SignalUpdate + SignalIsDisposed {}

pub trait SignalIsDisposed {
    fn is_disposed(&self) -> bool;
}

pub trait DefinedAt {
    fn defined_at(&self) -> Option<&'static Location<'static>>;
}

pub(crate) fn panic_getting_disposed_signal(
    defined_at: Option<&'static Location<'static>>,
    location: &'static Location<'static>,
) -> String {
    if let Some(defined_at) = defined_at {
        format!(
            "At {location}, you tried to access a reactive value which was \
             defined at {defined_at}, but it has already been disposed."
        )
    } else {
        format!(
            "At {location}, you tried to access a reactive value, but it has \
             already been disposed."
        )
    }
}
