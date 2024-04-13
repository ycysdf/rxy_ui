use core::marker::PhantomData;

use bevy_ecs::system::Resource;
use bevy_ecs::world::FromWorld;

use rxy_core::style::{AppliedStyleSheet, StyleSheetCtx, StyleSheets, StyleSheetsInfo};
use rxy_core::XValueWrapper;

use super::rxy_bevy_crate::BevyRenderer;

pub struct XRes<T, M>(T, PhantomData<M>);

pub fn res<F, Res, T>(f: F) -> XRes<F, Res>
where
   F: Fn(&Res) -> T + Send + 'static,
   Res: Resource,
   T: StyleSheets<BevyRenderer>,
{
   XRes(f, Default::default())
}

impl<F, Res> Into<XValueWrapper<Self>> for XRes<F, Res> {
   fn into(self) -> XValueWrapper<Self> {
      XValueWrapper(self)
   }
}

impl<F, Res, T> StyleSheets<BevyRenderer> for XRes<F, Res>
where
   F: Fn(&Res) -> T + Send + 'static,
   Res: Resource + FromWorld,
   T: StyleSheets<BevyRenderer>,
{
   fn style_sheets(
      self,
      ctx: StyleSheetCtx<BevyRenderer>,
   ) -> (
      impl Iterator<Item = AppliedStyleSheet<BevyRenderer>> + Send + 'static,
      StyleSheetsInfo,
   ) {
      let f = self.0;
      if !ctx.world.contains_resource::<Res>() {
         let res = Res::from_world(ctx.world);
         ctx.world.insert_resource(res);
      }
      let res = ctx.world.resource::<Res>();
      f(res).style_sheets(ctx)
   }
}
