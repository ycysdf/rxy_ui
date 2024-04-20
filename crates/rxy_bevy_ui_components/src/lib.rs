use bevy_ecs::prelude::Resource;
use bevy_reflect::Reflect;
use bevy_render::prelude::Color;
use rxy_ui::bevy::{x_res_once, BevyRenderer};
use rxy_ui::prelude::ElementEventId;
use rxy_ui::{x_world, MaybeReflect, MaybeSend, RendererWorld, XWorld};
use std::sync::Arc;

mod checkbox;
// mod select;
// mod slider;

#[derive(Resource, Reflect)]
pub struct UiGlobalSetting {
   confirm_event_ids: Arc<Vec<ElementEventId>>,
}

#[derive(Resource, Reflect)]
pub struct UiThemeSetting {
   primary_color: Color,
}

macro_rules! define_x_res_fn {
   ($ident:ident,$ty:ty) => {
      #[inline]
      pub fn $ident<T, F>(
         f: F,
      ) -> XWorld<
         BevyRenderer,
         impl FnOnce(&mut RendererWorld<BevyRenderer>) -> T + MaybeSend + 'static,
      >
      where
         F: FnOnce(&$ty) -> T + MaybeSend + 'static,
      {
         x_res_once(|n: &$ty| f(n))
      }
   };
}

define_x_res_fn!(x_theme_once, UiThemeSetting);
define_x_res_fn!(x_ui_setting_once, UiGlobalSetting);
