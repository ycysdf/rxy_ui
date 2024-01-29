#[cfg(feature = "web")]
mod hydrate;
mod islands;
mod ssr;
use crate::{PinnedFuture, PinnedStream};
#[cfg(feature = "web")]
pub use hydrate::*;
pub use islands::*;
use serde::{Deserialize, Serialize};
pub use ssr::*;
use std::fmt::Debug;

pub trait SharedContext: Debug {
    /// Returns the next in a series of IDs that is unique to a particular request and response.
    ///
    /// This should not be used as a global unique ID mechanism. It is specific to the process
    /// of serializing and deserializing data from the server to the browser as part of an HTTP
    /// response.
    fn next_id(&self) -> SerializedDataId;

    /// The given [`Future`] should resolve with some data that can be serialized
    /// from the server to the client. This will be polled as part of the process of
    /// building the HTTP response, *not* when it is first created.
    ///
    /// In browser implementations, this should be a no-op.
    fn write_async(&self, id: SerializedDataId, fut: PinnedFuture<String>);

    /// Reads the current value of some data from the shared context, if it has been
    /// sent from the server. This returns the serialized data as a `String` that should
    /// be deserialized using [`Serializable::de`].
    ///
    /// On the server and in client-side rendered implementations, this should
    /// always return [`None`].
    fn read_data(&self, id: &SerializedDataId) -> Option<String>;

    /// Returns a [`Future`] that resolves with a `String` that should
    /// be deserialized using [`Serializable::de`] once the given piece of server
    /// data has resolved.
    ///
    /// On the server and in client-side rendered implementations, this should
    /// return a [`Future`] that is immediately ready with [`None`].
    fn await_data(&self, id: &SerializedDataId) -> Option<String>;

    /// Returns some [`Stream`] of HTML that contains JavaScript `<script>` tags defining
    /// all values being serialized from the server to the client, with their serialized values
    /// and any boilerplate needed to notify a running application that they exist; or `None`.
    ///
    /// In browser implementations, this return `None`.
    fn pending_data(&self) -> Option<PinnedStream<String>>;
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, Default, Deserialize, Serialize,
)]
#[serde(transparent)]
pub struct SerializedDataId(usize);

/*
enum SerializableData {
    Sync(Box<dyn FnOnce() + Send + Sync>),
    Async(Pin<Box<dyn Future<Output = String> + Send + Sync>>),
}

#[derive(Debug)]
pub struct ServerData<T> {
    id: SerializedDataId,
    data: T,
}

impl<T: Serializable> ServerData<T> {
    pub fn new(data: T) -> Self {
        let id = Owner::shared_context().map(|n| n.id()).unwrap_or_default();
        Self { id, data }
    }
}
*/
