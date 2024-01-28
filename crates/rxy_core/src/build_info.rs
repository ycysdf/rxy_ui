use crate::{Renderer, RendererNodeId, RendererWorld, ViewMemberCtx};

pub struct BuildInfo {
    // > 0
    pub build_times: usize,
}
pub fn build_times_increment<R>(world: &mut RendererWorld<R>, state_node_id: RendererNodeId<R>)
where
    R: Renderer,
{
    if let Some(build_info) = R::get_state_mut::<BuildInfo>(world, &state_node_id) {
        build_info.build_times += 1;
    } else {
        R::set_state(world, &state_node_id, BuildInfo { build_times: 1 });
    }
}

pub fn build_info_is_contained<R>(
    world: &RendererWorld<R>,
    state_node_id: &RendererNodeId<R>,
) -> bool
where
    R: Renderer,
{
    R::get_state_ref::<BuildInfo>(world, state_node_id).is_some()
}

impl<'a, R: Renderer> ViewMemberCtx<'a, R> {
    pub fn build_times_increment(&mut self) {
        if let Some(build_info) = self.indexed_view_member_state_mut::<BuildInfo>() {
            build_info.build_times += 1;
        } else {
            self.set_indexed_view_member_state(BuildInfo { build_times: 1 });
        }
    }

    pub fn build_info_is_contained(&mut self) -> bool {
        self.indexed_view_member_state_mut::<BuildInfo>().is_some()
    }
}
