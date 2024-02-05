/*

#[derive(Clone, Debug)]
pub struct ViewAttr<A>(pub A::Value)
where
    A: ElementUnitAttr;

impl<EA> ViewMember<BevyRenderer> for ViewAttr<EA>
where
    EA: ElementUnitAttr,
{
    fn count() -> ViewMemberIndex {
        1
    }

    fn unbuild(ctx: ViewMemberCtx<BevyRenderer>, view_removed: bool) {
        if view_removed {
            return;
        }
        ctx.world.unbuild_attr::<EA>(ctx.node_id);
    }

    fn build(self, ctx: ViewMemberCtx<BevyRenderer>, _will_rebuild: bool) {
        ctx.world.build_attr::<EA>(ctx.node_id, self.0);
    }

    fn rebuild(self, ctx: ViewMemberCtx<BevyRenderer>) {
        ctx.world.rebuild_attr::<EA>(ctx.node_id, self.0);
    }
}

impl<EA> ViewAttrMember for ViewAttr<EA>
where
    EA: ElementUnitAttr,
{
    type EA = EA;
}*/

// pub struct AttrValues {
//     attr_values: HashMap<usize, SmallBox<dyn AttrValue, S1>>,
// }
//
// pub struct PropValues {
//     prop_values: HashMap<usize, SmallBox<dyn Any, S1>>,
// }

/*
impl<R, EA> ViewMember<R> for ViewAttr<EA, true>
where
    R: Renderer,
    EA: ElementUnitAttr,
    EA::Value: FromReflect + TypePath,
{
    type State = EA::Value;

    fn build(self, mut ctx: R::MemberCtx<'_>) -> Self::State {
        let entity = ctx.node_id();
        R::set_attr_value::<EA>(ctx.tree(), &entity, self.0.clone());
        self.0
    }

    fn rebuild(self, mut ctx: R::MemberCtx<'_>, state: &mut Self::State) {
        if !self.0.eq(&state) {
            let node_id = ctx.node_id();
            R::set_attr_value::<EA>(ctx.tree(), &node_id, self.0.clone());
        }
        *state = self.0;
    }
}*/
