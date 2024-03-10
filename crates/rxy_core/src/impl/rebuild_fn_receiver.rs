use alloc::boxed::Box;

use crate::utils::all_tuples;

use crate::{
    target_rebuild_fn_channel, IntoView, MaybeSend, Renderer, RendererNodeId, View, ViewCtx,
    ViewKey, ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin, XNest,
};

#[cfg(feature = "send_sync")]
pub type RebuildFn<R, T> = Box<dyn FnMut(T, &mut <R as Renderer>::NodeTree) + MaybeSend + 'static>;

#[cfg(feature = "send_sync")]
pub type RebuildFnMaybeSender<R, T> = Box<dyn FnOnce(RebuildFn<R, T>) + MaybeSend + 'static>;

#[cfg(not(feature = "send_sync"))]
pub type RebuildFn<R, T> = Box<dyn FnMut(T, &mut <R as Renderer>::NodeTree) + 'static>;

#[cfg(not(feature = "send_sync"))]
pub type RebuildFnMaybeSender<R, T> = Box<dyn FnOnce(RebuildFn<R, T>) + 'static>;

pub trait RebuildFnReceiverSplit<R, T, FU, RU> {
    fn split(self, map_f: impl Fn(T) -> FU + Clone + MaybeSend + 'static) -> RU;
}

// impl<R, T, U1> RebuildFnReceiverSplit<R, T, (U1,), (RebuildFnReceiver<R, U1>,)>
//     for RebuildFnReceiver<R, T>
// where
//     R: Renderer,
//     U1: 'static,
// {
//     fn split(
//         self,
//         map_f: impl Fn(T) -> (U1,) + Clone + MaybeSend + 'static,
//     ) -> (RebuildFnReceiver<R, U1>,) {
//         let rebuild_fn_sender_fn = self.1;
//         let (u1,) = map_f(self.0);
//         let (mut rebuild_fn1, receiver1) = target_rebuild_fn_channel(u1);
//
//         rebuild_fn_sender_fn(Box::new(move |iv, world| {
//             let (u1,) = map_f(iv);
//             rebuild_fn1.call(world, u1);
//         }));
//         (receiver1,)
//     }
// }
// impl<R, T, U1, U2> RebuildFnReceiverSplit<R, T, (U1, U2), (RebuildFnReceiver<R, U1>, RebuildFnReceiver<R, U2>,)>
//     for RebuildFnReceiver<R, T>
// where
//     R: Renderer,
//     U1: 'static,
//     U2: 'static,
// {
//     fn split(self, map_f: impl Fn(T) -> (U1, U2) + Clone + MaybeSend + 'static) -> (RebuildFnReceiver<R, U1>, RebuildFnReceiver<R, U2>) {
//         let rebuild_fn_sender_fn = self.1;
//         let (u1, u2) = map_f(self.0);
//         let (mut rebuild_fn1, receiver1) = target_rebuild_fn_channel(u1);
//         let (mut rebuild_fn2, receiver2) = target_rebuild_fn_channel(u2);
//
//         rebuild_fn_sender_fn(Box::new(move |iv, world| {
//             let (u1, u2) = map_f(iv);
//             rebuild_fn1.call(world, u1);
//             rebuild_fn2.call(world, u2);
//         }));
//         (receiver1, receiver2)
//     }
// }

// macro_rules! impl_split {
//     ($($U:ident),*) => {
//         #[allow(non_snake_case)]
//         impl<R, T, $($U),*> RebuildFnReceiverSplit<R, T, ($($U,)*), ($(RebuildFnReceiver<R, $U>,)*)> for RebuildFnReceiver<R, T>
//         where
//             R: Renderer,
//             $($U: 'static),*
//         {
//             fn split(self, map_f: impl Fn(T) -> ($($U,)*) + Clone + MaybeSend + 'static) -> ($(RebuildFnReceiver<R, $U>,)*) {
//                 let rebuild_fn_sender_fn = self.1;
//                 let ($($U,)*) = match self.0.map(&map_f) {
//                     Some(($($U,)*)) => ($(Some($U),)*),
//                     None => (($({
//                         let $U = None;
//                         $U
//                     },)*)),
//                 };
//                 paste::paste! {
//                     $(
//                     let (mut [<rebuild_fn $U>], [<receiver $U>]) = target_rebuild_fn_channel($U);
//                     )*
//
//                     rebuild_fn_sender_fn(Box::new(move |iv, world| {
//                         let ($($U,)*) = map_f(iv);
//                         $(
//                         [<rebuild_fn $U>].call(world, $U);
//                         )*
//                     }));
//                     ($([<receiver $U>],)*)
//                 }
//             }
//         }
//     };
//     () => {};
// }

// all_tuples!(impl_split, 1, 8, U);

// pub struct EachMapWrapper<T, M>(T, PhantomData<M>);
// // impl Fn(T) -> (U1, U2) + Clone + MaybeSend + 'static
// impl<R, T, U1> EachMapRebuildFnReceiver<R> for EachMapWrapper<T, (U1,)>
// where
//     R: Renderer,
// {
//     type Output = (TargetRebuildFnChannel<R, U1>,);
//
//     fn each_map(self, f: impl FnOnce(T) -> (U1,)) -> Self::Output {
//         let (u1,) = f(self);
//         (target_rebuild_fn_channel(u1),)
//     }
// }
// impl<R, T1, T2> EachMapRebuildFnReceiver<R> for (Option<T1>, Option<T2>)
// where
//     R: Renderer,
// {
//     type Output = (TargetRebuildFnChannel<R, T1>, TargetRebuildFnChannel<R, T2>);
//
//     fn each_map(self, f: impl FnOnce(Self) -> Self::Output) -> Self::Output {
//         let (u1, u2) = self;
//         (target_rebuild_fn_channel(u1), target_rebuild_fn_channel(u2))
//     }
// }

pub struct RebuildFnReceiver<R, T>(pub Option<T>, pub RebuildFnMaybeSender<R, T>)
where
    R: Renderer,
    T: 'static;

impl<R, T> RebuildFnReceiver<R, T>
where
    R: Renderer,
    T: 'static,
{
    pub fn default_value(self, value: T) -> Self {
        Self(Some(value), self.1)
    }

    pub fn map<U>(self, map_f: impl Fn(T) -> U + MaybeSend + 'static) -> RebuildFnReceiver<R, U> {
        let rebuild_fn_sender_fn = self.1;
        RebuildFnReceiver(
            self.0.map(&map_f),
            Box::new(move |mut f| {
                rebuild_fn_sender_fn(Box::new(move |iv, world| f(map_f(iv), world)))
            }),
        )
    }

    // pub fn split<U1, U2>(
    //     self,
    //     map_f: impl Fn(T) -> (U1, U2) + Clone + MaybeSend + 'static,
    // ) -> (RebuildFnReceiver<R, U1>, RebuildFnReceiver<R, U2>) {
    //     let rebuild_fn_sender_fn = self.1;
    //
    //     let (ui, u2) = self.0.map(|n| map_f(n)).unzip();
    //     let (mut rebuild_fn1, sender1) = rebuild_fn_channel::<R, U1>();
    //     let (mut rebuild_fn2, sender2) = rebuild_fn_channel::<R, U2>();
    //
    //     let receiver1 = crate::rebuild_fn(
    //         ui,
    //         Box::new(move |f| {
    //             let _ = sender1.send(f);
    //         }),
    //     );
    //     let receiver2 = crate::rebuild_fn(
    //         u2,
    //         Box::new(move |f| {
    //             let _ = sender2.send(f);
    //         }),
    //     );
    //     rebuild_fn_sender_fn(Box::new(move |iv, world| {
    //         let (u1, u2) = map_f(iv);
    //         rebuild_fn1.call(world, u1);
    //         rebuild_fn2.call(world, u2);
    //     }));
    //
    //     (receiver1, receiver2)
    // }
}

impl<R, T> RebuildFnReceiver<R, T>
where
    R: Renderer,
    T: 'static,
{
    pub fn send_view_member_rebuild_fn(
        f: RebuildFnMaybeSender<R, T>,
        node_id: RendererNodeId<R>,
        index: ViewMemberIndex,
        is_build: bool,
    ) where
        T: ViewMember<R>,
    {
        f({
            let mut is_build = is_build;
            Box::new(move |vm, world| {
                if is_build {
                    vm.rebuild(ViewMemberCtx {
                        index,
                        world,
                        node_id: node_id.clone(),
                    });
                } else {
                    vm.build(
                        ViewMemberCtx {
                            index,
                            world,
                            node_id: node_id.clone(),
                        },
                        true,
                    );
                    is_build = true;
                }
            })
        });
    }
    pub fn send_view_rebuild_fn(
        f: RebuildFnMaybeSender<R, T>,
        key: T::Key,
        parent: RendererNodeId<R>,
        is_build: bool,
    ) where
        T: View<R>,
    {
        f({
            let mut is_build = is_build;
            Box::new(move |view, world| {
                if is_build {
                    view.rebuild(
                        ViewCtx {
                            world,
                            parent: parent.clone(),
                        },
                        key.clone(),
                    );
                } else {
                    view.build(
                        ViewCtx {
                            world,
                            parent: parent.clone(),
                        },
                        Some(key.clone()),
                        true,
                    );
                    is_build = true;
                }
            })
        });
    }
}

// todo:
// impl<R, VM, IVM> XNest<R> for RebuildFnReceiver<R, IVM>
// where
//     R: Renderer,
//     VM: ViewMember<R>,
//     IVM: XNest<R, MapInner=VM>,
// {
//     type MapInner = RebuildFnReceiver<R, VM>;
//
//     fn into_member(self) -> RebuildFnReceiver<R, VM> {
//         self.map(|n| n.into_member())
//     }
// }

// impl<R, VM> ViewMemberOrigin<R> for RebuildFnReceiver<R, VM>
// where
//     R: Renderer,
//     VM: ViewMemberOrigin<R>,
// {
//     type Origin = VM::Origin;
// }

impl<R, VM> ViewMember<R> for RebuildFnReceiver<R, VM>
where
    R: Renderer,
    VM: ViewMember<R>,
{
    fn count() -> ViewMemberIndex {
        VM::count()
    }

    fn unbuild(ctx: ViewMemberCtx<R>, view_removed: bool) {
        VM::unbuild(ctx, view_removed)
    }

    #[inline]
    fn build(self, ctx: ViewMemberCtx<R>, _will_rebuild: bool) {
        let node_id = ctx.node_id.clone();
        let index = ctx.index;
        let is_build = match self.0 {
            None => false,
            Some(vm) => {
                vm.build(ctx, true);
                true
            }
        };
        Self::send_view_member_rebuild_fn(self.1, node_id, index, is_build);
    }

    #[inline]
    fn rebuild(self, _ctx: ViewMemberCtx<R>) {
        unreachable!()
        // Self::send_view_member_rebuild_fn(self.1, ctx.node_id.clone());
        // self.0.build(ctx, true)
    }
}

impl<R, V> View<R> for RebuildFnReceiver<R, V>
where
    R: Renderer,
    V: View<R>,
{
    type Key = V::Key;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let parent = ctx.parent.clone();
        let (key, is_build) = match self.0 {
            None => (
                V::Key::reserve_key(&mut *ctx.world, will_rebuild, ctx.parent.clone(), true),
                false,
            ),
            Some(view) => (view.build(ctx, reserve_key, true), true),
        };

        Self::send_view_rebuild_fn(self.1, key.clone(), parent, is_build);
        key
    }

    fn rebuild(self, _ctx: ViewCtx<R>, _key: Self::Key) {
        unreachable!()
        // let parent = ctx.parent.clone();
        // self.0.rebuild(ctx, key.clone());
        //
        // Self::send_view_rebuild_fn(self.1, key, parent);
    }
}

impl<R, IV> IntoView<R> for RebuildFnReceiver<R, IV>
where
    R: Renderer,
    IV: IntoView<R> + MaybeSend,
{
    type View = RebuildFnReceiver<R, IV::View>;

    fn into_view(self) -> Self::View {
        self.map(|n| n.into_view())
    }
}

pub fn rebuild_fn<R: Renderer, T>(
    target: Option<T>,
    sender: RebuildFnMaybeSender<R, T>,
) -> RebuildFnReceiver<R, T> {
    RebuildFnReceiver(target, sender)
}
