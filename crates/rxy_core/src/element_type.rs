use crate::{MaybeReflect, MaybeSend, Renderer, RendererNodeId, RendererWorld};

pub trait RendererElementType<R>: MaybeReflect + MaybeSend + 'static
where
    R: Renderer,
{
    const NAME: &'static str;
    fn spawn(
        world: &mut RendererWorld<R>,
        parent: Option<RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R>;
}
