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
pub use rxy_macro;
pub use schema::*;
pub use slot::*;
pub use view::*;
pub use view_member::*;
pub use view_state::*;

mod either;
mod element_type;
mod r#impl;
mod into_view;
mod mutable_view;
mod rebuild;
mod renderer;
mod view;
mod view_member;
mod view_state;

pub mod prelude {
    pub use crate::{
        add_members, build_configure, into_view, member_builder, provide_context, style_builder,
        view_builder, x_future, x_if, x_if_else, x_iter, x_iter_keyed, x_stream,
        BoxedCloneableDynamicView, BoxedDynamicView, BoxedErasureView, Context,
        DeferredWorldScoped, DynamicView, Either, EitherExt, ElementView, ErasureView,
        IntoDynamicView, IntoElementView, IntoView, IntoViewErasureExt, Keyed, MemberOwner,
        Renderer, RendererElementType, RendererViewExt, Required, SoloView, Static, View,
        ViewCtx, ViewKey, ViewMember, ViewMemberCtx,
    };
    #[cfg(feature = "xy_reactive")]
    pub use crate::{rx, use_list, x_iter_source, MemberOwnerRxExt};
    
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
