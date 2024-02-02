use crate::{Renderer, RendererNodeId, RendererWorld, ViewMemberCtx};

pub struct BuildInfo {
    // > 0
    pub build_times: usize,
}
pub fn node_build_times_increment<R>(
    world: &mut RendererWorld<R>,
    state_node_id: RendererNodeId<R>,
) -> BuildStatus
where
    R: Renderer,
{
    if let Some(build_info) = R::get_state_mut::<BuildInfo>(world, &state_node_id) {
        build_info.build_times += 1;
        BuildStatus::AlreadyBuild
    } else {
        R::set_state(world, &state_node_id, BuildInfo { build_times: 1 });
        BuildStatus::NoBuild
    }
}

pub fn node_build_status<R>(
    world: &RendererWorld<R>,
    state_node_id: &RendererNodeId<R>,
) -> BuildStatus
where
    R: Renderer,
{
    match R::get_state_ref::<BuildInfo>(world, state_node_id).is_some() {
        true => BuildStatus::AlreadyBuild,
        false => BuildStatus::NoBuild,
    }
}

pub enum BuildStatus {
    AlreadyBuild,
    NoBuild,
}

impl BuildStatus {
    pub fn is_no_build(&self) -> bool {
        match self {
            BuildStatus::AlreadyBuild => false,
            BuildStatus::NoBuild => true,
        }
    }
}

impl<'a, R: Renderer> ViewMemberCtx<'a, R> {
    pub fn build_times_increment(&mut self) -> BuildStatus {
        if let Some(build_info) = self.indexed_view_member_state_mut::<BuildInfo>() {
            build_info.build_times += 1;
            BuildStatus::AlreadyBuild
        } else {
            self.set_indexed_view_member_state(BuildInfo { build_times: 1 });
            BuildStatus::NoBuild
        }
    }

    pub fn build_status(&mut self) -> BuildStatus {
        match self.indexed_view_member_state_mut::<BuildInfo>().is_some() {
            true => BuildStatus::AlreadyBuild,
            false => BuildStatus::NoBuild,
        }
    }
}
