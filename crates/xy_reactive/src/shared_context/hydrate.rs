use super::{SerializedDataId, SharedContext};
use crate::{PinnedFuture, PinnedStream};
use core::fmt::Debug;
use js_sys::Array;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    static __RESOLVED_RESOURCES: Array;
}

#[derive(Default)]
pub struct HydrateSharedContext {
    id: AtomicUsize,
}

impl HydrateSharedContext {
    pub fn new() -> Self {
        Self {
            id: AtomicUsize::new(0),
        }
    }
}

impl Debug for HydrateSharedContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HydrateSharedContext").finish()
    }
}

impl SharedContext for HydrateSharedContext {
    fn next_id(&self) -> SerializedDataId {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        SerializedDataId(id)
    }

    fn write_async(&self, _id: SerializedDataId, _fut: PinnedFuture<String>) {}

    fn read_data(&self, id: &SerializedDataId) -> Option<String> {
        __RESOLVED_RESOURCES.get(id.0 as u32).as_string()
    }

    fn await_data(&self, _id: &SerializedDataId) -> Option<String> {
        todo!()
    }

    fn pending_data(&self) -> Option<PinnedStream<String>> {
        None
    }
}
