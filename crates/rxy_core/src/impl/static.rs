/*use crate::{IntoView, Renderer, View, ViewCtx};

pub fn static_fn<T>(f: fn() -> T) -> Static<T> {
    Static(f)
}

pub struct Static<T>(fn() -> T);

impl<R, IV> View<R> for Static<IV>
where
    R: Renderer,
    IV: IntoView<R>,
{
    type Key = <IV::View as View<R>>::Key;
    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        self.0().into_view().build(ctx, reserve_key, false)
    }
    fn rebuild(self, _ctx: ViewCtx<R>, _key: Self::Key) {}
}

impl<R, IV> IntoView<R> for Static<IV>
where
    R: Renderer,
    IV: IntoView<R>,
{
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}
*/
