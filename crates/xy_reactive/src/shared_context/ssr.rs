use super::{SerializedDataId, SharedContext};
use crate::{PinnedFuture, PinnedStream};
use futures::{
    stream::{self, FuturesUnordered},
    StreamExt,
};
use parking_lot::RwLock;
use std::{
    fmt::{Debug, Write},
    mem,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
pub struct SsrSharedContext {
    id: AtomicUsize,
    sync_buf: RwLock<Vec<ResolvedData>>,
    async_buf: RwLock<Vec<(SerializedDataId, PinnedFuture<String>)>>,
}

impl SsrSharedContext {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Debug for SsrSharedContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SsrSharedContext")
            .field("id", &self.id)
            .field("sync_buf", &self.sync_buf)
            .field("async_buf", &self.async_buf.read().len())
            .finish()
    }
}

impl SharedContext for SsrSharedContext {
    fn next_id(&self) -> SerializedDataId {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        SerializedDataId(id)
    }

    fn write_async(&self, id: SerializedDataId, fut: PinnedFuture<String>) {
        self.async_buf.write().push((id, fut))
    }

    fn pending_data(&self) -> Option<PinnedStream<String>> {
        let sync_data = mem::take(&mut *self.sync_buf.write());
        let async_data = mem::take(&mut *self.async_buf.write());

        // 1) initial, synchronous setup chunk
        let mut initial_chunk = String::new();
        // resolved synchronous resources
        initial_chunk.push_str("__RESOLVED_RESOURCES=[");
        for resolved in sync_data {
            resolved.write_to_buf(&mut initial_chunk);
            initial_chunk.push(',');
        }
        initial_chunk.push_str("];");

        // pending async resources
        initial_chunk.push_str("__PENDING_RESOURCES=[");
        for (id, _) in &async_data {
            write!(&mut initial_chunk, "{},", id.0).unwrap();
        }
        initial_chunk.push_str("];");

        // resolvers
        initial_chunk.push_str("__RESOURCE_RESOLVERS=[];");

        // 2) async resources as they resolve
        let async_data = async_data
            .into_iter()
            .map(|(id, data)| async move {
                let data = data.await;
                format!("__RESOLVED_RESOURCES[{}] = {data:?};", id.0)
            })
            .collect::<FuturesUnordered<_>>();

        let stream =
            stream::once(async move { initial_chunk }).chain(async_data);
        Some(Box::pin(stream))
    }

    fn read_data(&self, _id: &SerializedDataId) -> Option<String> {
        None
    }

    fn await_data(&self, _id: &SerializedDataId) -> Option<String> {
        None
    }
}

#[derive(Debug)]
struct ResolvedData(SerializedDataId, String);

impl ResolvedData {
    pub fn write_to_buf(&self, buf: &mut String) {
        let ResolvedData(id, ser) = self;
        // escapes < to prevent it being interpreted as another opening HTML tag
        let ser = ser.replace('<', "\\u003c");
        write!(buf, "{}: {:?}", id.0, ser).unwrap();
    }
}
