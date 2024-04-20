#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use kurbo::Affine;
use vello::glyph::Glyph;
use vello::peniko::{Brush, Color, Fill, Font};
use vello::skrifa::FontRef;
use vello::Scene;

pub(crate) fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
   use vello::skrifa::raw::FileRef;
   let file_ref = FileRef::new(font.data.as_ref()).ok()?;
   match file_ref {
      FileRef::Font(font) => Some(font),
      FileRef::Collection(collection) => collection.get(font.index).ok(),
   }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub struct TextStyle {
   pub font_size: f32,
   #[cfg_attr(feature = "reflect", reflect(ignore))]
   pub color: Brush,
   pub brush_alpha: f32,
   pub hint: bool,
   pub line_height: f32,
   #[cfg_attr(feature = "reflect", reflect(ignore))]
   pub font: Option<Font>,
}

impl Default for TextStyle {
   fn default() -> Self {
      Self {
         font_size: 18.,
         color: Brush::Solid(Color::BLACK),
         brush_alpha: 1.,
         hint: false,
         line_height: 1.,
         font: None,
      }
   }
}

pub trait SceneExt {
   fn draw_text(
      &mut self,
      glyphs: impl Iterator<Item = Glyph>,
      style: &TextStyle,
      transform: Affine,
   );
}

impl SceneExt for Scene {
   fn draw_text(
      &mut self,
      glyphs: impl Iterator<Item = Glyph>,
      style: &TextStyle,
      transform: Affine,
   ) {
      let font = style.font.as_ref().unwrap();
      // {
      //    let font_ref = to_font_ref(&font).unwrap();
      //    let axes = font_ref.axes();
      //    let variations: &[(&str, f32)] = &[];
      //    let var_loc = axes.location(variations.iter().copied());
      // }
      self
         .draw_glyphs(&font)
         .font_size(style.font_size)
         .transform(transform)
         // .glyph_transform(glyph_transform)
         // .normalized_coords(var_loc.coords())
         .brush(&style.color)
         .brush_alpha(style.brush_alpha)
         .hint(style.hint)
         .draw(Fill::NonZero, glyphs);
   }
}
