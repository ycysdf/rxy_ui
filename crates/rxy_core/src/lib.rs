#![allow(clippy::type_complexity)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// pub use clone_to::*;
pub use count_macro;
pub use paste::paste;

pub use either::*;
pub use element_type::*;
pub use element_view::*;
pub use into_view::*;
pub use maybe_traits::*;
pub use member_owner::*;
pub use mutable_view::*;
pub use r#impl::*;
pub use r#static::*;
#[cfg(feature = "xy_reactive")]
pub use reactive::*;
pub use rebuild::*;
pub use renderer::*;
pub use schema::*;
pub use slot::*;
pub use view::*;
pub use view_member::*;

mod either;
mod element_type;
mod r#impl;
mod into_view;
mod mutable_view;
mod rebuild;
mod renderer;
mod view;
mod view_member;

pub mod prelude {
    #[cfg(feature = "async-channel")]
    pub use async_channel::Sender;

    pub use crate::{
        add_members, BoxedCloneableDynamicView, BoxedDynamicView, BoxedErasureView, build_configure, Context,
        DeferredNodeTreeScoped, DynamicView, Either, EitherExt, ElementView, ErasureView, fn_schema_view,
        into_view, IntoDynamicView, IntoElementView, IntoView,
        IntoViewErasureExt, Keyed, member_builder, MemberOwner, provide_context, Renderer,
        RendererElementType, Required, SchemaIntoViewFn, SoloView, Static,
        style_builder, View, view_builder, ViewCtx, ViewKey, ViewMember,
        ViewMemberCtx, x_future, x_if, x_if_else, x_iter, x_iter_keyed, x_stream
    };
    #[cfg(feature = "x_iter_source")]
    pub use crate::{use_list, x_iter_source};
    #[cfg(feature = "xy_reactive")]
    pub use crate::{MemberOwnerRxExt, rx, SignalExt};
    #[cfg(feature = "hooked_collection")]
    pub use crate::ListOperator;
}

mod element_view;
#[cfg(feature = "xy_reactive")]
mod reactive;
mod maybe_traits;
mod schema;
mod slot;
// mod styled;
pub mod build_info;
pub mod diff;
mod member_owner;
mod r#static;
