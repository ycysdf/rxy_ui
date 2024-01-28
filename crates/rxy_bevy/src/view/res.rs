use core::marker::PhantomData;

use bevy_ecs::prelude::Entity;
use bevy_ecs::system::Resource;
use rxy_bevy_macro::BevyIntoView;
use rxy_core::{DeferredWorldScoped, Renderer, View, ViewCtx, ViewKey};

use crate::{BevyRenderer, ResChangeWorldExt};
use rxy_core::IntoView;

#[derive(BevyIntoView)]
pub struct XRes<T, F, V>
where
    T: Resource,
    F: Fn(&T) -> V + Clone + Send + Sync + 'static,
    V: View<BevyRenderer>,
{
    pub f: F,
    _marker: PhantomData<T>,
}

pub struct XResViewState {
    #[allow(dead_code)]
    task: <BevyRenderer as Renderer>::Task<()>,
}

fn x_res_build<T, F, V>(
    res: XRes<T, F, V>,
    key: V::Key,
    state_node_id: &Entity,
    ctx: ViewCtx<BevyRenderer>,
) where
    T: Resource,
    F: Fn(&T) -> V + Clone + Send + Sync + 'static,
    V: View<BevyRenderer>,
{
    let deferred_world_scoped = BevyRenderer::deferred_world_scoped(ctx.world);

    let task = BevyRenderer::spawn({
        let res_change_receiver = ctx.world.get_res_change_receiver::<T>();
        let parent = ctx.parent;
        let f = res.f;
        let key = key.clone();
        async move {
            while let Ok(()) = res_change_receiver.recv().await {
                let f = f.clone();
                let key = key.clone();
                deferred_world_scoped.deferred_world(move |world| {
                    let resource = world.resource::<T>();
                    let view = f(resource);
                    view.rebuild(ViewCtx { world, parent }, key);
                })
            }
        }
    });
    BevyRenderer::set_state(ctx.world, &state_node_id, XResViewState { task });
}

impl<T, F, V> View<BevyRenderer> for XRes<T, F, V>
where
    T: Resource,
    F: Fn(&T) -> V + Clone + Send + Sync + 'static,
    V: View<BevyRenderer>,
{
    type Key = V::Key;

    fn build(
        self,
        ctx: ViewCtx<BevyRenderer>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        let view = (self.f)(ctx.world.resource::<T>());
        let key = view.build(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            },
            reserve_key,
            will_rebuild,
        );
        let Some(state_node_id) = key.state_node_id() else {
            return key;
        };
        x_res_build(self, key.clone(), &state_node_id, ctx);
        key
    }

    fn rebuild(self, ctx: ViewCtx<BevyRenderer>, key: Self::Key) {
        let view = (self.f)(ctx.world.resource::<T>());
        view.rebuild(
            ViewCtx {
                world: &mut *ctx.world,
                parent: ctx.parent.clone(),
            },
            key.clone(),
        );

        let Some(state_node_id) = key.state_node_id() else {
            return;
        };
        drop(BevyRenderer::take_state::<XResViewState>(
            ctx.world,
            &state_node_id,
        ));

        x_res_build(self, key, &state_node_id, ctx);
    }
}

pub fn x_res<T, U>(
    f: impl Fn(&T) -> U + Clone + Send + Sync + 'static,
) -> impl IntoView<BevyRenderer>
where
    T: Resource,
    U: IntoView<BevyRenderer> + Send + 'static,
{
    XRes {
        f: move |r| f(r).into_view(),
        _marker: Default::default(),
    }
}
