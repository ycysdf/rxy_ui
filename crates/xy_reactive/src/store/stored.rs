use super::{ArcReadStoreField, ArcRwStoreField, ArcWriteStoreField};
use crate::{
    arena::{Stored, StoredData},
    signal_traits::{DefinedAt, SignalIsDisposed},
    source::Track,
    unwrap_signal,
};
use std::fmt::Debug;

pub struct RwStoreField<Orig: Send + Sync + 'static, T: Send + Sync + 'static> {
    pub(crate) inner: Stored<ArcRwStoreField<Orig, T>>,
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static>
    RwStoreField<Orig, T>
{
    #[inline(always)]
    pub fn read_only(&self) -> ReadStoreField<Orig, T> {
        ReadStoreField {
            inner: Stored::new(
                self.get_value()
                    .map(|inner| inner.read_only())
                    .unwrap_or_else(unwrap_signal!(self)),
            ),
        }
    }

    #[inline(always)]
    pub fn write_only(&self) -> WriteStoreField<Orig, T> {
        WriteStoreField {
            inner: Stored::new(
                self.get_value()
                    .map(|inner| inner.write_only())
                    .unwrap_or_else(unwrap_signal!(self)),
            ),
        }
    }

    #[inline(always)]
    pub fn split(&self) -> (ReadStoreField<Orig, T>, WriteStoreField<Orig, T>) {
        (self.read_only(), self.write_only())
    }

    pub fn unite(
        read: ReadStoreField<Orig, T>,
        write: WriteStoreField<Orig, T>,
    ) -> Option<Self> {
        match (read.inner.get(), write.inner.get()) {
            (Some(read), Some(write)) => ArcRwStoreField::unite(read, write)
                .map(|inner| Self {
                    inner: Stored::new(inner),
                }),
            _ => None,
        }
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Copy
    for RwStoreField<Orig, T>
{
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Clone
    for RwStoreField<Orig, T>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Debug
    for RwStoreField<Orig, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RwStoreField")
            .field("type", &std::any::type_name::<T>())
            .finish()
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> SignalIsDisposed
    for RwStoreField<Orig, T>
{
    fn is_disposed(&self) -> bool {
        self.inner.exists()
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Track
    for RwStoreField<Orig, T>
{
    fn track(&self) {
        if let Some(inner) = self.inner.get() {
            inner.track();
        }
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> StoredData
    for RwStoreField<Orig, T>
{
    type Data = ArcRwStoreField<Orig, T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

pub struct ReadStoreField<Orig: Send + Sync + 'static, T: Send + Sync + 'static>
{
    pub(crate) inner: Stored<ArcReadStoreField<Orig, T>>,
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> StoredData
    for ReadStoreField<Orig, T>
{
    type Data = ArcReadStoreField<Orig, T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Clone
    for ReadStoreField<Orig, T>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Copy
    for ReadStoreField<Orig, T>
{
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Track
    for ReadStoreField<Orig, T>
{
    fn track(&self) {
        if let Some(inner) = self.inner.get() {
            inner.track();
        }
    }
}

pub struct WriteStoreField<
    Orig: Send + Sync + 'static,
    T: Send + Sync + 'static,
> {
    pub(crate) inner: Stored<ArcWriteStoreField<Orig, T>>,
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> StoredData
    for WriteStoreField<Orig, T>
{
    type Data = ArcWriteStoreField<Orig, T>;

    fn get_value(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Clone
    for WriteStoreField<Orig, T>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> Copy
    for WriteStoreField<Orig, T>
{
}

impl<Orig: Send + Sync + 'static, T: Send + Sync + 'static> SignalIsDisposed
    for WriteStoreField<Orig, T>
{
    fn is_disposed(&self) -> bool {
        !self.inner.exists()
    }
}
