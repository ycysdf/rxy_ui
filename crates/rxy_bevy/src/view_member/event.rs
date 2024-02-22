use std::marker::PhantomData;

use bevy_ecs::prelude::IntoSystem;
use bevy_ecs::system::SystemId;
use bevy_utils::tracing::error;

use rxy_core::{ViewMember, ViewMemberCtx, ViewMemberOrigin};

use crate::event::*;
use crate::BevyRenderer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FocusInputEventMemberState<T>(pub SystemId, pub T);

pub struct EventViewMember<T, S, M> {
    pub element_event_ids: T,
    pub system: S,
    pub _marker: PhantomData<M>,
}

impl<T, S, M> ViewMemberOrigin<BevyRenderer> for EventViewMember<T, S, M>
where
    T: ElementEventIds,
    S: IntoSystem<(), (), M> + Send + 'static,
    M: Send + 'static,
{
    type Origin = Self;
}

impl<T, S, M> ViewMember<BevyRenderer> for EventViewMember<T, S, M>
where
    T: ElementEventIds,
    S: IntoSystem<(), (), M> + Send + 'static,
    M: Send + 'static,
{
    fn count() -> rxy_core::ViewMemberIndex {
        1
    }

    fn unbuild(mut ctx: ViewMemberCtx<BevyRenderer>, _view_removed: bool) {
        let state = ctx
            .take_indexed_view_member_state::<FocusInputEventMemberState<T>>()
            .unwrap();
        for event_id in state.1.iter_event_ids() {
            ctx.world.remove_event(ctx.node_id, event_id, state.0);
        }
        if let Err(err) = ctx.world.remove_system(state.0) {
            error!("remove_system error: {:?}", err);
        }
    }

    fn build(self, mut ctx: ViewMemberCtx<BevyRenderer>, _will_rebuild: bool) {
        let system_id = ctx.world.register_system(self.system);

        for event_id in self.element_event_ids.clone().iter_event_ids() {
            ctx.world.add_event(ctx.node_id, event_id, system_id);
        }

        ctx.set_indexed_view_member_state(FocusInputEventMemberState(
            system_id,
            self.element_event_ids,
        ));
    }

    fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
        Self::unbuild(
            ViewMemberCtx {
                index: ctx.index,
                world: &mut *ctx.world,
                node_id: ctx.node_id,
            },
            false,
        );
        self.build(ctx, true);
    }
}

