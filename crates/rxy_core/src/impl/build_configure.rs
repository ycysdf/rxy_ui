use crate::{IntoView, Renderer, View, ViewCtx};

pub fn build_configure<T>(t: T, config: BuildConfig) -> BuildConfigure<T> {
   BuildConfigure(t, config)
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Copy)]
pub enum WillRebuildOverride {
   Nop,
   Value(bool),
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
   pub override_will_rebuild: WillRebuildOverride,
}

#[derive(Debug, Clone)]
pub struct BuildConfigure<T>(T, BuildConfig);

impl<R: Renderer, V: View<R>> View<R> for BuildConfigure<V> {
   type Key = V::Key;

   fn build(
      self,
      ctx: ViewCtx<R>,
      reserve_key: Option<Self::Key>,
      will_rebuild: bool,
   ) -> Self::Key {
      self.0.build(
         ctx,
         reserve_key,
         match self.1.override_will_rebuild {
            WillRebuildOverride::Nop => will_rebuild,
            WillRebuildOverride::Value(v) => v,
         },
      )
   }

   fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
      self.0.rebuild(ctx, key)
   }
}

impl<R, IV> IntoView<R> for BuildConfigure<IV>
where
   R: Renderer,
   IV: IntoView<R>,
{
   type View = IV::View;

   fn into_view(self) -> Self::View {
      self.0.into_view()
   }
}
