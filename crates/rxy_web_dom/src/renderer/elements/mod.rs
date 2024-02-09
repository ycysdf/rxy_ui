use crate::renderer::{body, document, WebRenderer};
use rxy_core::{ElementAttrUntyped, ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld};
use rxy_core::{count_macro,paste};
use crate::renderer::attrs::CommonAttrs;

pub fn spawn_element(
    name: &str,
    parent: Option<&RendererNodeId<WebRenderer>>,
    reserve_node_id: Option<RendererNodeId<WebRenderer>>,
) -> RendererNodeId<WebRenderer> {
    let element =
        reserve_node_id.unwrap_or_else(|| document().create_element(name).unwrap().into());
    if let Some(parent) = parent {
        parent.append_child(&element).unwrap();
    } else {
        body().append_child(&element).unwrap();
    }
    element
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WebRendererElementType<const T: usize>;

// pub type WebRendererDivElementType = WebRendererElementType<0>;

impl CommonAttrs for ElementTypeDiv {}
pub const VIEW_ATTRS: &[&'static dyn ElementAttrUntyped<WebRenderer>] =
    <ElementTypeDiv as CommonAttrs>::ATTRS;


macro_rules! define_html_elements {
    ($($ty:ident)*) => {

        count_macro::count! {
            $(
                paste! {
                    pub type [<ElementType $ty:camel>] = WebRendererElementType<_int_a_>;
                    impl ElementType<WebRenderer> for [<ElementType $ty:camel>]  {
                        const TAG_NAME: &'static str = stringify!($ty);
                        const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped<WebRenderer>]] =
                            &[VIEW_ATTRS];

                        fn get() -> &'static dyn ElementTypeUnTyped<WebRenderer> {
                            &WebRendererElementType::<_int_b_>
                        }

                        #[inline]
                        fn spawn(
                            _world: &mut RendererWorld<WebRenderer>,
                            parent: Option<&RendererNodeId<WebRenderer>>,
                            reserve_node_id: Option<RendererNodeId<WebRenderer>>,
                        ) -> RendererNodeId<WebRenderer> {
                            spawn_element(Self::TAG_NAME, parent, reserve_node_id)
                        }
                    }
                }
            )*
        }
    };
}
define_html_elements! {
    div
    span
    button
    a
    p
    h1
    h2
    h3
    h4
    h5
    h6
    br
    hr
    pre
    blockquote
    ol
    ul
    li
    dl
    dt
    dd
    figure
    figcaption
    main
}