use core::fmt;
use rxy_macro::IntoView;

use crate::{BoxedCloneableErasureView, BoxedErasureView, ErasureViewKey, Renderer, View, ViewCtx};

#[derive(IntoView)]
pub struct SchemaSlot<R>
where
    R: Renderer,
{
    view: Option<BoxedErasureView<R>>,
}

impl<R> fmt::Debug for SchemaSlot<R>
where
    R: Renderer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SchemaSlot").finish()
    }
}

impl<R> SchemaSlot<R>
where
    R: Renderer,
{
    pub fn new(view: Option<BoxedErasureView<R>>) -> Self {
        Self { view }
    }
}

impl<R> View<R> for SchemaSlot<R>
where
    R: Renderer,
{
    type Key = Option<ErasureViewKey<R>>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        let view = self.view?;

        let key = view.build(
            ctx,
            reserve_key.map(|key| key.expect("reserve_key must not be None")),
            false,
        );

        Some(key)
    }

    fn rebuild(self, _ctx: ViewCtx<R>, _key: Self::Key) {}
}

#[derive(IntoView, Clone)]
pub struct CloneableSchemaSlot<R>
where
    R: Renderer,
{
    view: Option<BoxedCloneableErasureView<R>>,
}

impl<R> fmt::Debug for CloneableSchemaSlot<R>
where
    R: Renderer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloneableSchemaSlot").finish()
    }
}

impl<R> CloneableSchemaSlot<R>
where
    R: Renderer,
{
    pub fn new(view: Option<BoxedCloneableErasureView<R>>) -> Self {
        Self { view }
    }
}

impl<R> View<R> for CloneableSchemaSlot<R>
where
    R: Renderer,
{
    type Key = Option<ErasureViewKey<R>>;

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        _will_rebuild: bool,
    ) -> Self::Key {
        let view = self.view?;

        let key = view.build(
            ctx,
            reserve_key.map(|key| key.expect("reserve_key must not be None")),
            false,
        );

        Some(key)
    }

    fn rebuild(self, _ctx: ViewCtx<R>, _key: Self::Key) {}
}
