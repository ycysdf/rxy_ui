use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use glam::{Affine2, Affine3A, DAffine2, DVec2, Quat, Vec2, Vec3};
use kurbo::Affine;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub struct GlobalTransform(pub Affine2);

impl GlobalTransform {
   pub const IDENTITY: Self = Self(Affine2::IDENTITY);
}

impl Default for GlobalTransform {
   fn default() -> Self {
      Self::IDENTITY
   }
}

impl From<Transform> for GlobalTransform {
   fn from(transform: Transform) -> Self {
      Self(transform.0)
   }
}

impl Into<Affine> for GlobalTransform {
   fn into(self) -> Affine {
      (&self).into()
   }
}

impl Into<Affine> for &GlobalTransform {
   fn into(self) -> Affine {
      let x = self.0.to_cols_array();
      Affine::new([x[0] as _, x[1] as _, x[2] as _, x[3] as _, x[4] as _, x[5] as _])
   }
}

#[derive(Component, Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, PartialEq))]
#[cfg_attr(
   all(feature = "reflect", feature = "serialize"),
   reflect(Serialize, Deserialize)
)]
pub struct Transform(pub Affine2);

impl Transform {
   pub const IDENTITY: Self = Transform(Affine2::IDENTITY);
}

impl Default for Transform {
   fn default() -> Self {
      Self::IDENTITY
   }
}

#[derive(Bundle, Clone, Copy, Debug, Default)]
pub struct TransformBundle {
   pub local: Transform,
   pub global: GlobalTransform,
}

impl TransformBundle {
   /// An identity [`TransformBundle`] with no translation, rotation, and a scale of 1 on all axes.
   pub const IDENTITY: Self = TransformBundle {
      local: Transform::IDENTITY,
      global: GlobalTransform::IDENTITY,
   };

   /// Creates a new [`TransformBundle`] from a [`Transform`].
   ///
   /// This initializes [`GlobalTransform`] as identity, to be updated later by the
   /// [`PostUpdate`] schedule.
   #[inline]
   pub const fn from_transform(transform: Transform) -> Self {
      TransformBundle {
         local: transform,
         ..Self::IDENTITY
      }
   }
}

impl From<Transform> for TransformBundle {
   #[inline]
   fn from(transform: Transform) -> Self {
      Self::from_transform(transform)
   }
}
