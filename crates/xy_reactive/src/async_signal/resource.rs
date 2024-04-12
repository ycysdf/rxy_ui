use super::{ArcAsyncDerived, AsyncDerived, AsyncDerivedFuture, AsyncState};
#[cfg(feature = "miniserde")]
use crate::serialization::Miniserde;
#[cfg(feature = "rkyv")]
use crate::serialization::Rkyv;
#[cfg(feature = "serde-lite")]
use crate::serialization::SerdeLite;
use crate::{
    arena::Owner,
    prelude::SignalWithUntracked,
    serialization::{SerdeJson, SerializableData, Serializer, Str},
    // shared_context::SerializedDataId,
};
use core::{fmt::Debug, marker::PhantomData};
use futures::Future;
use std::{future::IntoFuture, ops::Deref};

pub struct ArcResource<T, Ser> {
    ser: PhantomData<Ser>,
    data: ArcAsyncDerived<T>,
}

impl<T, Ser> Deref for ArcResource<T, Ser> {
    type Target = ArcAsyncDerived<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> ArcResource<T, Str>
    where
        T: SerializableData<Str>,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        ArcResource::new_with_encoding(fun)
    }
}

impl<T> ArcResource<T, SerdeJson>
    where
        T: SerializableData<SerdeJson>,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_serde<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        ArcResource::new_with_encoding(fun)
    }
}

#[cfg(feature = "miniserde")]
impl<T> ArcResource<T, Miniserde>
    where
        T: SerializableData<Miniserde>,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_miniserde<Fut>(
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        ArcResource::new_with_encoding(fun)
    }
}

#[cfg(feature = "serde-lite")]
impl<T> ArcResource<T, SerdeLite>
    where
        T: SerializableData<SerdeLite>,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_serde_lite<Fut>(
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        ArcResource::new_with_encoding(fun)
    }
}

#[cfg(feature = "rkyv")]
impl<T> ArcResource<T, SerdeLite>
    where
        T: SerializableData<SerdeLite>,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_rkyv<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        ArcResource::new_with_encoding(fun)
    }
}

impl<T, Ser> ArcResource<T, Ser>
    where
        Ser: Serializer,
        T: SerializableData<Ser>,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_with_encoding<Fut>(
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> ArcResource<T, Ser>
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        let id = Owner::shared_context()
            .map(|sc| sc.next_id())
            .unwrap_or_default();

        let initial = Self::initial_value(&id);

        let data = ArcAsyncDerived::new_with_initial(initial, fun);

        if let Some(shared_context) = Owner::shared_context() {
            let value = data.clone();
            let ready_fut = data.ready();

            shared_context.write_async(
                id,
                Box::pin(async move {
                    ready_fut.await;
                    value
                        .with_untracked(|data| match &data {
                            AsyncState::Complete(val) => val.ser(),
                            _ => unreachable!(),
                        })
                        .unwrap() // TODO handle
                }),
            );
        }

        ArcResource {
            ser: PhantomData,
            data,
        }
    }

    #[inline(always)]
    fn initial_value(id: &SerializedDataId) -> AsyncState<T> {
        #[cfg(feature = "hydration")]
        {
            if let Some(shared_context) = Owner::shared_context() {
                let value = shared_context.read_data(id);
                if let Some(value) = value {
                    match T::de(&value) {
                        Ok(value) => return AsyncState::Complete(value),
                        Err(e) => {
                            crate::log(&format!(
                                "couldn't deserialize from {value:?}: {e:?}"
                            ));
                        }
                    }
                }
            }
        }
        AsyncState::Loading
    }
}

impl<T, Ser> IntoFuture for ArcResource<T, Ser>
    where
        T: Clone + 'static,
{
    type Output = T;
    type IntoFuture = AsyncDerivedFuture<T>;

    fn into_future(self) -> Self::IntoFuture {
        self.data.into_future()
    }
}

pub struct Resource<T, Ser>
    where
        T: Send + Sync + 'static,
{
    ser: PhantomData<Ser>,
    data: AsyncDerived<T>,
}

impl<T: Send + Sync + 'static, Ser> Copy for Resource<T, Ser> {}

impl<T: Send + Sync + 'static, Ser> Clone for Resource<T, Ser> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, Ser> Deref for Resource<T, Ser>
    where
        T: Send + Sync + 'static,
{
    type Target = AsyncDerived<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Resource<T, Str>
    where
        T: SerializableData<Str> + Send + Sync + 'static,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        Resource::new_with_encoding(fun)
    }
}

impl<T> Resource<T, SerdeJson>
    where
        T: SerializableData<SerdeJson> + Send + Sync + 'static,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_serde<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        Resource::new_with_encoding(fun)
    }
}

#[cfg(feature = "miniserde")]
impl<T> Resource<T, Miniserde>
    where
        T: SerializableData<Miniserde> + Send + Sync + 'static,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_miniserde<Fut>(
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        Resource::new_with_encoding(fun)
    }
}

#[cfg(feature = "serde-lite")]
impl<T> Resource<T, SerdeLite>
    where
        T: SerializableData<SerdeLite> + Send + Sync + 'static,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_serde_lite<Fut>(
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        Resource::new_with_encoding(fun)
    }
}

#[cfg(feature = "rkyv")]
impl<T> Resource<T, Rkyv>
    where
        T: SerializableData<Rkyv> + Send + Sync + 'static,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_rkyv<Fut>(fun: impl Fn() -> Fut + Send + Sync + 'static) -> Self
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        Resource::new_with_encoding(fun)
    }
}

impl<T, Ser> Resource<T, Ser>
    where
        Ser: Serializer,
        T: SerializableData<Ser> + Send + Sync + 'static,
        T::SerErr: Debug,
        T::DeErr: Debug,
{
    pub fn new_with_encoding<Fut>(
        fun: impl Fn() -> Fut + Send + Sync + 'static,
    ) -> Resource<T, Ser>
        where
            T: Send + Sync + 'static,
            Fut: Future<Output=T> + Send + Sync + 'static,
    {
        let id = Owner::shared_context()
            .map(|sc| sc.next_id())
            .unwrap_or_default();

        let initial = Self::initial_value(&id);

        let data = AsyncDerived::new_with_initial(initial, fun);

        if let Some(shared_context) = Owner::shared_context() {
            let value = data;
            let ready_fut = data.ready();

            shared_context.write_async(
                id,
                Box::pin(async move {
                    ready_fut.await;
                    value
                        .with_untracked(|data| match &data {
                            AsyncState::Complete(val) => val.ser(),
                            _ => unreachable!(),
                        })
                        .unwrap() // TODO handle
                }),
            );
        }

        Resource {
            ser: PhantomData,
            data,
        }
    }

    #[inline(always)]
    fn initial_value(id: &SerializedDataId) -> AsyncState<T> {
        #[cfg(feature = "hydration")]
        {
            if let Some(shared_context) = Owner::shared_context() {
                let value = shared_context.read_data(id);
                if let Some(value) = value {
                    match T::de(&value) {
                        Ok(value) => return AsyncState::Complete(value),
                        Err(e) => {
                            crate::log(&format!(
                                "couldn't deserialize from {value:?}: {e:?}"
                            ));
                        }
                    }
                }
            }
        }
        AsyncState::Loading
    }
}

impl<T, Ser> IntoFuture for Resource<T, Ser>
    where
        T: Clone + Send + Sync + 'static,
{
    type Output = T;
    type IntoFuture = AsyncDerivedFuture<T>;

    fn into_future(self) -> Self::IntoFuture {
        self.data.into_future()
    }
}
