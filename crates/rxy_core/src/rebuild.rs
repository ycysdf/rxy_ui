use alloc::boxed::Box;

use bevy_utils::synccell::SyncCell;

use crate::{rebuild_fn, RebuildFn, RebuildFnReceiver, Renderer};

#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(type_path = false)
)]
pub struct ReBuildFn<R, T>(
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))]
    Option<Box<dyn FnMut(&mut <R as Renderer>::NodeTree, T) + Send + 'static>>,
)
where
    R: Renderer;

#[cfg(feature = "bevy_reflect")]
impl<R: Renderer, T: 'static> bevy_reflect::TypePath for ReBuildFn<R, T> {
    fn type_path() -> &'static str {
        "rxy_core::ReBuildFn<R,T>"
    }

    fn short_type_path() -> &'static str {
        "ReBuildFn<R,T>"
    }
}

impl<R, T> ReBuildFn<R, T>
where
    R: Renderer,
{
    pub fn new(f: impl FnMut(&mut <R as Renderer>::NodeTree, T) + Send + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    pub fn call(&mut self, world: &mut <R as Renderer>::NodeTree, vp: T) {
        self.0.as_mut().unwrap()(world, vp);
    }
}

pub fn rebuild_fn_channel<R, T>() -> (ReBuildFn<R, T>, oneshot::Sender<RebuildFn<R, T>>)
where
    R: Renderer,
    T: 'static,
{
    let (sender, receiver) = oneshot::channel();
    let mut receiver = SyncCell::new(receiver);
    let mut cell = once_cell::sync::OnceCell::new();

    // let mut once_lock = core::sync::OnceLock::new();
    let build_fn = ReBuildFn::new(move |world, n| {
        cell.get_or_init(|| receiver.get().try_recv().unwrap());
        let rebuild_fn: &mut RebuildFn<R, T> = cell.get_mut().unwrap();
        (*rebuild_fn)(n, world);
    });
    (build_fn, sender)
}

pub type TargetRebuildFnChannel<R, T> = (ReBuildFn<R, T>, RebuildFnReceiver<R, T>);

pub fn target_rebuild_fn_channel<R, T>(target: Option<T>) -> TargetRebuildFnChannel<R, T>
where
    R: Renderer,
    T: 'static,
{
    let (rebuild_fn1, sender1) = rebuild_fn_channel::<R, T>();
    let receiver1 = rebuild_fn(
        target,
        Box::new(move |f| {
            let _ = sender1.send(f);
        }),
    );
    (rebuild_fn1, receiver1)
}
