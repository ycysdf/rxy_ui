mod attr_value;
mod element;
mod element_attr;
mod element_attr_member;
mod element_type;
pub mod into_attr;
// mod test;
mod view_member;
mod element_children;

pub use attr_value::*;
pub use element_attr::*;
pub use element_attr_member::*;
pub use element_type::*;
pub use view_member::*;
pub use element_children::*;
pub use element::*;


/*
impl<R> ElementType<R> for R::Wrapper<test_element_type>
where
    R: Renderer,
{
    const TAG_NAME: &'static str = "";
    const ATTRS: &'static [&'static [&'static dyn ElementUnitAttrUntyped<R>]] = &[];

    fn spawn(
        world: &mut RendererWorld<R>,
        parent: Option<RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R> {
        todo!()
    }
}

pub struct test_element_type;

impl<R> RendererElementType<R> for test_element_type where R: Renderer {
    const NAME: &'static str = "";

    fn spawn(world: &mut RendererWorld<R>, parent: Option<RendererNodeId<R>>, reserve_node_id: Option<RendererNodeId<R>>) -> RendererNodeId<R> {
        todo!()
    }
}*/

// mod into_attr_value;
