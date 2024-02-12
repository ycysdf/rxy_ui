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
use crate::attrs::node_value;

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

macro_rules! define_html_elements {
    ($($ty:ident)*) => {
        count_macro::count! {
            $(
                paste! {

                    pub struct [<element_ $ty:snake>];
                    impl ElementType<WebRenderer> for [<element_ $ty:snake>]  {
                        const TAG_NAME: &'static str = stringify!($ty);

                        fn get() -> &'static dyn ElementTypeUnTyped<WebRenderer> {
                            &Self
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
    img
    input
    textarea
    select
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
                pub fn $ty() -> crate::WebElement<[<element_ $ty:snake>], ()> {
                    crate::WebElement::default()
                }
            }
        )*
    };
}

pub type WebElementWithContent<E, VM> =
    ElementViewChildren<WebRenderer, WebElement<E, ()>, WebElement<element_text, (VM,)>>;

macro_rules! define_view_fns_with_content {
    ($($ty:ident)*) => {
        $(
            paste! {
                #[inline]
                pub fn $ty<VM>(
                    str: impl XNest<MapInner<MapToAttrMarker<node_value>> = VM>,
                ) -> WebElementWithContent<[<element_ $ty:snake>],VM>
                where
                    VM: ElementAttrMember<WebRenderer, node_value>,
                {
                    view_children(
                        WebElement::default(),
                        WebElement::<element_text,()>::default().members(str.map_inner::<MapToAttrMarker<node_value>>())
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
    img
    input
    textarea
    select
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

pub struct element_text;

impl ElementType<WebRenderer> for element_text {
    const TAG_NAME: &'static str = stringify!(text);

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
