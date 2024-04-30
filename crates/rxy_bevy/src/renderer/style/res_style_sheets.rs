use bevy_ecs::system::Resource;
use bevy_ecs::world::FromWorld;
use rxy_core::Renderer;

use rxy_core::style::{AppliedStyleSheet, StyleSheetCtx, StyleSheets, StyleSheetsInfo};

use crate::XRes;

impl<F, Res, T, R> StyleSheets<R> for XRes<Res, F>
where
   R: Renderer<NodeTree = bevy_ecs::world::World>,
   F: Fn(&Res) -> T + Send + 'static,
   Res: Resource + FromWorld,
   T: StyleSheets<R>,
{
   fn style_sheets(
      self,
      ctx: StyleSheetCtx<R>,
   ) -> (
      impl Iterator<Item = AppliedStyleSheet<R>> + Send + 'static,
      StyleSheetsInfo,
   ) {
      if !ctx.world.contains_resource::<Res>() {
         let res = Res::from_world(ctx.world);
         ctx.world.insert_resource(res);
      }
      let res = ctx.world.resource::<Res>();
      (self.f)(res).style_sheets(ctx)
   }
}
