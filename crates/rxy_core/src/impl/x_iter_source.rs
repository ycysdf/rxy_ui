use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::vec;
use crate::diff::diff;
use crate::{virtual_container, Either, EitherExt, IntoView, MutableView, MutableViewKey, NodeTree, Renderer, RendererNodeId, RendererWorld, View, ViewCtx, ViewKey, VirtualContainer, MaybeSend};
use alloc::borrow::Cow;
use async_channel::{Receiver, Recv, RecvError, Sender, TryRecvError};
use crate::utils::SyncCell;
use core::fmt::Debug;
use core::future::Future;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::pin::pin;
use futures_lite::stream::Map;
use futures_lite::{FutureExt, StreamExt};
use hooked_collection::{
    ApplyVecOperation, ApplyVecOperationResult, HookedVec, VecOperation, VecOperationRecord,
};

pub enum UseListOperation<T> {
    WatchCount(Sender<usize>),
    Ops(VecOperation<T>),
    Callback(Box<dyn FnOnce(&mut HookedVec<T, VecOperationRecord<T>>) + MaybeSend>),
}

pub trait ListOperator {
    type Item;
    fn push(&self, item: Self::Item);
    fn pop(&self);
    fn insert(&self, index: usize, item: Self::Item);
    fn remove(&self, index: usize);
    fn update(&self, index: usize, item: Self::Item);
    fn clear(&self);
    fn move_item(&self, from: usize, to: usize);
    fn watch_count(&self) -> Receiver<usize>;
    fn callback(
        &self,
        f: impl FnOnce(&mut HookedVec<Self::Item, VecOperationRecord<Self::Item>>) + MaybeSend + 'static,
    );
    fn patch(&self, index: usize, f: impl FnOnce(&mut Self::Item) + MaybeSend + 'static)
    where
        Self::Item: Clone;
}

#[derive(Clone)]
pub struct UseList<T> {
    op_sender: Sender<UseListOperation<T>>,
}

impl<T> ListOperator for UseList<T> {
    type Item = T;

    fn push(&self, item: T) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Ops(VecOperation::Push { item }));
    }

    fn callback(&self, f: impl FnOnce(&mut HookedVec<T, VecOperationRecord<T>>) + MaybeSend + 'static) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Callback(Box::new(f)));
    }

    fn patch(&self, index: usize, f: impl FnOnce(&mut T) + MaybeSend + 'static)
    where
        T: Clone,
    {
        self.callback(move |vec: &mut HookedVec<T, VecOperationRecord<T>>| {
            vec.patch(index, f);
        });
    }

    fn pop(&self) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Ops(VecOperation::Pop));
    }

    fn insert(&self, index: usize, item: T) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Ops(VecOperation::Insert { index, item }));
    }

    fn remove(&self, index: usize) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Ops(VecOperation::Remove { index }));
    }

    fn update(&self, index: usize, item: T) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Ops(VecOperation::Update { index, item }));
    }

    fn clear(&self) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Ops(VecOperation::Clear));
    }

    fn move_item(&self, from: usize, to: usize) {
        let _ = self
            .op_sender
            .send_blocking(UseListOperation::Ops(VecOperation::Move { from, to }));
    }

    fn watch_count(&self) -> Receiver<usize> {
        todo!()
        // let (sender, receiver) = async_channel::unbounded();
        // let _ = self
        //     .op_sender
        //     .send_blocking(UseListOperation::WatchCount(sender));
        // receiver
    }
}

#[allow(dead_code)]
pub struct UseListSource<T> {
    vec: Vec<T>,
    op_receiver: Receiver<UseListOperation<T>>,
    op_handlers: Vec<Box<dyn OperatorHandler<Op = VecOperation<T>>>>,
}
pub trait OperatorHandler: MaybeSend + 'static {
    type Op;
    fn handle(&mut self, op: &Self::Op);
    fn commit(&mut self);
}

pub struct ListCountSender<T> {
    sender: Sender<usize>,
    count: usize,
    _marker: PhantomData<T>,
}

impl<T: Clone + MaybeSend + 'static> OperatorHandler for ListCountSender<T> {
    type Op = VecOperation<T>;

    fn handle(&mut self, op: &Self::Op) {
        match op {
            VecOperation::Push { .. } | VecOperation::Insert { .. } => {
                self.count += 1;
            }
            VecOperation::Pop | VecOperation::Remove { .. } => {
                self.count += 1;
            }
            VecOperation::Clear => {
                self.count = 0;
            }
            _ => {}
        }
    }

    fn commit(&mut self) {
        let _ = self.sender.send_blocking(self.count);
    }
}

impl<T: Clone + MaybeSend + 'static> UseListSource<T> {
    // #[inline(always)]
    // pub fn add_op_handler(&mut self, op_handler: impl OperatorHandler<Op = VecOperation<T>>) {
    //     self.op_handlers.push(Box::new(op_handler));
    // }

    // #[inline(always)]
    // pub fn handle_handlers(&mut self, op: &VecOperation<T>) {
    //     for op_handler in self.op_handlers.iter_mut() {
    //         op_handler.handle(op);
    //     }
    // }

    // #[inline(always)]
    // pub fn commit_handlers(&mut self) {
    //     for op_handler in self.op_handlers.iter_mut() {
    //         op_handler.commit();
    //     }
    // }

    // pub async fn handle_op(&mut self, op: UseListOperation<T>) -> Option<VecOperation<T>> {
    //     match op {
    //         UseListOperation::Ops(op) => {
    //             self.handle_handlers(&op);
    //             Some(op)
    //         }
    //         UseListOperation::WatchCount(_sender) => {
    //             // let count = self.vec.len();
    //             // let _ = sender.send(count).await;
    //             // self.add_op_handler(ListCountSender {
    //             //     sender,
    //             //     count,
    //             //     _marker: PhantomData,
    //             // });
    //             None
    //         }
    //         UseListOperation::Callback(_f) => None,
    //     }
    // }
    // pub async fn handle(
    //     &mut self,
    //     mut f: impl FnMut(Vec<VecOperation<T>>),
    // ) -> Result<(), RecvError> {
    //     loop {
    //         let op = self.op_receiver.recv().await?;
    //         let mut ops = vec![];
    //         if let Some(op) = self.handle_op(op).await {
    //             ops.push(op);
    //         }
    //         while let Ok(op) = self.op_receiver.try_recv() {
    //             if let Some(op) = self.handle_op(op).await {
    //                 ops.push(op);
    //             }
    //         }
    //         self.commit_handlers();
    //         f(ops);
    //     }
    // }
    pub fn try_apply_ops(&mut self) {
        while let Ok(op) = self.op_receiver.try_recv() {
            // todo:
            #[allow(clippy::single_match)]
            match op {
                UseListOperation::Ops(op) => {
                    self.vec.apply_op(op);
                }
                _ => {}
            }
        }
    }
}

pub fn use_list<T>(init: impl IntoIterator<Item = T>) -> (UseList<T>, UseListSource<T>) {
    let (op_sender, op_receiver) = async_channel::unbounded();
    let vec = init.into_iter().collect::<Vec<_>>();
    (
        UseList { op_sender },
        UseListSource {
            vec,
            op_receiver,
            op_handlers: vec![],
        },
    )
}

pub fn x_iter_source<R, T, F, IV>(source: UseListSource<T>, view_f: F) -> ForSource<T, F>
where
    R: Renderer,
    T: Clone + MaybeSend + 'static,
    IV: IntoView<R>,
    F: Fn(Cow<T>) -> IV + Clone + MaybeSend + 'static,
{
    ForSource {
        source,
        view_f,
        _marker: PhantomData,
    }
}

pub struct ForSource<T, F> {
    source: UseListSource<T>,
    view_f: F,
    _marker: PhantomData<T>,
}

impl<R, T, F, IV> IntoView<R> for ForSource<T, F>
where
    R: Renderer,
    T: Clone + Debug + MaybeSend + 'static,
    IV: IntoView<R>,
    F: Fn(Cow<T>) -> IV + Clone + MaybeSend + 'static,
{
    type View = VirtualContainer<R, Self>;

    fn into_view(self) -> Self::View {
        virtual_container(self, "[ForSource Placeholder]")
    }
}

pub struct ForSourceState<R, K>
where
    R: Renderer,
{
    view_keys: Vec<K>,
    task: SyncCell<R::Task<()>>,
}

#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Debug)]
pub struct ForSourceViewKey<R, K>(
    DataOrPlaceholderNodeId<R>,
    #[cfg_attr(feature = "bevy_reflect", reflect(ignore))] PhantomData<K>,
)
where
    R: Renderer,
    K: ViewKey<R>;

impl<R, K> ForSourceViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    pub fn new(state_node_id: DataOrPlaceholderNodeId<R>) -> Self {
        Self(state_node_id, Default::default())
    }
}

pub fn get_for_source_view_keys_scoped<R, K, U>(
    world: &mut RendererWorld<R>,
    state_node_id: &RendererNodeId<R>,
    f: impl for<'a, 'b> FnOnce(&'a mut Vec<K>, &'b mut RendererWorld<R>) -> U,
) -> U
where
    R: Renderer,
    K: ViewKey<R>,
{
    let mut view_keys = core::mem::take(
        &mut world
            .get_node_state_mut::<ForSourceState<R, K>>(state_node_id)
            .unwrap()
            .view_keys,
    );
    let r = f(&mut view_keys, world);
    world
        .get_node_state_mut::<ForSourceState<R, K>>(state_node_id)
        .unwrap()
        .view_keys = view_keys;
    r
}

impl<R, K> MutableViewKey<R> for ForSourceViewKey<R, K>
where
    R: Renderer,
    K: ViewKey<R>,
{
    fn remove(self, world: &mut RendererWorld<R>) {
        let state = world
            .take_node_state::<ForSourceState<R, K>>(self.0.state_node_id())
            .unwrap();
        drop(state.task);
        for key in state.view_keys {
            key.remove(world);
        }
        if let DataOrPlaceholderNodeId::Data(state_node_id) = self.0 {
            world.remove_node(&state_node_id);
        }
    }

    fn insert_before(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        before_node_id: Option<&RendererNodeId<R>>,
    ) {
        get_for_source_view_keys_scoped(
            world,
            self.0.state_node_id(),
            |view_keys: &mut Vec<K>, world| {
                for key in view_keys.iter() {
                    key.insert_before(world, parent, before_node_id);
                }
            },
        );
    }

    fn set_visibility(&self, world: &mut RendererWorld<R>, hidden: bool) {
        get_for_source_view_keys_scoped(
            world,
            self.0.state_node_id(),
            |view_keys: &mut Vec<K>, world| {
                for key in view_keys.iter() {
                    key.set_visibility(world, hidden);
                }
            },
        );
    }

    fn first_node_id(&self, world: &RendererWorld<R>) -> Option<RendererNodeId<R>> {
        world
            .get_node_state_ref::<ForSourceState<R, K>>(self.0.state_node_id())
            .unwrap()
            .view_keys
            .first()
            .and_then(|n| n.first_node_id(world))
    }

    fn state_node_id(&self) -> Option<RendererNodeId<R>> {
        Some(self.0.state_node_id().clone())
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum DataOrPlaceholderNodeId<R>
where
    R: Renderer,
{
    Data(RendererNodeId<R>),
    Placeholder(RendererNodeId<R>),
}

impl<R> DataOrPlaceholderNodeId<R>
where
    R: Renderer,
{
    pub fn state_node_id(&self) -> &RendererNodeId<R> {
        match self {
            DataOrPlaceholderNodeId::Data(node_id) => node_id,
            DataOrPlaceholderNodeId::Placeholder(node_id) => node_id,
        }
    }
}

pub fn build_for_source<R, T, F, IV>(
    mut for_source: ForSource<T, F>,
    ctx: ViewCtx<R>,
    state_node_id: Option<RendererNodeId<R>>,
) -> DataOrPlaceholderNodeId<R>
where
    R: Renderer,
    T: Clone + MaybeSend + Debug + 'static,
    IV: IntoView<R>,
    F: Fn(Cow<T>) -> IV + Clone + MaybeSend + 'static,
{
    for_source.source.try_apply_ops();

    let UseListSource {
        vec, op_receiver, ..
    } = for_source.source;

    let view_keys = vec
        .iter()
        .map(|n| {
            let view = (for_source.view_f)(Cow::Borrowed(n)).into_view();
            view.build(
                ViewCtx {
                    world: &mut *ctx.world,
                    parent: ctx.parent.clone(),
                },
                None,
                true,
            )
        })
        .collect::<Vec<_>>();

    let world_scoped = ctx.world.deferred_world_scoped();
    let state_node_id = if let Some(state_node_id) = state_node_id {
        DataOrPlaceholderNodeId::Placeholder(state_node_id)
    } else {
        DataOrPlaceholderNodeId::Data(ctx.world.spawn_data_node())
    };

    let task = R::spawn_task({
        use crate::renderer::DeferredNodeTreeScoped;
        let parent = ctx.parent;
        let view_f = for_source.view_f;
        let state_node_id = state_node_id.clone();
        async move {
            let mut ops = vec![];

            fn handle_op<T>(op: UseListOperation<T>) -> Option<UseListOperation<T>> {
                match op {
                    UseListOperation::Ops(op) => Some(UseListOperation::Ops(op)),
                    UseListOperation::WatchCount(_sender) => {
                        // let count = self.vec.len();
                        // let _ = sender.send(count).await;
                        // self.add_op_handler(ListCountSender {
                        //     sender,
                        //     count,
                        //     _marker: PhantomData,
                        // });
                        None
                    }
                    UseListOperation::Callback(f) => Some(UseListOperation::Callback(f)),
                }
            }

            let mut vec_or_receiver = Some(vec.either_left());

            'r: loop {
                let Ok(op) = op_receiver.recv().await else {
                    break 'r;
                };
                if let Some(op) = handle_op(op) {
                    ops.push(op);
                }
                loop {
                    match op_receiver.try_recv() {
                        Ok(op) => {
                            if let Some(op) = handle_op(op) {
                                ops.push(op);
                            }
                        }
                        Err(TryRecvError::Closed) => {
                            break 'r;
                        }
                        Err(TryRecvError::Empty) => {
                            break;
                        }
                    }
                }
                let parent = parent.clone();
                let (sender, receiver) = oneshot::channel();
                let mut taken_vec_or_receiver = vec_or_receiver.take();
                vec_or_receiver = Some(receiver.either_right());

                world_scoped.scoped({
                    let ops = core::mem::take(&mut ops);
                    let view_f = view_f.clone();
                    let state_node_id = state_node_id.clone();
                    move |world| {
                        get_for_source_view_keys_scoped(
                            world,
                            state_node_id.state_node_id(),
                            |view_keys: &mut Vec<<IV::View as View<R>>::Key>, world| {
                                let mut vec = taken_vec_or_receiver
                                    .take()
                                    .unwrap()
                                    .map_right(|n| n.try_recv().unwrap())
                                    .into_inner();
                                for op in ops {
                                    match op {
                                        UseListOperation::WatchCount(_) => {}
                                        UseListOperation::Ops(op) => {
                                            apply_op_to_view_keys(
                                                world,
                                                parent.clone(),
                                                view_f.clone(),
                                                op.as_ref().map(|n| Cow::Borrowed(n)),
                                                &state_node_id,
                                                view_keys,
                                                &vec,
                                            );
                                            vec.apply_op(op);
                                        }
                                        UseListOperation::Callback(f) => {
                                            let mut hooked_vec = HookedVec::from_vec(
                                                core::mem::take(&mut vec),
                                                VecOperationRecord::new(),
                                            );
                                            f(&mut hooked_vec);
                                            let (mut vec_result, record) = hooked_vec.into_inner();
                                            for op in record {
                                                apply_op_to_view_keys(
                                                    world,
                                                    parent.clone(),
                                                    view_f.clone(),
                                                    op.map(|n| Cow::Owned(n)),
                                                    &state_node_id,
                                                    view_keys,
                                                    &vec_result,
                                                );
                                            }
                                            core::mem::swap(&mut vec_result, &mut vec);
                                        }
                                    }
                                }
                                sender.send(vec).unwrap();
                            },
                        );
                    }
                });
            }
        }
    });
    ctx.world.set_node_state(
        state_node_id.state_node_id(),
        ForSourceState::<R, _> {
            view_keys,
            task: SyncCell::new(task),
        },
    );
    state_node_id
}

fn apply_op_to_view_keys<R, T, F, IV>(
    world: &mut RendererWorld<R>,
    parent: RendererNodeId<R>,
    view_f: F,
    op: VecOperation<Cow<T>>,
    state_node_id: &DataOrPlaceholderNodeId<R>,
    view_keys: &mut Vec<<IV::View as View<R>>::Key>,
    vec: &[T],
) where
    R: Renderer,
    T: Clone + MaybeSend + Debug + 'static,
    IV: IntoView<R>,
    F: Fn(Cow<T>) -> IV + Clone + MaybeSend + 'static,
{
    match op {
        VecOperation::Push { item } => {
            let view = view_f(item).into_view();
            let view_key = view.build(
                ViewCtx {
                    world,
                    parent: parent.clone(),
                },
                None,
                true,
            );
            match state_node_id {
                DataOrPlaceholderNodeId::Data(_state_node_id) => unreachable!(),
                DataOrPlaceholderNodeId::Placeholder(placeholder_node_id) => {
                    view_key.insert_before(world, Some(&parent), Some(placeholder_node_id));
                }
            }

            view_keys.push(view_key);
        }
        VecOperation::Pop => {
            if let Some(view_key) = view_keys.pop() {
                view_key.remove(world);
            }
        }
        VecOperation::Insert { index, item } => {
            let view = view_f(item).into_view();
            let view_key = view.build(
                ViewCtx {
                    world,
                    parent: parent.clone(),
                },
                None,
                true,
            );

            match state_node_id {
                DataOrPlaceholderNodeId::Data(_state_node_id) => unreachable!(),
                DataOrPlaceholderNodeId::Placeholder(state_node_id) => {
                    let first_node_id = view_keys[index]
                        .first_node_id(world)
                        .unwrap_or(state_node_id.clone());
                    view_key.insert_before(world, Some(&parent), Some(&first_node_id));
                }
            }
            view_keys.insert(index, view_key);
        }
        VecOperation::Update { index, item } => {
            let view = view_f(item).into_view();
            view.rebuild(
                ViewCtx {
                    world,
                    parent: parent.clone(),
                },
                view_keys[index].clone(),
            );
        }
        VecOperation::Remove { index } => {
            view_keys.remove(index).remove(world);
        }
        VecOperation::Clear => {
            for view_key in view_keys.drain(..) {
                view_key.remove(world);
            }
        }
        VecOperation::Move { from, to } => {
            let before_node_id = view_keys[to].first_node_id(world).unwrap();
            view_keys[from].insert_before(world, Some(&parent), Some(&before_node_id));

            let view_key = view_keys.remove(from);
            if from < to {
                view_keys.insert(to - 1, view_key);
            } else {
                view_keys.insert(to, view_key);
            }
        }
        VecOperation::Patch { index } => {
            let view = view_f(Cow::Borrowed(&vec[index])).into_view();
            view.rebuild(
                ViewCtx {
                    world,
                    parent: parent.clone(),
                },
                view_keys[index].clone(),
            );
        }
    }
}

impl<R, T, F, IV> MutableView<R> for ForSource<T, F>
where
    R: Renderer,
    T: Clone + MaybeSend + Debug + 'static,
    IV: IntoView<R>,
    F: Fn(Cow<T>) -> IV + Clone + MaybeSend + 'static,
{
    type Key = ForSourceViewKey<R, <IV::View as View<R>>::Key>;

    fn no_placeholder_when_no_rebuild() -> bool {
        false
    }

    fn build(self, ctx: ViewCtx<R>, placeholder_node_id: Option<RendererNodeId<R>>) -> Self::Key {
        // because no_placeholder_when_no_rebuild is false. placeholder_node_id must be some
        assert!(placeholder_node_id.is_some());
        let state_node_id = build_for_source(self, ctx, placeholder_node_id);
        ForSourceViewKey::new(state_node_id)
    }

    fn rebuild(
        self,
        ctx: ViewCtx<R>,
        _key: Self::Key,
        placeholder_node_id: RendererNodeId<R>,
    ) -> Option<Self::Key> {
        assert!(matches!(_key.0, DataOrPlaceholderNodeId::Placeholder(_)));
        let view_keys = if let Some(state) = ctx
            .world
            .take_node_state::<ForSourceState<R, <IV::View as View<R>>::Key>>(&placeholder_node_id)
        {
            drop(state.task);
            state.view_keys
        } else {
            vec![]
        };
        // todo: Can be optimized
        for view_key in view_keys {
            view_key.remove(&mut *ctx.world);
        }

        let state_node_id = build_for_source(self, ctx, Some(placeholder_node_id));
        Some(ForSourceViewKey::new(state_node_id))
    }
}
