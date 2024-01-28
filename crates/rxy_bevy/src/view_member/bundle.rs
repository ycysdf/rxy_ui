use bevy_ecs::bundle::Bundle;

use rxy_core::{MemberOwner, ViewMember, ViewMemberCtx};

use crate::BevyRenderer;

pub struct BundleMember<T: Bundle>(pub T);

impl<T> ViewMember<BevyRenderer> for BundleMember<T>
where
    T: Bundle,
{
    fn count() -> u8 {
        1
    }

    fn unbuild(ctx: ViewMemberCtx<BevyRenderer>) {
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

pub trait MemberOwnerBundleExt: MemberOwner<BevyRenderer> {
    #[inline(always)]
    fn bundle<T: Bundle>(self, bundle: T) -> Self::AddMember<BundleMember<T>>
    where
        Self: Sized,
    {
        self.member(BundleMember(bundle))
    }
}

impl<T> MemberOwnerBundleExt for T where T: MemberOwner<BevyRenderer> {}
