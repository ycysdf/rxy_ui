#![allow(clippy::type_complexity)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// pub use clone_to::*;
pub use count_macro;
pub use paste::paste;

pub use build_info::ViewMemberBuildExt;
pub use either::*;
pub use element::*;
pub use element_view::*;
pub use into_view::*;
pub use maybe_traits::*;
pub use member_owner::*;
pub use mutable_view::*;
pub use nest::*;
pub use r#impl::*;
pub use r#static::*;
pub use rebuild::*;
pub use renderer::*;
pub use schema::*;
pub use smallbox::*;
pub use view::*;
pub use view_member::*;

mod either;
mod r#impl;
mod into_view;
mod mutable_view;
mod nest;
mod rebuild;
mod renderer;
mod smallbox;
mod view;
mod view_member;

// pub use nest::*;

#[cfg(test)]
pub mod test;

pub mod prelude {
   #[cfg(feature = "async-channel")]
   pub use async_channel::Sender;

   #[cfg(feature = "style")]
   pub use crate::style::prelude::*;
   #[cfg(feature = "hooked_collection")]
   pub use crate::ListOperator;
   pub use crate::OnBuildExt;
   pub use crate::{
      build_configure, fn_schema_view, into_view, member_builder, provide_context, style_builder,
      view_builder, x_future, x_if, x_if_else, x_iter, x_iter_keyed, x_stream,
      BoxedCloneableDynamicView, BoxedDynamicView, BoxedErasureView, Context,
      DeferredNodeTreeScoped, DynamicView, Either, EitherExt, ElementView, ErasureView,
      IntoDynamicView, IntoElementView, IntoView, IntoViewErasureExt, Keyed, MemberOwner, Renderer,
      Required, SchemaIntoViewFn, SoloView, Static, View, ViewCtx, ViewKey, ViewMember,
      ViewMemberCtx,
   };
   #[cfg(feature = "xy_reactive")]
   pub use crate::{rx, ElementViewRxExt, MemberOwnerRxExt};
   #[cfg(feature = "x_iter_source")]
   pub use crate::{use_list, x_iter_source};
   pub use crate::{ElementAttrType, ElementAttrUntyped, ElementType, ElementTypeUnTyped};
   pub use crate::{SchemaElementView, SchemaView};

   pub use super::member_after_children::MemberAfterChildrenExt;
}

mod element_view;
mod maybe_traits;
mod schema;
// mod styled;
pub mod build_info;

#[cfg(feature = "common_renderer")]
pub mod common_renderer;
pub mod diff;
mod element;
pub mod member_after_children;
mod member_owner;
pub mod remove_on_drop;
mod renderers;
mod r#static;
#[cfg(feature = "style")]
pub mod style;
pub mod utils;
