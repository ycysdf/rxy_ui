use crate::attrs::ElementAttrNodeValue;
use crate::renderer::attrs::CommonAttrs;
use crate::renderer::{body, document, WebRenderer};
use crate::{span, WebElement};
use rxy_core::MapToAttrMarker;
use rxy_core::MemberOwner;
use rxy_core::{count_macro, paste, view_children, ElementAttrMember, ElementViewChildren, XNest};
use rxy_core::{
    ElementAttrUntyped, ElementType, ElementTypeUnTyped, RendererNodeId, RendererWorld,
};
use wasm_bindgen::intern;
use web_sys::wasm_bindgen::JsValue;
use web_sys::Node;

pub fn replace_placeholder(placeholder: &Node, new_node: &Node) -> Result<(), JsValue> {
    placeholder
        .parent_node()
        .unwrap()
        .replace_child(new_node, placeholder)?;
    Ok(())
}

pub fn spawn_element(
    name: &str,
    parent: Option<&RendererNodeId<WebRenderer>>,
    reserve_node_id: Option<RendererNodeId<WebRenderer>>,
) -> RendererNodeId<WebRenderer> {
    let element = document().create_element(name).unwrap();
    if let Some(reserve_node_id) = reserve_node_id {
        replace_placeholder(&reserve_node_id, &element).unwrap();
    }
    if let Some(parent) = parent {
        parent.append_child(&element).unwrap();
    } else {
        body().append_child(&element).unwrap();
    }
    element.into()
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WebRendererElementType<const T: usize>;

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
                            spawn_element(intern(Self::TAG_NAME), parent, reserve_node_id)
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

macro_rules! define_view_fns {
    ($($ty:ident)*) => {
        $(
            paste! {
                #[inline]
                pub fn $ty() -> crate::WebElement<[<ElementType $ty:camel>], ()> {
                    crate::WebElement::default()
                }
            }
        )*
    };
}

pub type WebElementWithContent<E, VM> =
    ElementViewChildren<WebRenderer, WebElement<E, ()>, WebElement<NodeTypeText, (VM,)>>;
macro_rules! define_view_fns_with_content {
    ($($ty:ident)*) => {
        $(
            paste! {
                #[inline]
                pub fn $ty<VM>(
                    str: impl XNest<MapInner<MapToAttrMarker<ElementAttrNodeValue>> = VM>,
                ) -> WebElementWithContent<[<ElementType $ty:camel>],VM>
                where
                    VM: ElementAttrMember<WebRenderer, ElementAttrNodeValue>,
                {
                    view_children(
                        WebElement::default(),
                        WebElement::<NodeTypeText,()>::default().members(str.map_inner::<MapToAttrMarker<ElementAttrNodeValue>>())
                    )
                }
            }
        )*
    };
}

define_view_fns_with_content! {
    a
    p
    h1
    h2
    h3
    h4
    h5
    h6
}

define_view_fns! {
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

pub struct NodeTypeText;

impl ElementType<WebRenderer> for NodeTypeText {
    const TAG_NAME: &'static str = stringify!(text);
    const ATTRS: &'static [&'static [&'static dyn ElementAttrUntyped<WebRenderer>]] = &[];

    fn get() -> &'static dyn ElementTypeUnTyped<WebRenderer> {
        &Self
    }

    #[inline]
    fn spawn(
        _world: &mut RendererWorld<WebRenderer>,
        parent: Option<&RendererNodeId<WebRenderer>>,
        reserve_node_id: Option<RendererNodeId<WebRenderer>>,
    ) -> RendererNodeId<WebRenderer> {
        let element = document().create_text_node("");
        if let Some(reserve_node_id) = reserve_node_id {
            replace_placeholder(&reserve_node_id, &element).unwrap();
        }
        if let Some(parent) = parent {
            parent.append_child(&element).unwrap();
        } else {
            body().append_child(&element).unwrap();
        }
        element.into()
    }
}
