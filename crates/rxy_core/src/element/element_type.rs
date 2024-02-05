use crate::element::{AttrIndex, ElementAttrUntyped};
use crate::{MaybeReflect, MaybeSend, MaybeSync, Renderer, RendererNodeId, RendererWorld};

pub trait ElementTypeUnTyped<R>: MaybeReflect + MaybeSend + MaybeSync
where
    R: Renderer,
{
    fn tag_name(&self) -> &'static str;

    fn attrs(&self) -> &'static [&'static [&'static dyn ElementAttrUntyped<R>]];

    // fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementUnitAttrUntyped<R>>;
    fn attr_by_index(&self, index: AttrIndex) -> &'static dyn ElementAttrUntyped<R> {
        let mut index = index as usize;
        for attrs in self.attrs() {
            if index < attrs.len() {
                return attrs[index];
            }
            index -= attrs.len();
        }
        unreachable!();
    }
    fn spawn(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R>;
}

impl<R, T: ElementType<R>> ElementTypeUnTyped<R> for T
where
    R: Renderer,
{
    #[inline]
    fn tag_name(&self) -> &'static str {
        T::TAG_NAME
    }

    #[inline]
    fn attrs(&self) -> &'static [&'static [&'static dyn ElementAttrUntyped<R>]] {
        T::ATTRS
    }

    // #[inline]
    // fn attr(&self, attr_name: &str) -> Option<&'static dyn ElementUnitAttrUntyped<R>> {
    //     T::attr(attr_name)
    // }

    #[inline]
    fn spawn(
        &self,
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R> {
        T::spawn(world, parent, reserve_node_id)
    }
}

pub trait ElementType<R>: MaybeReflect + MaybeSend + MaybeSync + 'static
where
    R: Renderer,
{
    const TAG_NAME: &'static str;
    const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped<R>]];

    // fn attr(attr_name: &str) -> Option<&'static dyn ElementUnitAttrUntyped<R>> {
    //     use rxy_core::utils::HashMap;
    //
    //     static ATTRS: core::cell::OnceCell<
    //         HashMap<&'static str, &'static dyn ElementUnitAttrUntyped<R>>,
    //     > = core::cell::OnceCell::new();
    //
    //     // static ATTRS: core::sync::OnceLock<
    //     //     HashMap<&'static str, &'static dyn ElementUnitAttrUntyped<R>>,
    //     // > = core::sync::OnceLock::new();
    //     let map = ATTRS.get_or_init(|| {
    //         let mut map: HashMap<&'static str, &'static dyn ElementUnitAttrUntyped<R>> =
    //             HashMap::new();
    //         for attrs in Self::ATTRS {
    //             for attr in *attrs {
    //                 map.insert(attr.attr_name(), *attr);
    //             }
    //         }
    //         map
    //     });
    //     map.get(attr_name).copied()
    // }
    fn spawn(
        world: &mut RendererWorld<R>,
        parent: Option<&RendererNodeId<R>>,
        reserve_node_id: Option<RendererNodeId<R>>,
    ) -> RendererNodeId<R>;
}
