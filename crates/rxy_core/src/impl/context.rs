use core::marker::PhantomData;

use crate::{
   ElementView, IntoView, MaybeSend, MaybeSync, MemberOwner, NodeTree, Renderer, RendererNodeId,
   SoloView, View, ViewCtx, ViewKey, ViewMember, ViewMemberIndex,
};

pub struct ProvideContext<R, T, V> {
   provide_context: T,
   view: V,
   _marker: PhantomData<R>,
}

pub fn provide_context<R, T, IV>(provide_context: T, view: IV) -> ProvideContext<R, T, IV::View>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
   IV: IntoView<R>,
   IV::View: SoloView<R>,
{
   ProvideContext {
      provide_context,
      view: view.into_view(),
      _marker: Default::default(),
   }
}

impl<R> ViewCtx<'_, R>
where
   R: Renderer,
{
   pub fn context<T: MaybeSend + MaybeSync + Clone + 'static>(&self) -> T {
      self
         .get_context()
         .unwrap_or_else(|| panic!("Tried to access a context that has not been provided."))
   }

   pub fn context_ref<T: MaybeSend + MaybeSync + 'static>(&self) -> &T {
      self
         .get_context_ref()
         .unwrap_or_else(|| panic!("Tried to access a context that has not been provided."))
   }

   pub fn get_context<T: MaybeSend + MaybeSync + Clone + 'static>(&self) -> Option<T> {
      self.get_context_ref().cloned()
   }

   pub fn get_context_ref<T: MaybeSend + MaybeSync + 'static>(&self) -> Option<&T> {
      let mut current_parent = self.parent.clone();
      loop {
         if let Some(context) = self.world.get_node_state_ref::<Context<T>>(&current_parent) {
            return Some(&context.0);
         }
         if let Some(parent) = self.world.get_parent(&current_parent) {
            current_parent = parent;
         } else {
            return None;
         }
      }
   }
   pub fn context_scoped<T: MaybeSend + MaybeSync + 'static>(
      &mut self,
      f: impl FnOnce(&mut T),
   ) -> bool {
      let mut current_parent = self.parent.clone();
      loop {
         if let Some(mut context) = self.world.take_node_state::<Context<T>>(&current_parent) {
            f(&mut context.0);
            self.world.set_node_state(&current_parent, context);
            return true;
         }
         if let Some(parent) = self.world.get_parent(&current_parent) {
            current_parent = parent;
         } else {
            return false;
         }
      }
   }
}

#[derive(Clone, Debug)]
pub struct Context<T>(pub T);

impl<R, T, V> View<R> for ProvideContext<R, T, V>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
   V: ElementView<R>,
{
   type Key = V::Key;

   fn build(
      self,
      ctx: ViewCtx<R>,
      reserve_key: Option<Self::Key>,
      will_rebuild: bool,
   ) -> Self::Key {
      let reserve_key = reserve_key
         .unwrap_or_else(|| V::Key::reserve_key(ctx.world, will_rebuild, ctx.parent.clone(), true));
      let node_id = V::element_node_id(&reserve_key);
      ctx.world
         .set_node_state::<Context<T>>(node_id, Context(self.provide_context));

      self.view.build(
         ViewCtx {
            world: &mut *ctx.world,
            parent: ctx.parent,
         },
         Some(reserve_key),
         will_rebuild,
      )
   }

   fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
      let node_id = V::element_node_id(&key);
      ctx.world
         .set_node_state::<Context<T>>(node_id, Context(self.provide_context));
      self.view.rebuild(ctx, key);
   }
}

impl<R, T, V> IntoView<R> for ProvideContext<R, T, V>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
   V: ElementView<R>,
{
   type View = ProvideContext<R, T, V>;
   fn into_view(self) -> Self::View {
      self
   }
}

impl<R, T, V> SoloView<R> for ProvideContext<R, T, V>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
   V: ElementView<R>,
{
   fn node_id(key: &Self::Key) -> &RendererNodeId<R> {
      V::element_node_id(key)
   }
}

impl<R, T, V> ElementView<R> for ProvideContext<R, T, V>
where
   R: Renderer,
   T: MaybeSend + MaybeSync + 'static,
   V: ElementView<R>,
{
   fn element_node_id(key: &Self::Key) -> &RendererNodeId<R> {
      V::element_node_id(key)
   }

   type E = V::E;
   type AddMember<VM: ViewMember<R>> = ProvideContext<R, T, V::AddMember<VM>>;
   type SetMembers<VM: ViewMember<R> + MemberOwner<R>> = ProvideContext<R, T, V::SetMembers<VM>>;

   fn member_count(&self) -> ViewMemberIndex {
      self.view.member_count()
   }

   fn member<VM>(self, member: VM) -> Self::AddMember<VM>
   where
      VM: ViewMember<R>,
   {
      ProvideContext {
         provide_context: self.provide_context,
         view: self.view.member(member),
         _marker: Default::default(),
      }
   }

   fn members<VM: ViewMember<R>>(self, members: VM) -> Self::SetMembers<(VM,)>
   where
      VM: ViewMember<R>,
   {
      ProvideContext {
         provide_context: self.provide_context,
         view: self.view.members(members),
         _marker: Default::default(),
      }
   }
}
