// use crate::{ElementView, Renderer, ViewMember, ViewMemberCtx};
// use core::marker::PhantomData;
// use xy_reactive::with;
//
// pub struct Styled<R, F, P, T>(F, PhantomData<(R, T, P)>)
// where
//     R: Renderer,
//     F: FnOnce(P) -> T + Send + 'static,
//     T: ViewMember<R>,
//     P: Send + 'static;
//
// // fn member<T>(self, member: T) -> crate::element_view::ElementView::AddMember<T>
// //     where
// //         (crate::element_view::ElementView::VM, T): ViewMember<R>, T: ViewMember<R>;
// pub trait StyledExt<R>: ElementView<R>
// where
//     R: Renderer,
// {
//     fn styled<F, P, T>(
//         self,
//         style: Styled<R, F, P, T>,
//         p: P,
//     ) -> Self::AddMember<UseStyled<R, F, P, T>>
//     where
//         Self: Sized,
//         (Self::VM, UseStyled<R, F, P, T>): ViewMember<R>,
//         F: FnOnce(P) -> T + Send + 'static,
//         T: ViewMember<R>,
//         P: Send + 'static,
//     {
//         self.member(UseStyled(style, p))
//     }
// }
//
// pub struct UseStyled<R, F, P, T>(Styled<R, F, P, T>, P)
// where
//     R: Renderer,
//     F: FnOnce(P) -> T + Send + 'static,
//     T: ViewMember<R>,
//     P: Send + 'static;
//
// impl<R, F, P, T> ViewMember<R> for UseStyled<R, F, P, T>
// where
//     R: Renderer,
//     F: FnOnce(P) -> T + Send + 'static,
//     T: ViewMember<R>,
//     P: Send + 'static,
// {
//     fn unbuild(ctx: ViewMemberCtx<R>) {
//         T::unbuild(ctx)
//     }
//
//     fn build(self, ctx: ViewMemberCtx<R>, will_rebuild: bool) {
//         self.0 .0(self.1).build(ctx, will_rebuild)
//     }
//
//     fn rebuild(self, ctx: ViewMemberCtx<R>) {
//         self.0 .0(self.1).rebuild(ctx)
//     }
// }
