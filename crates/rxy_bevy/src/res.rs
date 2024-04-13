use bevy_ecs::prelude::Resource;

use rxy_core::{x_world, MaybeSend, RendererWorld, XWorld};

use crate::BevyRenderer;

#[inline]
pub fn x_res_once<T, F, RES>(
   f: F,
) -> XWorld<BevyRenderer, impl FnOnce(&mut RendererWorld<BevyRenderer>) -> T + MaybeSend + 'static>
where
   RES: Resource,
   F: FnOnce(&RES) -> T + MaybeSend + 'static,
{
   x_world(|world: &mut RendererWorld<BevyRenderer>| f(world.resource::<RES>()))
}
