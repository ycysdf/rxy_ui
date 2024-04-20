use bevy_ecs::entity::{Entity, EntityHashMap};
use bevy_ecs::prelude::{Or, QueryState, With, World};
use kurbo::{Insets, Point, Rect, RoundedRect, RoundedRectRadii, Stroke, Vec2};
use vello::glyph::Glyph;
use vello::peniko::{Brush, Fill, Font};
use vello::Scene;

use rxy_core::{Either, EitherExt};

use crate::{GlobalTransform, Style, Text, TextLayoutInfo, UiRect, Val, ViewVisibility};
use crate::draw_text::{SceneExt, TextStyle};
use crate::ui_node::{BackgroundColor, BorderColor, BorderRadius, Node, Outline};


pub trait RoundedRectExt {
   fn sub_inset(&self, value: f64) -> Self;
}

impl RoundedRectExt for RoundedRect {
   fn sub_inset(&self, value: f64) -> Self {
      let rect = self.rect();
      let radii = self.radii();
      RoundedRect::from_rect(
         rect - Insets::uniform(value),
         RoundedRectRadii {
            top_left: radii.top_left - value,
            top_right: radii.top_right - value,
            bottom_right: radii.bottom_right - value,
            bottom_left: radii.bottom_left - value,
         },
      )
   }
}

pub struct DrawState {
   pub shape_query_state: QueryState<
      (
         Entity,
         Option<&'static BorderRadius>,
         Option<&'static Style>,
         &'static Node,
         &'static ViewVisibility,
      ),
      Or<(With<BackgroundColor>, With<BorderColor>, With<Outline>)>,
   >,
   pub bg_query_state: QueryState<(Entity, &'static BackgroundColor, &'static GlobalTransform)>,
   pub border_query_state: QueryState<(
      Entity,
      &'static BorderColor,
      &'static GlobalTransform,
      &'static Style,
   )>,
   pub outline_query_state: QueryState<(
      Entity,
      &'static Outline,
      &'static GlobalTransform,
      &'static Style,
   )>,
   pub text_query_state: QueryState<(
      Entity,
      &'static Text,
      &'static TextLayoutInfo,
      &'static GlobalTransform,
      &'static Style,
   )>,
   pub shape_map: EntityHashMap<(Either<RoundedRect, Rect>, Vec2)>,
}

pub fn resolve_border_val(value: Val, node_size: Vec2, viewport_size: Vec2, ui_scale: f64) -> f64 {
   match value {
      Val::Auto => 0.,
      Val::Px(px) => ui_scale * px as f64,
      Val::Percent(percent) => node_size.x.min(node_size.y) * percent as f64 / 100.,
      Val::Vw(percent) => viewport_size.x * percent as f64 / 100.,
      Val::Vh(percent) => viewport_size.y * percent as f64 / 100.,
      Val::VMin(percent) => viewport_size.x.min(viewport_size.y) * percent as f64 / 100.,
      Val::VMax(percent) => viewport_size.x.max(viewport_size.y) * percent as f64 / 100.,
   }
}

pub fn resolve_border_rect(
   ui_rect: UiRect,
   node_size: Vec2,
   viewport_size: Vec2,
   ui_scale: f64,
) -> (f64, f64, f64, f64) {
   (
      resolve_border_val(ui_rect.left, node_size, viewport_size, ui_scale),
      resolve_border_val(ui_rect.right, node_size, viewport_size, ui_scale),
      resolve_border_val(ui_rect.top, node_size, viewport_size, ui_scale),
      resolve_border_val(ui_rect.bottom, node_size, viewport_size, ui_scale),
   )
}

impl DrawState {
   pub fn draw_scene(&mut self, world: &mut World, scene: &mut Scene) {
      let viewport = Vec2::default();
      let ui_scale = 1.;
      self.shape_query_state.iter_mut(world).for_each(
         |(entity, border_radius, style, node, view_visibility)| {
            let calculated_size = node.calculated_size;
            let calculated_size = Vec2::new(calculated_size.x as _, calculated_size.y as _);
            let shape = if let Some(border_radius) = border_radius {
               let radii = border_radius.resolve(calculated_size, viewport, ui_scale);
               RoundedRect::from_rect(
                  Rect::from_points(Point::ZERO, calculated_size.to_point()),
                  radii,
               )
               .either_left()
            } else {
               Rect::from_points(Point::ZERO, calculated_size.to_point()).either_right()
            };
            self.shape_map.insert(entity, (shape, calculated_size));
         },
      );

      self
         .bg_query_state
         .iter_mut(world)
         .for_each(|(entity, bg_color, global_transform)| {
            let Some((shape, _)) = self.shape_map.get(&entity) else {
               return;
            };
            scene.fill(
               Fill::NonZero,
               global_transform.into(),
               &Brush::Solid(bg_color.0),
               None,
               shape,
            );
         });

      self.border_query_state.iter_mut(world).for_each(
         |(entity, border_color, global_transform, style)| {
            let Some((shape, calculated_size)) = self.shape_map.get(&entity) else {
               return;
            };
            let border_rect =
               resolve_border_rect(style.border, *calculated_size, viewport, ui_scale);

            let width = border_rect.0;
            let width_half = width / 2.;
            let insets = Insets::uniform(width_half);
            let shape = match shape {
               Either::Left(shape) => shape.sub_inset(width_half).either_left(),
               Either::Right(mut shape) => (shape - insets).either_right(),
            };
            scene.stroke(
               &Stroke::new(width),
               global_transform.into(),
               &Brush::Solid(border_color.0),
               None,
               &shape,
            );
         },
      );
      self.outline_query_state.iter_mut(world).for_each(
         |(entity, outline, global_transform, style)| {
            let Some((shape, calculated_size)) = self.shape_map.get(&entity) else {
               return;
            };
            let width = resolve_border_val(outline.width, *calculated_size, viewport, ui_scale);

            let width_half = width / 2.;
            let insets = Insets::uniform(width_half);
            let shape = match shape {
               Either::Left(shape) => shape.sub_inset(-width_half).either_left(),
               Either::Right(mut shape) => (shape + insets).either_right(),
            };
            scene.stroke(
               &Stroke::new(width),
               global_transform.into(),
               &Brush::Solid(outline.color),
               None,
               &shape,
            );
         },
      );

      self.text_query_state.iter_mut(world).for_each(
         |(entity, text, text_layout_info, global_transform, style)| {
            // let Some((shape, calculated_size)) = self.shape_map.get(&entity) else {
            //    return;
            // };
            scene.draw_text(
               text_layout_info
                  .glyphs
                  .iter()
                  .enumerate()
                  .filter_map(|(i, n)| {
                     n.as_ref().map(|a| Glyph {
                        id: a.glyph_id as _,
                        x: a.position.x,
                        y: a.position.y,
                     })
                  }),
               &text.style,
               global_transform.into(),
            );
         },
      );
   }
}
