#![allow(clippy::type_complexity)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub use paste::paste;
// pub use clone_to::*;
pub use count_macro;
pub use either::*;
pub use element_type::*;
pub use element_view::*;
pub use into_view::*;
pub use member_owner::*;
pub use mutable_view::*;
pub use r#impl::*;
pub use r#static::*;
#[cfg(feature = "xy_reactive")]
pub use reactive::*;
pub use rebuild::*;
pub use reflect::*;
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
    pub use crate::{
        add_members, build_configure, into_view, member_builder, provide_context, style_builder,
        view_builder, x_future, x_if, x_if_else, x_iter, x_iter_keyed, x_stream,
        BoxedCloneableDynamicView, BoxedDynamicView, BoxedErasureView, Context,
        DeferredNodeTreeScoped, DynamicView, Either, EitherExt, ElementView, ErasureView,
        IntoDynamicView, IntoElementView, IntoView, IntoViewErasureExt, Keyed,
        MemberOwner, Renderer, RendererElementType, Required, SoloView, Static,
        View, ViewCtx, ViewKey, ViewMember, ViewMemberCtx,
    };
    #[cfg(feature = "hooked_collection")]
    pub use crate::{ListOperator};
    #[cfg(feature = "x_iter_source")]
    pub use crate::{use_list, x_iter_source};
    #[cfg(feature = "xy_reactive")]
    pub use crate::{rx, MemberOwnerRxExt, SignalExt};

    #[cfg(feature = "async-channel")]
    pub use crate::Sender;
}

mod element_view;
#[cfg(feature = "xy_reactive")]
mod reactive;
mod reflect;
mod schema;
mod slot;
// mod styled;
pub mod build_info;
pub mod diff;
mod member_owner;
mod r#static;
