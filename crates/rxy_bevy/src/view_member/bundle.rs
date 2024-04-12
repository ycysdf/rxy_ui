use bevy_ecs::bundle::Bundle;

use rxy_core::{ViewMember, ViewMemberCtx, ViewMemberIndex, ViewMemberOrigin};

use crate::BevyRenderer;

pub struct XBundle<T: Bundle>(pub T);

pub fn x_bundle<T: Bundle>(bundle: T) -> XBundle<T> {
   XBundle(bundle)
}

impl<T> ViewMemberOrigin<BevyRenderer> for XBundle<T>
where
   T: Bundle,
{
   type Origin = Self;
}

impl<T> ViewMember<BevyRenderer> for XBundle<T>
where
   T: Bundle,
{
   fn count() -> ViewMemberIndex {
      1
   }

   fn unbuild(ctx: ViewMemberCtx<BevyRenderer>, view_removed: bool) {
      if view_removed {
         return;
      }
      let entity = ctx.node_id;
      ctx.world.entity_mut(entity).remove::<T>();
   }

   fn build(self, ctx: ViewMemberCtx<BevyRenderer>, _will_rebuild: bool) {
      let entity = ctx.node_id;
      ctx.world.entity_mut(entity).insert(self.0);
   }

   fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
      self.build(ctx, true);
   }
}
